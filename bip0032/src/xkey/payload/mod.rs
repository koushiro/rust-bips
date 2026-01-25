//! Extended key payload serialization and deserialization.

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

use zeroize::{Zeroize, Zeroizing};

mod version;

pub use self::version::{KnownVersion, Version};
use crate::{
    error::{Error, ErrorKind, Result},
    xkey::core::ExtendedKeyMetadata,
};

/// Base58Check-encoded extended key payload plus version metadata.
pub struct ExtendedKeyPayload {
    /// 4 bytes: the version bytes.
    pub(crate) version: Version,
    /// 41 bytes: the metadata for extended key (depth, parent link, and chain code).
    pub(crate) meta: ExtendedKeyMetadata,
    /// 33 bytes: the extended public/private key data
    pub(crate) key_data: [u8; 33],
}

impl Drop for ExtendedKeyPayload {
    fn drop(&mut self) {
        self.key_data.zeroize();
    }
}

impl ExtendedKeyPayload {
    /// Returns the version bytes.
    pub const fn version(&self) -> Version {
        self.version
    }

    /// Length of a serialized extended key payload (without Base58Check).
    const KEY_PAYLOAD_LENGTH: usize = 78;
    /// Base58Check adds 4 checksum bytes, so decode needs 82 bytes and encode needs the
    /// Base58 max-length upper bound for 82 bytes: len + (len + 1) / 2 = 123.
    const KEY_PAYLOAD_WITH_CHECKSUM_LENGTH: usize = Self::KEY_PAYLOAD_LENGTH + 4;
    const MAX_KEY_PAYLOAD_STRING_LENGTH: usize =
        Self::KEY_PAYLOAD_WITH_CHECKSUM_LENGTH + Self::KEY_PAYLOAD_WITH_CHECKSUM_LENGTH.div_ceil(2);

    /// Serializes this key into a 78-byte payload.
    fn serialize(&self) -> [u8; Self::KEY_PAYLOAD_LENGTH] {
        let mut out = [0u8; Self::KEY_PAYLOAD_LENGTH];
        out[..4].copy_from_slice(&self.version.to_bytes());
        out[4] = self.meta.depth;
        out[5..9].copy_from_slice(&self.meta.parent_fingerprint);
        out[9..13].copy_from_slice(&self.meta.child_number.to_be_bytes());
        out[13..45].copy_from_slice(&self.meta.chain_code);
        out[45..78].copy_from_slice(&self.key_data);
        out
    }
}

impl core::fmt::Display for ExtendedKeyPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let serialized = Zeroizing::new(self.serialize());
        let mut encoded = String::with_capacity(Self::MAX_KEY_PAYLOAD_STRING_LENGTH);
        bs58::encode(&serialized[..])
            .with_check()
            .onto(&mut encoded)
            .expect("base58 encoding should fit the fixed buffer");
        f.write_str(&encoded)
    }
}

impl core::str::FromStr for ExtendedKeyPayload {
    type Err = Error;

    fn from_str(encoded: &str) -> Result<Self> {
        let mut data = Zeroizing::new([0u8; Self::KEY_PAYLOAD_WITH_CHECKSUM_LENGTH]);
        let len = bs58::decode(encoded).with_check(None).onto(&mut data[..]).map_err(|err| {
            Error::new(ErrorKind::InvalidPayload, "invalid base58check encoding")
                .with_context("encoded_len", encoded.len())
                .set_source({
                    #[cfg(feature = "std")]
                    {
                        anyhow::Error::new(err)
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        anyhow::Error::msg(err)
                    }
                })
        })?;

        if len != Self::KEY_PAYLOAD_LENGTH {
            return Err(Error::new(ErrorKind::InvalidPayload, "invalid extended key length")
                .with_context("decoded_len", len)
                .with_context("expected_len", Self::KEY_PAYLOAD_LENGTH));
        }

        parse_payload(&data[..len])
    }
}

pub(crate) fn parse_payload(data: &[u8]) -> Result<ExtendedKeyPayload> {
    if data.len() != ExtendedKeyPayload::KEY_PAYLOAD_LENGTH {
        return Err(Error::new(ErrorKind::InvalidPayload, "invalid extended key length")
            .with_context("decoded_len", data.len())
            .with_context("expected_len", ExtendedKeyPayload::KEY_PAYLOAD_LENGTH));
    }

    let mut raw_version_bytes = [0u8; 4];
    raw_version_bytes.copy_from_slice(&data[0..4]);
    let raw_version = u32::from_be_bytes(raw_version_bytes);

    let depth = data[4];

    let mut raw_parent_fingerprint = [0u8; 4];
    raw_parent_fingerprint.copy_from_slice(&data[5..9]);

    let mut child_number_bytes = [0u8; 4];
    child_number_bytes.copy_from_slice(&data[9..13]);
    let child_number = u32::from_be_bytes(child_number_bytes);

    let mut chain_code = [0u8; 32];
    chain_code.copy_from_slice(&data[13..45]);

    let mut key_data = [0u8; 33];
    key_data.copy_from_slice(&data[45..78]);

    if depth == 0 {
        if raw_parent_fingerprint != [0u8; 4] {
            return Err(Error::new(
                ErrorKind::InvalidPayload,
                "zero depth with non-zero parent fingerprint",
            )
            .with_context("depth", depth)
            .with_context(
                "parent_fingerprint",
                format!(
                    "0x{:02X}{:02X}{:02X}{:02X}",
                    raw_parent_fingerprint[0],
                    raw_parent_fingerprint[1],
                    raw_parent_fingerprint[2],
                    raw_parent_fingerprint[3],
                ),
            ));
        }
        if child_number != 0 {
            return Err(Error::new(
                ErrorKind::InvalidPayload,
                "zero depth with non-zero child number",
            )
            .with_context("depth", depth)
            .with_context("child_number", child_number));
        }
    }

    let version = if let Some(known) = KnownVersion::from_raw(raw_version) {
        let version = Version::from(known);
        match version {
            Version::Public(_) => {
                if !matches!(key_data[0], 0x02 | 0x03) {
                    return Err(Error::new(ErrorKind::InvalidKeyData, "invalid public key prefix")
                        .with_context("key_prefix", format!("0x{:02X}", key_data[0])));
                }
            },
            Version::Private(_) => {
                if key_data[0] != 0x00 {
                    return Err(Error::new(
                        ErrorKind::InvalidKeyData,
                        "invalid private key prefix",
                    )
                    .with_context("key_prefix", format!("0x{:02X}", key_data[0])));
                }
            },
        }
        version
    } else {
        match key_data[0] {
            // xprv: leading 0x00 + 32-byte secret
            0x00 => Version::private(raw_version),
            // xpub: compressed secp256k1 key prefix
            0x02 | 0x03 => Version::public(raw_version),
            // reject non-BIP32 key data
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidKeyData,
                    "invalid private/public key data prefix",
                )
                .with_context("key_prefix", format!("0x{:02X}", key_data[0])));
            },
        }
    };

    Ok(ExtendedKeyPayload {
        version,
        meta: ExtendedKeyMetadata {
            depth,
            parent_fingerprint: raw_parent_fingerprint,
            child_number,
            chain_code,
        },
        key_data,
    })
}

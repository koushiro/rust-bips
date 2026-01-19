#[cfg(not(feature = "std"))]
use alloc::format;
use core::str::FromStr;

use hmac::Mac;
use zeroize::Zeroizing;

use super::{
    ExtendedKeyMetadata, hmac_sha512_split, key_fingerprint,
    payload::{ExtendedKeyPayload, Version},
};
use crate::{
    backend::*,
    error::{Error, ErrorKind, Result},
    path::{ChildNumber, DerivationPath},
};

/// A BIP32 extended public key.
pub struct ExtendedPublicKey<B: Secp256k1Backend> {
    /// 41 bytes: the metadata for extended key (depth, parent link, and chain code).
    pub(crate) meta: ExtendedKeyMetadata,
    /// 33 bytes: the public key data
    pub(crate) public_key: PublicKey<B>,
}

impl<B: Secp256k1Backend> Clone for ExtendedPublicKey<B> {
    fn clone(&self) -> Self {
        Self { meta: self.meta.clone(), public_key: self.public_key.clone() }
    }
}

impl<B: Secp256k1Backend> ExtendedPublicKey<B> {
    /// Derives a child extended public key (non-hardened only).
    pub fn derive_child(&self, child: ChildNumber) -> Result<Self> {
        if child.is_hardened() {
            return Err(Error::new(
                ErrorKind::InvalidDerivation,
                "cannot derive hardened child from public key",
            )
            .with_context("child_index", child.index())
            .with_context("hardened", true));
        }

        let public_key_bytes = self.public_key.to_bytes();
        let (left, right) = hmac_sha512_split(&self.meta.chain_code, |mac| {
            mac.update(&public_key_bytes);
            mac.update(&child.to_bytes());
        });

        let left = Zeroizing::new(left);
        let child_public = self.public_key.add_tweak(&left).map_err(|err| {
            Error::new(ErrorKind::InvalidDerivation, "invalid child public key")
                .with_context("child_index", child.index())
                .with_context("hardened", false)
                .set_source(err)
        })?;

        Ok(Self {
            meta: ExtendedKeyMetadata {
                depth: self.meta.depth.saturating_add(1),
                parent_fingerprint: key_fingerprint(&public_key_bytes),
                child_number: child.into(),
                chain_code: right,
            },
            public_key: child_public,
        })
    }

    /// Derives a child extended public key along a path (non-hardened only).
    pub fn derive_path(&self, path: &DerivationPath) -> Result<Self> {
        let mut key = self.clone();
        for child in path.children() {
            key = key.derive_child(*child)?;
        }
        Ok(key)
    }

    /// Encodes this key with the specified version bytes.
    pub fn encode_with(&self, version: Version) -> Result<ExtendedKeyPayload> {
        if !version.is_public() {
            return Err(Error::new(ErrorKind::InvalidVersion, "expected public version bytes")
                .with_context("version", version));
        }

        Ok(self.encode_with_unchecked(version))
    }

    /// Encodes this key with the specified version bytes without validation.
    pub fn encode_with_unchecked(&self, version: Version) -> ExtendedKeyPayload {
        ExtendedKeyPayload {
            version,
            meta: self.meta.clone(),
            key_data: self.public_key.to_bytes(),
        }
    }
}

impl<B: Secp256k1Backend> TryFrom<ExtendedKeyPayload> for ExtendedPublicKey<B> {
    type Error = Error;

    fn try_from(payload: ExtendedKeyPayload) -> Result<Self> {
        if !payload.version.is_public() {
            return Err(Error::new(ErrorKind::InvalidVersion, "extended key is not public")
                .with_context("version", payload.version));
        }

        let public_key = <PublicKey<B> as Secp256k1PublicKey>::from_bytes(&payload.key_data)
            .map_err(|err| {
                Error::new(ErrorKind::InvalidKeyData, "invalid secp256k1 public key data")
                    .with_context("key_prefix", format!("0x{:02x}", payload.key_data[0]))
                    .set_source(err)
            })?;

        Ok(Self { meta: payload.meta.clone(), public_key })
    }
}

impl<B: Secp256k1Backend> FromStr for ExtendedPublicKey<B> {
    type Err = Error;

    fn from_str(encoded: &str) -> Result<Self> {
        let payload = encoded.parse::<ExtendedKeyPayload>()?;
        Self::try_from(payload)
    }
}

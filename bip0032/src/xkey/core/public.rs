#[cfg(not(feature = "std"))]
use alloc::format;
use core::str::FromStr;

use hmac::Mac;
use zeroize::Zeroizing;

use super::{ExtendedKeyMetadata, hmac_sha512_split, key_fingerprint};
use crate::{
    curve::{Bip32Curve, Curve, CurvePublicKey, TweakableKey},
    error::{Error, ErrorKind, Result},
    path::{ChildNumber, DerivationPath},
    xkey::{Version, payload::ExtendedKeyPayload},
};

/// A BIP32 extended public key.
pub struct ExtendedPublicKey<C: Curve> {
    /// 41 bytes: the metadata for extended key (depth, parent link, and chain code).
    pub(crate) meta: ExtendedKeyMetadata,
    /// Public key data.
    pub(crate) public_key: C::PublicKey,
}

impl<C: Curve> Clone for ExtendedPublicKey<C> {
    fn clone(&self) -> Self {
        Self { meta: self.meta.clone(), public_key: self.public_key.clone() }
    }
}

impl<C: Curve> ExtendedPublicKey<C> {
    /// Returns the fingerprint of the parent's key.
    pub fn parent_fingerprint(&self) -> [u8; 4] {
        self.meta.parent_fingerprint
    }

    /// Returns the chain code for this key.
    pub fn chain_code(&self) -> [u8; 32] {
        self.meta.chain_code
    }

    /// Returns the public key bytes.
    pub fn to_bytes(&self) -> <C::PublicKey as CurvePublicKey>::Bytes {
        CurvePublicKey::to_bytes(&self.public_key)
    }
}

impl<C> ExtendedPublicKey<C>
where
    C: Bip32Curve,
    C::PublicKey: TweakableKey,
{
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

        let public_key_bytes = CurvePublicKey::to_bytes(&self.public_key);
        let (left, right) = hmac_sha512_split(&self.meta.chain_code, |mac| {
            mac.update(public_key_bytes.as_ref());
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
                parent_fingerprint: key_fingerprint(public_key_bytes.as_ref()),
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
}

// BIP32 encoding
impl<C> ExtendedPublicKey<C>
where
    C: Bip32Curve,
    C::PublicKey: CurvePublicKey<Bytes = [u8; 33]>,
{
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
            key_data: CurvePublicKey::to_bytes(&self.public_key),
        }
    }
}

// BIP32 decoding
impl<C> FromStr for ExtendedPublicKey<C>
where
    C: Bip32Curve,
    C::PublicKey: CurvePublicKey<Bytes = [u8; 33]>,
{
    type Err = Error;

    fn from_str(encoded: &str) -> Result<Self> {
        let payload = encoded.parse::<ExtendedKeyPayload>()?;
        Self::try_from(payload)
    }
}

impl<C> TryFrom<ExtendedKeyPayload> for ExtendedPublicKey<C>
where
    C: Bip32Curve,
    C::PublicKey: CurvePublicKey<Bytes = [u8; 33]>,
{
    type Error = Error;

    fn try_from(payload: ExtendedKeyPayload) -> Result<Self> {
        if !payload.version.is_public() {
            return Err(Error::new(ErrorKind::InvalidVersion, "extended key is not public")
                .with_context("version", payload.version));
        }

        let public_key =
            <C::PublicKey as CurvePublicKey>::from_bytes(&payload.key_data).map_err(|err| {
                Error::new(ErrorKind::InvalidKeyData, "invalid public key data")
                    .with_context("key_prefix", format!("0x{:02x}", payload.key_data[0]))
                    .set_source(err)
            })?;

        Ok(Self { meta: payload.meta.clone(), public_key })
    }
}

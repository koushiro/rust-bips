#[cfg(not(feature = "std"))]
use alloc::format;
use core::str::FromStr;

use hmac::Mac;
use zeroize::Zeroizing;

use super::{ExtendedKeyMetadata, hmac_sha512_split, key_fingerprint, public::ExtendedPublicKey};
use crate::{
    curve::{Bip32Curve, Curve, CurvePrivateKey, CurvePublicKey, TweakableKey},
    error::{Error, ErrorKind, Result},
    path::{ChildNumber, DerivationPath},
    xkey::{Version, payload::ExtendedKeyPayload},
};

/// A BIP32 extended private key.
pub struct ExtendedPrivateKey<C: Curve> {
    /// 41 bytes: the metadata for extended key (depth, parent link, and chain code).
    pub(crate) meta: ExtendedKeyMetadata,
    /// Private key data.
    pub(crate) private_key: C::PrivateKey,
}

impl<C: Curve> Clone for ExtendedPrivateKey<C> {
    fn clone(&self) -> Self {
        Self { meta: self.meta.clone(), private_key: self.private_key.clone() }
    }
}

fn derive_master_key_parts(seed: &[u8], domain: &[u8]) -> ([u8; 32], [u8; 32]) {
    hmac_sha512_split(domain, |mac| mac.update(seed))
}

impl<C: Curve> ExtendedPrivateKey<C> {
    /// Generates a master extended private key from a seed.
    pub fn new(seed: &[u8]) -> Result<Self>
    where
        C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]>,
    {
        let (master_key, chain_code) = derive_master_key_parts(seed, C::HMAC_KEY);

        let master_key = Zeroizing::new(master_key);
        let private_key =
            <C::PrivateKey as CurvePrivateKey>::from_bytes(&*master_key).map_err(|err| {
                Error::new(ErrorKind::InvalidKeyData, "invalid master key derived from seed")
                    .with_context("seed_len", seed.len())
                    .set_source(err)
            })?;

        Ok(Self {
            meta: ExtendedKeyMetadata {
                depth: 0,
                parent_fingerprint: Some([0u8; 4]),
                child_number: 0,
                chain_code,
            },
            private_key,
        })
    }

    /// Returns the corresponding extended public key.
    pub fn public_key(&self) -> ExtendedPublicKey<C> {
        ExtendedPublicKey { meta: self.meta.clone(), public_key: self.private_key.to_public() }
    }
}

impl<C> ExtendedPrivateKey<C>
where
    C: Bip32Curve,
    C::PrivateKey: TweakableKey,
{
    /// Derives a child extended private key.
    pub fn derive_child(&self, child: ChildNumber) -> Result<Self> {
        let parent_public = self.private_key.to_public();
        let parent_public_bytes = CurvePublicKey::to_bytes(&parent_public);

        let (left, right) = hmac_sha512_split(&self.meta.chain_code, |mac| {
            if child.is_hardened() {
                let mut data = Zeroizing::new([0u8; 1 + 32 + 4]);
                data[1..33].copy_from_slice(CurvePrivateKey::to_bytes(&self.private_key).as_ref());
                data[33..].copy_from_slice(&child.to_bytes());
                mac.update(data.as_ref());
            } else {
                mac.update(parent_public_bytes.as_ref());
                mac.update(&child.to_bytes());
            }
        });

        let left = Zeroizing::new(left);
        let child_key = self.private_key.add_tweak(&left).map_err(|err| {
            Error::new(ErrorKind::InvalidDerivation, "invalid child private key")
                .with_context("child_index", child.index())
                .with_context("hardened", child.is_hardened())
                .set_source(err)
        })?;

        Ok(Self {
            meta: ExtendedKeyMetadata {
                depth: self.meta.depth.saturating_add(1),
                parent_fingerprint: Some(key_fingerprint(parent_public_bytes.as_ref())),
                child_number: child.into(),
                chain_code: right,
            },
            private_key: child_key,
        })
    }

    /// Derives a child extended private key along a path.
    pub fn derive_path(&self, path: &DerivationPath) -> Result<Self> {
        let mut key = self.clone();
        for child in path.children() {
            key = key.derive_child(*child)?;
        }
        Ok(key)
    }
}

impl<C> ExtendedPrivateKey<C>
where
    C: Bip32Curve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]>,
{
    /// Encodes this key with the specified version bytes.
    pub fn encode_with(&self, version: Version) -> Result<ExtendedKeyPayload> {
        if !version.is_private() {
            return Err(Error::new(ErrorKind::InvalidVersion, "expected private version bytes")
                .with_context("version", version));
        }

        if self.meta.parent_fingerprint.is_none() {
            return Err(Error::new(ErrorKind::InvalidPayload, "missing parent fingerprint")
                .with_context("depth", self.meta.depth));
        }

        Ok(self.encode_with_unchecked(version))
    }

    /// Encodes this key with the specified version bytes without validation.
    pub fn encode_with_unchecked(&self, version: Version) -> ExtendedKeyPayload {
        ExtendedKeyPayload {
            version,
            meta: self.meta.clone(),
            key_data: {
                let mut key_data = [0u8; 33];
                key_data[1..]
                    .copy_from_slice(CurvePrivateKey::to_bytes(&self.private_key).as_ref());
                key_data
            },
        }
    }
}

impl<C> TryFrom<ExtendedKeyPayload> for ExtendedPrivateKey<C>
where
    C: Bip32Curve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]>,
{
    type Error = Error;

    fn try_from(payload: ExtendedKeyPayload) -> Result<Self> {
        if !payload.version.is_private() {
            return Err(Error::new(ErrorKind::InvalidVersion, "extended key is not private")
                .with_context("version", payload.version));
        }

        let mut raw = Zeroizing::new([0u8; 32]);
        raw.copy_from_slice(&payload.key_data[1..]);

        let private_key = <C::PrivateKey as CurvePrivateKey>::from_bytes(&*raw).map_err(|err| {
            Error::new(ErrorKind::InvalidKeyData, "invalid private key data")
                .with_context("key_prefix", format!("0x{:02x}", payload.key_data[0]))
                .set_source(err)
        })?;

        Ok(Self { meta: payload.meta.clone(), private_key })
    }
}

impl<C> FromStr for ExtendedPrivateKey<C>
where
    C: Bip32Curve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]>,
{
    type Err = Error;

    fn from_str(encoded: &str) -> Result<Self> {
        let payload = encoded.parse::<ExtendedKeyPayload>()?;
        Self::try_from(payload)
    }
}

impl<C: Curve> Drop for ExtendedPrivateKey<C> {
    fn drop(&mut self) {
        CurvePrivateKey::zeroize(&mut self.private_key);
    }
}

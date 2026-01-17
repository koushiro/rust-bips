use anyhow::{Result, anyhow};
use hmac::Mac;
use zeroize::Zeroizing;

use super::{
    ExtendedKeyMetadata, hmac_sha512_split, key_fingerprint,
    payload::{ExtendedKeyPayload, Version},
};
use crate::{
    backend::*,
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
            return Err(anyhow!("cannot derive hardened child from public key"));
        }

        let public_key_bytes = self.public_key.to_bytes();
        let (left, right) = hmac_sha512_split(&self.meta.chain_code, |mac| {
            mac.update(&public_key_bytes);
            mac.update(&child.to_bytes());
        });

        let left = Zeroizing::new(left);
        let child_public = self.public_key.add_tweak(&left)?;

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
            return Err(anyhow!("expected public version bytes"));
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
    type Error = anyhow::Error;

    fn try_from(payload: ExtendedKeyPayload) -> Result<Self> {
        if !payload.version.is_public() {
            return Err(anyhow!("extended key is not public"));
        }

        let public_key = <PublicKey<B> as Secp256k1PublicKey>::from_bytes(&payload.key_data)
            .map_err(|_| anyhow!("invalid public key data"))?;

        Ok(Self { meta: payload.meta.clone(), public_key })
    }
}

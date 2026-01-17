use anyhow::{Result, anyhow};
use hmac::Mac;
use zeroize::Zeroizing;

use super::{
    ExtendedKeyMetadata, hmac_sha512_split, key_fingerprint,
    payload::{ExtendedKeyPayload, Version},
    public::ExtendedPublicKey,
};
use crate::{
    backend::*,
    path::{ChildNumber, DerivationPath},
};

/// A BIP32 extended private key.
pub struct ExtendedPrivateKey<B: Secp256k1Backend> {
    /// 41 bytes: the metadata for extended key (depth, parent link, and chain code).
    pub(crate) meta: ExtendedKeyMetadata,
    /// 33 bytes: the private key data
    pub(crate) private_key: PrivateKey<B>,
}

impl<B: Secp256k1Backend> Clone for ExtendedPrivateKey<B> {
    fn clone(&self) -> Self {
        Self { meta: self.meta.clone(), private_key: self.private_key.clone() }
    }
}

fn derive_master_key_parts(seed: &[u8]) -> ([u8; 32], [u8; 32]) {
    const MASTER_KEY_DOMAIN: &[u8] = b"Bitcoin seed";
    hmac_sha512_split(MASTER_KEY_DOMAIN, |mac| mac.update(seed))
}

impl<B: Secp256k1Backend> ExtendedPrivateKey<B> {
    /// Generates a master extended private key from a seed.
    pub fn new(seed: &[u8]) -> Result<Self> {
        let (master_key, chain_code) = derive_master_key_parts(seed);

        let master_key = Zeroizing::new(master_key);
        let private_key = <PrivateKey<B> as Secp256k1PrivateKey>::from_bytes(&master_key)
            .map_err(|_| anyhow!("invalid master key derived from seed"))?;

        Ok(Self {
            meta: ExtendedKeyMetadata {
                depth: 0,
                parent_fingerprint: [0u8; 4],
                child_number: 0,
                chain_code,
            },
            private_key,
        })
    }

    /// Derives a child extended private key.
    pub fn derive_child(&self, child: ChildNumber) -> Result<Self> {
        let parent_public = self.private_key.to_public();
        let parent_public_bytes = parent_public.to_bytes();

        let (left, right) = hmac_sha512_split(&self.meta.chain_code, |mac| {
            if child.is_hardened() {
                let mut data = Zeroizing::new([0u8; 1 + 32 + 4]);
                data[1..33].copy_from_slice(&self.private_key.to_bytes());
                data[33..].copy_from_slice(&child.to_bytes());
                mac.update(data.as_ref());
            } else {
                mac.update(&parent_public_bytes);
                mac.update(&child.to_bytes());
            }
        });

        let left = Zeroizing::new(left);
        let child_key = self.private_key.add_tweak(&left)?;

        Ok(Self {
            meta: ExtendedKeyMetadata {
                depth: self.meta.depth.saturating_add(1),
                parent_fingerprint: key_fingerprint(&parent_public_bytes),
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

    /// Returns the corresponding extended public key.
    pub fn public_key(&self) -> ExtendedPublicKey<B> {
        ExtendedPublicKey { meta: self.meta.clone(), public_key: self.private_key.to_public() }
    }

    /// Encodes this key with the specified version bytes.
    pub fn encode_with(&self, version: Version) -> Result<ExtendedKeyPayload> {
        if !version.is_private() {
            return Err(anyhow!("expected private version bytes"));
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
                key_data[1..].copy_from_slice(&self.private_key.to_bytes());
                key_data
            },
        }
    }
}

impl<B: Secp256k1Backend> TryFrom<ExtendedKeyPayload> for ExtendedPrivateKey<B> {
    type Error = anyhow::Error;

    fn try_from(payload: ExtendedKeyPayload) -> Result<Self> {
        if !payload.version.is_private() {
            return Err(anyhow!("extended key is not private"));
        }

        let mut raw = Zeroizing::new([0u8; 32]);
        raw.copy_from_slice(&payload.key_data[1..]);

        let private_key = <PrivateKey<B> as Secp256k1PrivateKey>::from_bytes(&raw)
            .map_err(|_| anyhow!("invalid private key data"))?;

        Ok(Self { meta: payload.meta.clone(), private_key })
    }
}

impl<B: Secp256k1Backend> Drop for ExtendedPrivateKey<B> {
    fn drop(&mut self) {
        Secp256k1PrivateKey::zeroize(&mut self.private_key);
    }
}

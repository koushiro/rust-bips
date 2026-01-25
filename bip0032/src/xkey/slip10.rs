#![doc = include_str!("../../SLIP-0010.md")]

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use hmac::Mac;
use zeroize::Zeroizing;

use crate::{
    curve::*,
    error::{Error, ErrorKind, Result},
    path::{ChildNumber, DerivationPath, HardenedChildNumber, HardenedDerivationPath},
    xkey::core::*,
};

/// SLIP-0010 master key generation.
pub trait Slip10MasterKey {
    /// Generates a SLIP-0010 master private key from a seed.
    fn new_slip10(seed: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

/// Hardened-only SLIP-0010 derivation for private keys.
pub trait Slip10HardenedOnlyDerivation {
    /// Derives a hardened child extended private key (SLIP-0010).
    fn derive_slip10_child(&self, child: HardenedChildNumber) -> Result<Self>
    where
        Self: Sized;

    /// Derives a hardened child extended private key along a path (SLIP-0010).
    fn derive_slip10_path(&self, path: &HardenedDerivationPath) -> Result<Self>
    where
        Self: Sized;
}

/// Non-hardened SLIP-0010 derivation for public/private keys.
pub trait Slip10NonHardenedDerivation {
    /// Derives a child extended public/private key (SLIP-0010).
    fn derive_slip10_child(&self, child: ChildNumber) -> Result<Self>
    where
        Self: Sized;

    /// Derives a child extended public/private key along a path (SLIP-0010).
    fn derive_slip10_path(&self, path: &DerivationPath) -> Result<Self>
    where
        Self: Sized;
}

impl<C> Slip10MasterKey for ExtendedPrivateKey<C>
where
    C: Slip10Curve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]>,
{
    fn new_slip10(seed: &[u8]) -> Result<Self> {
        let mut seed = seed.to_vec();

        loop {
            let (left, right) = derive_master_key_parts(&seed, C::HMAC_KEY);
            let left = Zeroizing::new(left);

            match <C::PrivateKey as CurvePrivateKey>::from_bytes(&*left) {
                Ok(private_key) => {
                    return Ok(Self {
                        meta: ExtendedKeyMetadata {
                            depth: 0,
                            parent_fingerprint: [0u8; 4],
                            child_number: 0,
                            chain_code: right,
                        },
                        private_key,
                    });
                },
                Err(err) => {
                    let mut next = [0u8; 64];
                    next[..32].copy_from_slice(&*left);
                    next[32..].copy_from_slice(&right);
                    seed.clear();
                    seed.extend_from_slice(&next);
                    if seed.is_empty() {
                        return Err(Error::new(
                            ErrorKind::InvalidDerivation,
                            "slip10 seed retry failed",
                        )
                        .set_source(err));
                    }
                },
            }
        }
    }
}

impl<C> Slip10HardenedOnlyDerivation for ExtendedPrivateKey<C>
where
    C: Slip10HardenedOnlyCurve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]>,
{
    fn derive_slip10_child(&self, child: HardenedChildNumber) -> Result<Self> {
        let child_bytes = child.to_bytes();
        let parent_public = self.private_key.to_public();
        let parent_public_bytes = CurvePublicKey::to_bytes(&parent_public);

        let mut data = Zeroizing::new([0u8; 1 + 32 + 4]);
        data[1..33].copy_from_slice(CurvePrivateKey::to_bytes(&self.private_key).as_ref());
        data[33..].copy_from_slice(&child_bytes);

        loop {
            let (left, right) = hmac_sha512_split(&self.meta.chain_code, |mac| {
                mac.update(data.as_ref());
            });

            let left = Zeroizing::new(left);
            let derived = <C::PrivateKey as CurvePrivateKey>::from_bytes(&*left);

            match derived {
                Ok(private_key) => {
                    return Ok(Self {
                        meta: ExtendedKeyMetadata {
                            depth: self.meta.depth.saturating_add(1),
                            parent_fingerprint: key_fingerprint(parent_public_bytes.as_ref()),
                            child_number: ChildNumber::from(child).into(),
                            chain_code: right,
                        },
                        private_key,
                    });
                },
                Err(err) => {
                    let mut retry = Zeroizing::new([0u8; 1 + 32 + 4]);
                    retry[0] = 0x01;
                    retry[1..33].copy_from_slice(&right);
                    retry[33..].copy_from_slice(&child_bytes);
                    data = retry;
                    let _ = err;
                },
            }
        }
    }

    fn derive_slip10_path(&self, path: &HardenedDerivationPath) -> Result<Self> {
        let mut key = self.clone();
        for child in path.children() {
            key = key.derive_slip10_child(child)?;
        }
        Ok(key)
    }
}

impl<C> Slip10NonHardenedDerivation for ExtendedPrivateKey<C>
where
    C: Slip10NonHardenedCurve,
    C::PrivateKey: CurvePrivateKey<Bytes = [u8; 32]> + TweakableKey,
    C::PublicKey: CurvePublicKey<Bytes = [u8; 33]>,
{
    fn derive_slip10_child(&self, child: ChildNumber) -> Result<Self> {
        let hardened = child.is_hardened();
        let parent_public = self.private_key.to_public();
        let parent_public_bytes = CurvePublicKey::to_bytes(&parent_public);
        let child_bytes = child.to_bytes();

        let mut data = Zeroizing::new([0u8; 1 + 32 + 4]);
        if hardened {
            data[1..33].copy_from_slice(CurvePrivateKey::to_bytes(&self.private_key).as_ref());
            data[33..].copy_from_slice(&child_bytes);
        } else {
            data[..33].copy_from_slice(parent_public_bytes.as_ref());
            data[33..].copy_from_slice(&child_bytes);
        }

        loop {
            let (left, right) = hmac_sha512_split(&self.meta.chain_code, |mac| {
                mac.update(data.as_ref());
            });

            let left = Zeroizing::new(left);
            let derived = self.private_key.add_tweak(&left);

            match derived {
                Ok(private_key) => {
                    return Ok(Self {
                        meta: ExtendedKeyMetadata {
                            depth: self.meta.depth.saturating_add(1),
                            parent_fingerprint: key_fingerprint(parent_public_bytes.as_ref()),
                            child_number: child.into(),
                            chain_code: right,
                        },
                        private_key,
                    });
                },
                Err(err) => {
                    let mut retry = Zeroizing::new([0u8; 1 + 32 + 4]);
                    retry[0] = 0x01;
                    retry[1..33].copy_from_slice(&right);
                    retry[33..].copy_from_slice(&child_bytes);
                    data = retry;
                    let _ = err;
                },
            }
        }
    }

    fn derive_slip10_path(&self, path: &DerivationPath) -> Result<Self> {
        let mut key = self.clone();
        for child in path.children() {
            key = key.derive_slip10_child(*child)?;
        }
        Ok(key)
    }
}

impl<C> Slip10NonHardenedDerivation for ExtendedPublicKey<C>
where
    C: Slip10NonHardenedCurve,
    C::PublicKey: CurvePublicKey<Bytes = [u8; 33]> + TweakableKey,
{
    fn derive_slip10_child(&self, child: ChildNumber) -> Result<Self> {
        if child.is_hardened() {
            return Err(Error::new(
                ErrorKind::InvalidDerivation,
                "cannot derive hardened child from public key",
            )
            .with_context("child_index", child.index())
            .with_context("hardened", true));
        }

        let parent_public_bytes = CurvePublicKey::to_bytes(&self.public_key);
        let child_bytes = child.to_bytes();
        let mut data = Zeroizing::new([0u8; 1 + 32 + 4]);
        data[..33].copy_from_slice(parent_public_bytes.as_ref());
        data[33..].copy_from_slice(&child_bytes);

        loop {
            let (left, right) = hmac_sha512_split(&self.meta.chain_code, |mac| {
                mac.update(data.as_ref());
            });

            let left = Zeroizing::new(left);
            let derived = self.public_key.add_tweak(&left);

            match derived {
                Ok(public_key) => {
                    return Ok(Self {
                        meta: ExtendedKeyMetadata {
                            depth: self.meta.depth.saturating_add(1),
                            parent_fingerprint: key_fingerprint(parent_public_bytes.as_ref()),
                            child_number: child.into(),
                            chain_code: right,
                        },
                        public_key,
                    });
                },
                Err(err) => {
                    let mut retry = Zeroizing::new([0u8; 1 + 32 + 4]);
                    retry[0] = 0x01;
                    retry[1..33].copy_from_slice(&right);
                    retry[33..].copy_from_slice(&child_bytes);
                    data = retry;
                    let _ = err;
                },
            }
        }
    }

    fn derive_slip10_path(&self, path: &DerivationPath) -> Result<Self> {
        let mut key = self.clone();
        for child in path.children() {
            key = key.derive_slip10_child(*child)?;
        }
        Ok(key)
    }
}

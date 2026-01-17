use anyhow::{Result, anyhow};
use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey, SignOnly, VerifyOnly};
use zeroize::Zeroizing;

use super::{Secp256k1Backend, Secp256k1PrivateKey, Secp256k1PublicKey};

/// Secp256k1 FFI backend powered by the [`secp256k1`](https://github.com/rust-bitcoin/rust-secp256k1) crate.
pub struct Secp256k1FfiBackend;

// RAII guard to erase scalar material on drop.
struct ScalarGuard(Scalar);

impl ScalarGuard {
    fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let bytes = Zeroizing::new(*bytes);
        Scalar::from_be_bytes(*bytes).map(Self).map_err(|_| anyhow!("invalid scalar"))
    }
}

impl AsRef<Scalar> for ScalarGuard {
    fn as_ref(&self) -> &Scalar {
        &self.0
    }
}

impl Drop for ScalarGuard {
    fn drop(&mut self) {
        self.0.non_secure_erase();
    }
}

#[inline]
fn with_signing_context<R>(f: impl FnOnce(&Secp256k1<SignOnly>) -> R) -> R {
    #[cfg(not(feature = "std"))]
    {
        let secp = Secp256k1::signing_only();
        f(&secp)
    }

    #[cfg(feature = "std")]
    {
        fn signing_context() -> &'static Secp256k1<SignOnly> {
            use std::sync::OnceLock;

            static CONTEXT: OnceLock<Secp256k1<SignOnly>> = OnceLock::new();
            CONTEXT.get_or_init(Secp256k1::signing_only)
        }
        f(signing_context())
    }
}

#[inline]
fn with_verification_context<R>(f: impl FnOnce(&Secp256k1<VerifyOnly>) -> R) -> R {
    #[cfg(not(feature = "std"))]
    {
        let secp = Secp256k1::verification_only();
        f(&secp)
    }
    #[cfg(feature = "std")]
    {
        fn verification_context() -> &'static Secp256k1<VerifyOnly> {
            use std::sync::OnceLock;

            static CONTEXT: OnceLock<Secp256k1<VerifyOnly>> = OnceLock::new();
            CONTEXT.get_or_init(Secp256k1::verification_only)
        }
        f(verification_context())
    }
}

impl Secp256k1PublicKey for PublicKey {
    fn from_bytes(bytes: &[u8; 33]) -> Result<Self> {
        PublicKey::from_byte_array_compressed(*bytes).map_err(|_| anyhow!("invalid public key"))
    }

    fn to_bytes(&self) -> [u8; 33] {
        self.serialize()
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self> {
        let scalar = ScalarGuard::from_bytes(tweak)?;

        with_verification_context(|secp| {
            (*self)
                .add_exp_tweak(secp, scalar.as_ref())
                .map_err(|_| anyhow!("derived public key is invalid"))
        })
    }
}

impl Secp256k1PrivateKey for SecretKey {
    type PublicKey = PublicKey;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        SecretKey::from_byte_array(*bytes).map_err(|_| anyhow!("invalid secret key"))
    }

    fn to_bytes(&self) -> [u8; 32] {
        self.secret_bytes()
    }

    fn add_tweak(&self, tweak: &[u8; 32]) -> Result<Self> {
        let scalar = ScalarGuard::from_bytes(tweak)?;

        (*self)
            .add_tweak(scalar.as_ref())
            .map_err(|_| anyhow!("derived secret key is invalid"))
    }

    fn to_public(&self) -> Self::PublicKey {
        with_signing_context(|secp| PublicKey::from_secret_key(secp, self))
    }

    fn zeroize(&mut self) {
        self.non_secure_erase();
    }
}

impl Secp256k1Backend for Secp256k1FfiBackend {
    type PublicKey = PublicKey;
    type PrivateKey = SecretKey;
}

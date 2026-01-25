use ed25519_dalek::{SigningKey, VerifyingKey};

use crate::curve::{CurveError, CurvePrivateKey, CurvePublicKey, ed25519::Ed25519Backend};

/// Ed25519 backend powered by the [`ed25519-dalek`](https://github.com/dalek-cryptography/curve25519-dalek/tree/main/ed25519-dalek) crate.
pub struct Ed25519DalekBackend;

impl CurvePublicKey for VerifyingKey {
    type Error = CurveError;
    type Bytes = [u8; 33];

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error> {
        if bytes[0] != 0 {
            return Err(CurveError::from("SLIP-0010 ed25519 public key needs a 0x00 prefix"));
        }

        let mut raw = [0u8; 32];
        raw.copy_from_slice(&bytes[1..]);
        VerifyingKey::from_bytes(&raw).map_err(CurveError::new)
    }

    fn to_bytes(&self) -> Self::Bytes {
        let raw = self.to_bytes();
        let mut out = [0u8; 33];
        out[1..].copy_from_slice(&raw);
        out
    }
}

impl CurvePrivateKey for SigningKey {
    type Error = CurveError;
    type PublicKey = VerifyingKey;
    type Bytes = [u8; 32];

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error> {
        Ok(SigningKey::from_bytes(bytes))
    }

    fn to_bytes(&self) -> Self::Bytes {
        self.to_bytes()
    }

    fn to_public(&self) -> Self::PublicKey {
        self.verifying_key()
    }
}

impl Ed25519Backend for Ed25519DalekBackend {
    type PublicKey = VerifyingKey;
    type PrivateKey = SigningKey;
}

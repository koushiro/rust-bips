//! Version bytes for extended key payloads.

use core::fmt;

/// Extended key version bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Version {
    /// Public key version bytes.
    Public(u32),
    /// Private key version bytes.
    Private(u32),
}

impl Version {
    /// Creates a public version from raw version bytes.
    pub const fn public(value: u32) -> Self {
        Self::Public(value)
    }

    /// Creates a private version from raw version bytes.
    pub const fn private(value: u32) -> Self {
        Self::Private(value)
    }

    /// Creates a public version from big-endian bytes.
    pub const fn from_public_bytes(bytes: [u8; 4]) -> Self {
        Self::Public(u32::from_be_bytes(bytes))
    }

    /// Creates a private version from big-endian bytes.
    pub const fn from_private_bytes(bytes: [u8; 4]) -> Self {
        Self::Private(u32::from_be_bytes(bytes))
    }

    /// Returns the raw version bytes.
    pub const fn as_u32(self) -> u32 {
        match self {
            Version::Public(value) | Version::Private(value) => value,
        }
    }

    /// Returns the big-endian version bytes.
    pub const fn to_bytes(self) -> [u8; 4] {
        self.as_u32().to_be_bytes()
    }

    /// Returns whether this version is public.
    pub const fn is_public(self) -> bool {
        matches!(self, Version::Public(_))
    }

    /// Returns whether this version is private.
    pub const fn is_private(self) -> bool {
        matches!(self, Version::Private(_))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08X}", self.as_u32())
    }
}

impl Version {
    // ========================================================================
    // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
    // https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
    // https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki
    // ========================================================================
    /// Bitcoin mainnet **P2PKH** or **P2SH** public key version.
    pub const XPUB: Version = Version::Public(0x0488_B21E);
    /// Bitcoin mainnet **P2PKH** or **P2SH** private key version.
    pub const XPRV: Version = Version::Private(0x0488_ADE4);

    /// Bitcoin testnet **P2PKH** or **P2SH** public key version.
    pub const TPUB: Version = Version::Public(0x0435_87CF);
    /// Bitcoin testnet **P2PKH** or **P2SH** private key version.
    pub const TPRV: Version = Version::Private(0x0435_8394);
    // ========================================================================

    // ========================================================================
    // https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki
    // ========================================================================
    /// Bitcoin mainnet **P2SH-P2WPKH** public key version.
    pub const YPUB: Version = Version::Public(0x049D_7CB2);
    /// Bitcoin mainnet **P2SH-P2WPKH** private key version.
    pub const YPRV: Version = Version::Private(0x049D_7878);

    /// Bitcoin mainnet multi-signature **P2SH-P2WSH** public key version.
    pub const YPUB_SHWSH: Version = Version::Public(0x0295_B43F);
    /// Bitcoin mainnet multi-signature **P2SH-P2WSH** private key version.
    pub const YPRV_SHWSH: Version = Version::Private(0x0295_B005);

    /// Bitcoin testnet **P2SH-P2WPKH** public key version.
    pub const UPUB: Version = Version::Public(0x044A_5262);
    /// Bitcoin testnet **P2SH-P2WPKH** private key version.
    pub const UPRV: Version = Version::Private(0x044A_4E28);

    /// Bitcoin testnet multi-signature **P2SH-P2WSH** public key version.
    pub const UPUB_SHWSH: Version = Version::Public(0x0242_89EF);
    /// Bitcoin testnet multi-signature **P2SH-P2WSH** private key version.
    pub const UPRV_SHWSH: Version = Version::Private(0x0242_85B5);
    // ========================================================================

    // ========================================================================
    // https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki
    // ========================================================================
    /// Bitcoin mainnet **P2WPKH** public key version.
    pub const ZPUB: Version = Version::Public(0x04B2_4746);
    /// Bitcoin mainnet **P2WPKH** private key version.
    pub const ZPRV: Version = Version::Private(0x04B2_430C);

    /// Bitcoin mainnet multi-signature **P2WSH** public key version.
    pub const ZPUB_WSH: Version = Version::Public(0x02AA_7ED3);
    /// Bitcoin mainnet multi-signature **P2WSH** private key version.
    pub const ZPRV_WSH: Version = Version::Private(0x02AA_7A99);

    /// Bitcoin testnet **P2WPKH** public key version.
    pub const VPUB: Version = Version::Public(0x045F_1CF6);
    /// Bitcoin testnet **P2WPKH** private key version.
    pub const VPRV: Version = Version::Private(0x045F_18BC);

    /// Bitcoin testnet multi-signature **P2WSH** public key version.
    pub const VPUB_WSH: Version = Version::Public(0x0257_5483);
    /// Bitcoin testnet multi-signature **P2WSH** private key version.
    pub const VPRV_WSH: Version = Version::Private(0x0257_5048);
    // ========================================================================

    /// Returns the known version descriptor for this version, if any.
    pub const fn into_known_version(self) -> Option<KnownVersion> {
        Some(match self {
            Self::XPUB => KnownVersion::Xpub,
            Self::XPRV => KnownVersion::Xprv,
            Self::TPUB => KnownVersion::Tpub,
            Self::TPRV => KnownVersion::Tprv,

            Self::YPUB => KnownVersion::Ypub,
            Self::YPRV => KnownVersion::Yprv,
            Self::YPUB_SHWSH => KnownVersion::YpubShWsh,
            Self::YPRV_SHWSH => KnownVersion::YprvShWsh,

            Self::UPUB => KnownVersion::Upub,
            Self::UPRV => KnownVersion::Uprv,
            Self::UPRV_SHWSH => KnownVersion::UprvShWsh,
            Self::UPUB_SHWSH => KnownVersion::UpubShWsh,

            Self::ZPUB => KnownVersion::Zpub,
            Self::ZPRV => KnownVersion::Zprv,
            Self::ZPUB_WSH => KnownVersion::ZpubWsh,
            Self::ZPRV_WSH => KnownVersion::ZprvWsh,

            Self::VPUB => KnownVersion::Vpub,
            Self::VPRV => KnownVersion::Vprv,
            Self::VPUB_WSH => KnownVersion::VpubWsh,
            Self::VPRV_WSH => KnownVersion::VprvWsh,

            _ => return None,
        })
    }
}

/// Standard extended key versions (BIP32 and common extensions).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KnownVersion {
    /// Bitcoin mainnet **P2PKH** or **P2SH** public key version.
    Xpub,
    /// Bitcoin mainnet **P2PKH** or **P2SH** private key version.
    Xprv,

    /// Bitcoin testnet **P2PKH** or **P2SH** public key version.
    Tpub,
    /// Bitcoin testnet **P2PKH** or **P2SH** private key version.
    Tprv,

    /// Bitcoin mainnet **P2SH-P2WPKH** public key version.
    Ypub,
    /// Bitcoin mainnet **P2SH-P2WPKH** private key version.
    Yprv,

    /// Bitcoin mainnet multi-signature **P2SH-P2WSH** public key version.
    YpubShWsh,
    /// Bitcoin mainnet multi-signature **P2SH-P2WSH** private key version.
    YprvShWsh,

    /// Bitcoin testnet **P2SH-P2WPKH** public key version.
    Upub,
    /// Bitcoin testnet **P2SH-P2WPKH** private key version.
    Uprv,

    /// Bitcoin testnet multi-signature **P2SH-P2WSH** public key version.
    UpubShWsh,
    /// Bitcoin testnet multi-signature **P2SH-P2WSH** private key version.
    UprvShWsh,

    /// Bitcoin mainnet **P2WPKH** public key version.
    Zpub,
    /// Bitcoin mainnet **P2WPKH** private key version.
    Zprv,

    /// Bitcoin mainnet multi-signature **P2WSH** public key version.
    ZpubWsh,
    /// Bitcoin mainnet multi-signature **P2WSH** private key version.
    ZprvWsh,

    /// Bitcoin testnet **P2WPKH** public key version.
    Vpub,
    /// Bitcoin testnet **P2WPKH** private key version.
    Vprv,

    /// Bitcoin testnet multi-signature **P2WSH** public key version.
    VpubWsh,
    /// Bitcoin testnet multi-signature **P2WSH** private key version.
    VprvWsh,
}

impl KnownVersion {
    /// Returns the version bytes for this known version.
    pub const fn into_version(self) -> Version {
        match self {
            Self::Xpub => Version::XPUB,
            Self::Xprv => Version::XPRV,
            Self::Tpub => Version::TPUB,
            Self::Tprv => Version::TPRV,

            Self::Ypub => Version::YPUB,
            Self::Yprv => Version::YPRV,
            Self::YpubShWsh => Version::YPUB_SHWSH,
            Self::YprvShWsh => Version::YPRV_SHWSH,

            Self::Upub => Version::UPUB,
            Self::Uprv => Version::UPRV,
            Self::UpubShWsh => Version::UPUB_SHWSH,
            Self::UprvShWsh => Version::UPRV_SHWSH,

            Self::Zpub => Version::ZPUB,
            Self::Zprv => Version::ZPRV,
            Self::ZpubWsh => Version::ZPUB_WSH,
            Self::ZprvWsh => Version::ZPRV_WSH,

            Self::Vpub => Version::VPUB,
            Self::Vprv => Version::VPRV,
            Self::VpubWsh => Version::VPUB_WSH,
            Self::VprvWsh => Version::VPRV_WSH,
        }
    }

    /// Returns the known version descriptor for raw version bytes, if any.
    pub fn from_raw(raw: u32) -> Option<Self> {
        Version::Public(raw)
            .into_known_version()
            .or_else(|| Version::Private(raw).into_known_version())
    }
}

#[cfg(test)]
mod tests {
    use super::{KnownVersion, Version};

    const CASES: &[(KnownVersion, Version)] = &[
        // ========================================================================
        // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
        // https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
        // https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki
        // ========================================================================
        (KnownVersion::Xpub, Version::XPUB),
        (KnownVersion::Xprv, Version::XPRV),
        (KnownVersion::Tpub, Version::TPUB),
        (KnownVersion::Tprv, Version::TPRV),
        // ========================================================================

        // ========================================================================
        // https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki
        // ========================================================================
        (KnownVersion::Ypub, Version::YPUB),
        (KnownVersion::Yprv, Version::YPRV),
        (KnownVersion::YpubShWsh, Version::YPUB_SHWSH),
        (KnownVersion::YprvShWsh, Version::YPRV_SHWSH),
        (KnownVersion::Upub, Version::UPUB),
        (KnownVersion::Uprv, Version::UPRV),
        (KnownVersion::UpubShWsh, Version::UPUB_SHWSH),
        (KnownVersion::UprvShWsh, Version::UPRV_SHWSH),
        // ========================================================================

        // ========================================================================
        // https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki
        // ========================================================================
        (KnownVersion::Zpub, Version::ZPUB),
        (KnownVersion::Zprv, Version::ZPRV),
        (KnownVersion::ZpubWsh, Version::ZPUB_WSH),
        (KnownVersion::ZprvWsh, Version::ZPRV_WSH),
        (KnownVersion::Vpub, Version::VPUB),
        (KnownVersion::Vprv, Version::VPRV),
        (KnownVersion::VpubWsh, Version::VPUB_WSH),
        (KnownVersion::VprvWsh, Version::VPRV_WSH),
        // ========================================================================
    ];

    #[test]
    fn known_version_roundtrip() {
        for (known, version) in CASES {
            assert_eq!(known.into_version(), *version);
            assert_eq!(version.into_known_version(), Some(*known));
            assert_eq!(KnownVersion::from_raw(version.as_u32()), Some(*known));
        }
    }

    #[test]
    fn from_raw_unknown_returns_none() {
        assert_eq!(KnownVersion::from_raw(0xDEAD_BEEF), None);
    }
}

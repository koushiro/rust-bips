use std::fmt;

use anyhow::{Result, anyhow, ensure};
use bip0032::{
    ChildNumber, DerivationPath, ExtendedKeyPayload, ExtendedPrivateKey, ExtendedPublicKey,
    Version,
    curve::secp256k1::{K256Backend, Secp256k1Curve},
};
use clap::{Args, ValueEnum, error::ErrorKind};
use serde::Serialize;
use zeroize::Zeroizing;

use crate::args::{Input, Output};

#[derive(Args)]
pub struct Derive {
    /// Derivation path: m/... from a master key, or a relative path such as 0/1.
    #[arg(short = 'p', long, value_name = "PATH")]
    pub path: String,

    #[command(flatten)]
    pub input: Input,

    /// Network for hex seed input only (default: mainnet). Extended keys retain their version.
    #[arg(long, value_enum)]
    pub network: Option<Network>,

    /// Output format (diagnostics always go to stderr).
    #[arg(short = 'o', long, value_enum, value_name = "FORMAT", default_value_t = Output::Text)]
    pub output: Output,
}

#[derive(Clone, Copy, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Mainnet,
    Testnet,
}

impl Network {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet => "testnet",
        }
    }
}

#[derive(Serialize)]
struct Derived<'a> {
    private_key: Option<&'a str>,
    public_key: &'a str,
}

impl fmt::Display for Derived<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(private) = self.private_key {
            writeln!(f, "private: {private}")?;
        }
        write!(f, "public: {}", self.public_key)
    }
}

impl Derive {
    pub fn run(self) -> Result<()> {
        let path = self
            .path
            .parse::<DerivationPath>()
            .map_err(|_| clap::Error::raw(ErrorKind::ValueValidation, "invalid derivation path"))?;

        let absolute = matches!(self.path.as_str(), "m" | "M")
            || self.path.starts_with("m/")
            || self.path.starts_with("M/");

        let input = self.input.read("Hex seed or extended key: ")?;
        let value = input.trim();
        ensure!(!value.is_empty(), "input is empty; expected a hex seed or extended key");

        let key = if value.starts_with("0x") || value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            let seed = Zeroizing::new(
                const_hex::decode(value)
                    .map_err(|_| anyhow!("seed must have an even number of hex digits"))?,
            );
            Key::from_seed(&seed, self.network.unwrap_or(Network::Mainnet))?
        } else {
            let key = Key::parse(value)?;
            ensure!(
                self.network.is_none(),
                clap::Error::raw(
                    ErrorKind::ArgumentConflict,
                    "--network only applies to seed input; extended keys retain their version",
                )
            );
            key
        };

        let child = key.derive(&path, absolute)?;

        let encoded = child.encode()?;
        self.output.write(&Derived {
            private_key: encoded.private.as_ref().map(|value| value.as_str()),
            public_key: &encoded.public,
        })
    }
}

type Curve = Secp256k1Curve<K256Backend>;
type PrivateKey = ExtendedPrivateKey<Curve>;
type PublicKey = ExtendedPublicKey<Curve>;

// Keep the input version alongside the typed key: the library's typed keys do not retain it.
pub enum Key {
    Private(PrivateKey, Version),
    Public(PublicKey, Version),
}

impl Key {
    pub fn parse(input: &str) -> Result<Self> {
        let payload = input
            .parse::<ExtendedKeyPayload>()
            .map_err(|_| anyhow!("invalid extended key encoding, checksum, or metadata"))?;
        let version = payload.version();
        if version.is_private() {
            PrivateKey::try_from(payload)
                .map(|key| Self::Private(key, version))
                .map_err(|_| anyhow!("invalid extended private key data"))
        } else {
            PublicKey::try_from(payload)
                .map(|key| Self::Public(key, version))
                .map_err(|_| anyhow!("invalid extended public key data"))
        }
    }

    fn from_seed(seed: &[u8], network: Network) -> Result<Self> {
        ensure!(
            (16..=64).contains(&seed.len()),
            "seed must contain 16 to 64 bytes (32 to 128 hex digits)"
        );
        let version = match network {
            Network::Mainnet => Version::XPRV,
            Network::Testnet => Version::TPRV,
        };
        PrivateKey::new(seed)
            .map(|key| Self::Private(key, version))
            .map_err(|_| anyhow!("could not derive a master key from the seed"))
    }

    fn derive(&self, path: &DerivationPath, absolute: bool) -> Result<Self> {
        ensure!(
            !absolute || self.depth() == 0,
            "absolute paths require a master key; use a relative path for this key"
        );
        ensure!(
            usize::from(self.depth()) + path.children().len() <= usize::from(u8::MAX),
            "derivation would exceed the maximum extended key depth of 255"
        );
        match self {
            Self::Private(key, version) => key
                .derive_path(path)
                .map(|key| Self::Private(key, *version))
                .map_err(|_| anyhow!("private child key derivation failed")),
            Self::Public(key, version) => {
                ensure!(
                    !path.children().iter().any(|child| child.is_hardened()),
                    "cannot derive a hardened child from an extended public key"
                );
                key.derive_path(path)
                    .map(|key| Self::Public(key, *version))
                    .map_err(|_| anyhow!("public child key derivation failed"))
            },
        }
    }

    pub fn version(&self) -> Version {
        match self {
            Self::Private(_, version) | Self::Public(_, version) => *version,
        }
    }

    pub fn depth(&self) -> u8 {
        match self {
            Self::Private(key, _) => key.depth(),
            Self::Public(key, _) => key.depth(),
        }
    }

    pub fn child_number(&self) -> ChildNumber {
        match self {
            Self::Private(key, _) => key.child_number(),
            Self::Public(key, _) => key.child_number(),
        }
    }

    pub fn parent_fingerprint(&self) -> [u8; 4] {
        match self {
            Self::Private(key, _) => key.parent_fingerprint(),
            Self::Public(key, _) => key.parent_fingerprint(),
        }
    }

    pub fn encode(&self) -> Result<EncodedKeys> {
        match self {
            Self::Private(key, version) => {
                let public_version = version_info(*version)
                    .ok_or_else(|| {
                        anyhow!("cannot determine the public version for this private key version")
                    })?
                    .public;

                let private = key
                    .encode_with(*version)
                    .map_err(|_| anyhow!("could not encode the private key"))?;
                let public = key
                    .public_key()
                    .encode_with(public_version)
                    .map_err(|_| anyhow!("could not encode the public key"))?;

                Ok(EncodedKeys {
                    private: Some(Zeroizing::new(private.to_string())),
                    public: Zeroizing::new(public.to_string()),
                })
            },
            Self::Public(key, version) => {
                let public = key
                    .encode_with(*version)
                    .map_err(|_| anyhow!("could not encode the public key"))?;
                Ok(EncodedKeys { private: None, public: Zeroizing::new(public.to_string()) })
            },
        }
    }
}

pub struct EncodedKeys {
    pub private: Option<Zeroizing<String>>,
    pub public: Zeroizing<String>,
}

pub struct VersionInfo {
    pub name: &'static str,
    pub network: Network,
    pub public: Version,
}

pub fn version_info(version: Version) -> Option<VersionInfo> {
    use Network::{Mainnet, Testnet};

    let (name, network, public) = match version {
        Version::XPRV => ("xprv", Mainnet, Version::XPUB),
        Version::XPUB => ("xpub", Mainnet, Version::XPUB),
        Version::TPRV => ("tprv", Testnet, Version::TPUB),
        Version::TPUB => ("tpub", Testnet, Version::TPUB),
        Version::YPRV => ("yprv", Mainnet, Version::YPUB),
        Version::YPUB => ("ypub", Mainnet, Version::YPUB),
        Version::YPRV_SHWSH => ("Yprv", Mainnet, Version::YPUB_SHWSH),
        Version::YPUB_SHWSH => ("Ypub", Mainnet, Version::YPUB_SHWSH),
        Version::UPRV => ("uprv", Testnet, Version::UPUB),
        Version::UPUB => ("upub", Testnet, Version::UPUB),
        Version::UPRV_SHWSH => ("Uprv", Testnet, Version::UPUB_SHWSH),
        Version::UPUB_SHWSH => ("Upub", Testnet, Version::UPUB_SHWSH),
        Version::ZPRV => ("zprv", Mainnet, Version::ZPUB),
        Version::ZPUB => ("zpub", Mainnet, Version::ZPUB),
        Version::ZPRV_WSH => ("Zprv", Mainnet, Version::ZPUB_WSH),
        Version::ZPUB_WSH => ("Zpub", Mainnet, Version::ZPUB_WSH),
        Version::VPRV => ("vprv", Testnet, Version::VPUB),
        Version::VPUB => ("vpub", Testnet, Version::VPUB),
        Version::VPRV_WSH => ("Vprv", Testnet, Version::VPUB_WSH),
        Version::VPUB_WSH => ("Vpub", Testnet, Version::VPUB_WSH),
        _ => return None,
    };
    Some(VersionInfo { name, network, public })
}

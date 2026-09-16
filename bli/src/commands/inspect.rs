use std::fmt;

use anyhow::{Result, anyhow, ensure};
use bip0032::ChildNumber;
use bip0039::{AnyMnemonic, BuiltInLanguage, Count};
use clap::{Args, ValueEnum, error::ErrorKind};
use serde::Serialize;

use crate::{
    args::{Input, Language, Output},
    commands::derive::{Key, Network, version_info},
};

#[derive(Args)]
pub struct Inspect {
    #[command(flatten)]
    pub input: Input,

    /// Limit mnemonic validation to this wordlist (default: auto-detect).
    #[arg(short = 'l', long, value_enum)]
    pub lang: Option<Language>,

    /// Output format (diagnostics always go to stderr).
    #[arg(short = 'o', long, value_enum, value_name = "FORMAT", default_value_t = Output::Text)]
    pub output: Output,
}

#[derive(Serialize)]
struct MnemonicInfo {
    #[serde(rename = "type")]
    kind: &'static str,
    words: usize,
    languages: Vec<Language>,
}

impl fmt::Display for MnemonicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Valid mnemonic")?;
        writeln!(f, "words: {}", self.words)?;
        let names: Vec<_> = self.languages.iter().map(|lang| lang.name()).collect();
        write!(f, "languages: {}", names.join(", "))
    }
}

#[derive(Serialize)]
struct KeyInfo {
    #[serde(rename = "type")]
    kind: &'static str,
    key_type: &'static str,
    network: Option<Network>,
    version: Option<&'static str>,
    version_bytes: String,
    depth: u8,
    child_number: u32,
    parent_fingerprint: String,
}

impl fmt::Display for KeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Valid extended {} key", self.key_type)?;
        writeln!(f, "network: {}", self.network.map_or("unknown", Network::name))?;
        writeln!(f, "version: {} ({})", self.version.unwrap_or("unknown"), self.version_bytes)?;
        writeln!(f, "depth: {}", self.depth)?;
        let child = ChildNumber::from_bytes(self.child_number.to_be_bytes());
        writeln!(f, "child: {child}")?;
        write!(f, "parent fingerprint: {}", self.parent_fingerprint)
    }
}

impl Inspect {
    pub fn run(self) -> Result<()> {
        let input = self.input.read("Mnemonic or extended key: ")?;
        let value = input.trim();
        ensure!(!value.is_empty(), "input is empty; expected a mnemonic or extended key");

        // Mnemonics have whitespace-separated words; keys are a single Base58Check token.
        if value.split_whitespace().count() > 1 {
            let mnemonic_info = inspect_mnemonic(value, self.lang)?;
            return self.output.write(&mnemonic_info);
        }

        let key_info = inspect_extended_key(value, self.lang)?;
        self.output.write(&key_info)?;

        Ok(())
    }
}

fn inspect_mnemonic(value: &str, language: Option<Language>) -> Result<MnemonicInfo> {
    let words = value.split_whitespace().count();
    Count::try_from(words)
        .map_err(|_| anyhow!("mnemonic must contain 12, 15, 18, 21, or 24 words"))?;

    let languages = if let Some(language) = language {
        AnyMnemonic::validate(BuiltInLanguage::from(language), value).map_err(|_| {
            anyhow!(
                "invalid mnemonic: word or checksum validation failed for the selected language"
            )
        })?;
        vec![language]
    } else {
        let matches = Language::value_variants()
            .iter()
            .copied()
            .filter(|language| {
                AnyMnemonic::validate(BuiltInLanguage::from(*language), value).is_ok()
            })
            .collect::<Vec<_>>();
        ensure!(
            !matches.is_empty(),
            "invalid mnemonic: no built-in wordlist passed word and checksum validation"
        );
        matches
    };

    Ok(MnemonicInfo { kind: "mnemonic", words, languages })
}

fn inspect_extended_key(value: &str, language: Option<Language>) -> Result<KeyInfo> {
    let key = Key::parse(value)?;
    ensure!(
        language.is_none(),
        clap::Error::raw(ErrorKind::ArgumentConflict, "--lang only applies to mnemonic input")
    );

    let version = key.version();
    let known = version_info(version);

    Ok(KeyInfo {
        kind: "extended-key",
        key_type: if version.is_private() { "private" } else { "public" },
        network: known.as_ref().map(|info| info.network),
        version: known.as_ref().map(|info| info.name),
        version_bytes: version.to_string(),
        depth: key.depth(),
        child_number: key.child_number().into(),
        parent_fingerprint: const_hex::encode(key.parent_fingerprint()),
    })
}

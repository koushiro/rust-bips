use std::{fmt, path::PathBuf};

use anyhow::{Result, ensure};
use bip0039::{AnyMnemonic, BuiltInLanguage, Count};
use clap::Args;
use serde::Serialize;
use zeroize::Zeroizing;

use crate::args::{
    Language, Output,
    input::{prompt, read_file},
};

#[derive(Args)]
pub struct Generate {
    /// Number of mnemonic words: 12, 15, 18, 21, or 24.
    #[arg(short = 'w', long, default_value = "12", value_parser = parse_words)]
    pub words: Count,

    /// Mnemonic wordlist language.
    #[arg(short = 'l', long, value_enum, default_value_t = Language::English)]
    pub lang: Language,

    #[command(flatten)]
    pub passphrase: Passphrase,

    /// Output format (diagnostics always go to stderr).
    #[arg(short = 'o', long, value_enum, value_name = "FORMAT", default_value_t = Output::Text)]
    pub output: Output,
}

#[derive(Args)]
pub struct Passphrase {
    /// Prompt twice for a hidden BIP39 passphrase (default: empty passphrase).
    #[arg(long, conflicts_with = "passphrase_file")]
    pub passphrase_prompt: bool,

    /// Read the BIP39 passphrase from a UTF-8 file, or '-' for stdin.
    #[arg(long, value_name = "PATH")]
    pub passphrase_file: Option<PathBuf>,
}

impl Passphrase {
    pub fn read(&self) -> Result<Zeroizing<String>> {
        if let Some(path) = &self.passphrase_file {
            return read_file(path);
        }
        if self.passphrase_prompt {
            let passphrase = prompt("Passphrase: ")?;
            let confirmation = prompt("Confirm passphrase: ")?;
            ensure!(passphrase == confirmation, "passphrases do not match");
            return Ok(passphrase);
        }
        Ok(Zeroizing::new(String::new()))
    }
}

fn parse_words(value: &str) -> std::result::Result<Count, String> {
    value
        .parse::<usize>()
        .ok()
        .and_then(|count| Count::try_from(count).ok())
        .ok_or_else(|| "word count must be 12, 15, 18, 21, or 24".to_owned())
}

#[derive(Serialize)]
struct Generated<'a> {
    mnemonic: &'a str,
    entropy: &'a str,
    seed: &'a str,
    passphrase: &'a str,
    language: Language,
    words: usize,
}

impl fmt::Display for Generated<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "mnemonic: {}", self.mnemonic)?;
        writeln!(f, "entropy: {}", self.entropy)?;
        writeln!(f, "seed: {}", self.seed)?;
        writeln!(f, "language: {}", self.language.name())?;
        write!(f, "words: {}", self.words)
    }
}

impl Generate {
    pub fn run(self) -> Result<()> {
        let passphrase = self.passphrase.read()?;
        let mnemonic = AnyMnemonic::generate(BuiltInLanguage::from(self.lang), self.words);
        let seed = Zeroizing::new(mnemonic.to_seed(passphrase.as_str()));
        let entropy_hex = Zeroizing::new(const_hex::encode_prefixed(mnemonic.entropy()));
        let seed_hex = Zeroizing::new(const_hex::encode_prefixed(seed.as_ref()));

        self.output.write(&Generated {
            mnemonic: mnemonic.phrase(),
            entropy: &entropy_hex,
            seed: &seed_hex,
            passphrase: &passphrase,
            language: self.lang,
            words: self.words.word_count(),
        })
    }
}

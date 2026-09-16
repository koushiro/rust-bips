use std::{
    fs::File,
    io::{self, IsTerminal, Read},
    path::{Path, PathBuf},
};

use anyhow::{Result, ensure};
use clap::Args;
use zeroize::Zeroizing;

const MAX_INPUT_BYTES: u64 = 64 * 1024;

#[derive(Args)]
pub struct Input {
    /// Read from a UTF-8 file, or '-' for stdin. By default, prompt on a terminal or read piped
    /// stdin.
    #[arg(short = 'i', long, value_name = "PATH")]
    pub input: Option<PathBuf>,
}

impl Input {
    pub fn read(&self, message: &str) -> Result<Zeroizing<String>> {
        match &self.input {
            Some(path) => read_file(path),
            None if io::stdin().is_terminal() => prompt(message),
            None => read_text(io::stdin().lock()),
        }
    }
}

pub(crate) fn prompt(message: &str) -> Result<Zeroizing<String>> {
    let text = Zeroizing::new(rpassword::prompt_password(message)?);
    check_length(&text)?;
    Ok(text)
}

pub(crate) fn read_file(path: &Path) -> Result<Zeroizing<String>> {
    if path == Path::new("-") {
        read_text(io::stdin().lock())
    } else {
        read_text(File::open(path)?)
    }
}

fn read_text(reader: impl Read) -> Result<Zeroizing<String>> {
    let mut text = Zeroizing::new(String::new());
    reader.take(MAX_INPUT_BYTES + 1).read_to_string(&mut text)?;
    check_length(&text)?;
    // Strip one transport line ending, not whitespace that may belong to a passphrase.
    if text.ends_with('\n') {
        text.pop();
        if text.ends_with('\r') {
            text.pop();
        }
    }
    Ok(text)
}

fn check_length(text: &str) -> Result<()> {
    ensure!(text.len() as u64 <= MAX_INPUT_BYTES, "input exceeds the 64 KiB limit");
    Ok(())
}

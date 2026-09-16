pub mod derive;
pub mod generate;
pub mod inspect;

use clap::Subcommand;

use self::{derive::Derive, generate::Generate, inspect::Inspect};

#[derive(Subcommand)]
pub enum Command {
    /// Generate a mnemonic, its entropy, and a BIP39 seed with an optional passphrase.
    #[command(visible_alias = "gen")]
    Generate(Generate),

    /// Validate and describe a mnemonic or extended key.
    #[command(visible_alias = "info")]
    Inspect(Inspect),

    /// Derive extended keys from a hex seed or an extended key.
    Derive(Derive),
}

#![doc = include_str!("../README.md")]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod args;
mod commands;

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use commands::Command;

#[derive(Parser)]
#[command(name = "bli", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Generate(cmd) => cmd.run(),
        Command::Inspect(cmd) => cmd.run(),
        Command::Derive(cmd) => cmd.run(),
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if let Some(error) = error.downcast_ref::<clap::Error>() {
                let _ = error.print();
                ExitCode::from(error.exit_code() as u8)
            } else {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        },
    }
}

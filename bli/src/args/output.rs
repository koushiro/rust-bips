use std::{
    fmt,
    io::{self, Write},
};

use anyhow::{Result, anyhow};
use clap::ValueEnum;
use serde::Serialize;

#[derive(Clone, Copy, ValueEnum)]
pub enum Output {
    Text,
    Json,
}

impl Output {
    pub fn write(self, result: &(impl Serialize + fmt::Display)) -> Result<()> {
        self.write_to(io::stdout().lock(), result)
    }

    fn write_to(
        self,
        mut writer: impl Write,
        result: &(impl Serialize + fmt::Display),
    ) -> Result<()> {
        let outcome: Result<()> = match self {
            Self::Text => write!(writer, "{result}").map_err(Into::into),
            Self::Json => serde_json::to_writer_pretty(&mut writer, result).map_err(|error| {
                match error.io_error_kind() {
                    // serde_json does not expose the wrapped I/O error itself in its chain.
                    Some(kind) => io::Error::new(kind, error).into(),
                    // A serializer's custom error could contain secret output values.
                    None => anyhow!("could not serialize the result"),
                }
            }),
        }
        .and_then(|()| {
            writeln!(writer)?;
            writer.flush()?;
            Ok(())
        });
        match outcome {
            Err(error)
                if error
                    .downcast_ref::<io::Error>()
                    .is_some_and(|error| error.kind() == io::ErrorKind::BrokenPipe) =>
            {
                Ok(())
            },
            outcome => outcome,
        }
    }
}

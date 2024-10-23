//! A CLI for interpreting FKYS script

use {
    anyhow::{Result, bail},
    clap::Parser,
    fkys_rs::eval,
    std::{
        fs::read_to_string,
        io::{BufWriter, Write as _, stdout},
        path::PathBuf,
    },
};

/// Arguments passed to interpreter.
///
/// There is 2 variants: path to script or script itself (with -c option).
#[derive(Parser)]
#[clap(about, version, long_about = None)]
struct Args {
    /// Path to script
    #[clap(value_name = "FILE")]
    path: Option<PathBuf>,

    /// A script to evaluate
    #[clap(short, long, value_name = "COMMAND")]
    command: Option<String>,
}

fn main() -> Result<()> {
    let (args, mut handle) = (Args::parse(), BufWriter::new(stdout()));
    let script = match (args.path, args.command) {
        (Some(_), Some(_)) | (None, None) => bail!("only one option must be specified"),
        (Some(path), None) => read_to_string(path)?,
        (None, Some(command)) => command,
    };

    eval(&script, &mut handle)?;

    handle.flush().unwrap_or_else(|_| println!("Error: no stdout shown"));
    Ok(())
}

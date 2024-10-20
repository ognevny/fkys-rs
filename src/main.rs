//! A CLI for interpreting FKYS script

use {
    anyhow::Result,
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
/// Currently only path to script
#[derive(Parser)]
struct Args {
    /// Path to script
    path: PathBuf,
}

fn main() -> Result<()> {
    let (args, mut handle) = (Args::parse(), BufWriter::new(stdout()));
    let script = read_to_string(args.path)?;

    eval(&script, &mut handle)?;

    handle.flush().unwrap_or_else(|| println!("Error: no stdout shown"));
    Ok(())
}

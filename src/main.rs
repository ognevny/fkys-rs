//! A CLI for interpreting FKYS script

use {
    anyhow::{Context as _, Result, bail},
    clap::Parser,
    fkys_rs::{eval, eval_char},
    std::{
        fs::read_to_string,
        io::{BufWriter, Write as _, stdin, stdout},
        path::PathBuf,
    },
};

/// Run interactive shell
fn interactive_shell() -> Result<()> {
    let (mut handle, mut arr, mut pointer, mut int_mode) =
        (BufWriter::new(stdout().lock()), [0; 500], 0, true);
    loop {
        handle.write_all(b"\n>>> ")?;
        handle.flush()?;
        let mut input = String::with_capacity(1);
        stdin().read_line(&mut input)?;
        let mut input_iter = input.trim().chars();
        if input_iter.clone().count() != 1 {
            handle.write_all(
                b"> Can't evaluate this command, type `h` to get list of commands, `e` to exit",
            )?;
            handle.flush()?;
            continue;
        }
        // SAFETY: size of iterator checked above
        let char = unsafe { input_iter.next().unwrap_unchecked() };
        if char == 'e' {
            break;
        }
        eval_char(char, &mut handle, &mut arr, &mut pointer, &mut int_mode, true)?;
    }

    Ok(())
}

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
    let (args, mut handle) = (Args::parse(), BufWriter::new(stdout().lock()));
    let script = match (args.path, args.command) {
        (Some(_), Some(_)) => bail!("only one option must be specified"),
        (None, None) => return interactive_shell(),
        (Some(path), None) => read_to_string(path).with_context(|| "failed to read script file")?,
        (None, Some(command)) => command,
    };

    eval(&script, &mut handle).with_context(|| "failed ro evaluate script")?;

    handle.flush().with_context(|| "no output shown")?;
    Ok(())
}

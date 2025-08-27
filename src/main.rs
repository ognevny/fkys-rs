//! A CLI for interpreting FKYS script

mod fkys;

use {
    anyhow::{Context as _, Result},
    clap::Parser,
    core::hint::unreachable_unchecked,
    fkys::{eval, eval_char},
    std::{
        fs::read_to_string,
        io::{BufWriter, Write as _, stdin, stdout},
        path::PathBuf,
    },
};

/// Arguments passed to interpreter.
///
/// There is 2 variants: path to script or script itself (with -c option).
#[derive(Parser)]
#[clap(about, version, long_about = None)]
struct Args {
    /// A script to evaluate
    #[clap(short, long, exclusive = true, value_name = "COMMAND")]
    command: Option<String>,

    /// Path to script
    #[clap(exclusive = true, value_name = "FILE")]
    path: Option<PathBuf>,
}

/// Run interactive shell
fn interactive_shell() -> Result<()> {
    let (mut handle, mut arr, mut pointer, mut int_mode) = (BufWriter::new(stdout().lock()), [0; 500], 0, true);
    loop {
        handle.write_all(b"\n>>> ")?;
        handle.flush()?;
        let mut input = String::with_capacity(1);
        match stdin().read_line(&mut input) {
            Ok(0) => return Ok(()),
            Err(_) => {
                handle.write_all(b"> Failed to read input, try again")?;
                handle.flush()?;
                continue;
            },
            _ => (),
        };
        let input = input.trim();
        if input.len() != 1 {
            handle.write_all(b"> Can't evaluate this command, type `h` to get list of commands, `e` to exit")?;
            handle.flush()?;
            continue;
        }
        // SAFETY: size of iterator is checked above
        let char = unsafe { input.chars().next().unwrap_unchecked() };
        if char == 'e' {
            return Ok(());
        }
        eval_char(char, &mut handle, &mut arr, &mut pointer, &mut int_mode, true)?;
    }
}

fn main() -> Result<()> {
    let (args, mut handle) = (Args::parse(), BufWriter::new(stdout().lock()));
    let script = match (args.path, args.command) {
        (None, None) => return interactive_shell(),
        (Some(path), None) => read_to_string(path).context("failed to read script file")?,
        (None, Some(command)) => command,
        // SAFETY: clap handles this case
        _ => unsafe { unreachable_unchecked() },
    };

    eval(&script, &mut handle).context("failed to evaluate script")?;

    handle.flush().context("no output shown")?;
    Ok(())
}

use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (mut arr, mut pointer, mut int_mode) = ([0; 500], 0, true);
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for char in fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_string_lossy()))?
        .chars()
    {
        match char {
            '>' => pointer = (pointer + 1) % 500,
            '<' => {
                if pointer == 0 {
                    pointer = 500;
                }
                pointer -= 1;
            }
            '+' => arr[pointer] += 1,
            '-' => arr[pointer] -= 1,
            'o' => {
                if int_mode {
                    write!(handle, "{}", arr[pointer])?;
                } else if arr[pointer] >= 0 {
                    write!(handle, "{}", arr[pointer] as u8 as char)?;
                }
            }
            'p' => {
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input)?;
                arr[pointer] = user_input.trim_end().parse()?;
            }
            'n' => writeln!(handle)?,
            'i' => int_mode = true,
            'c' => int_mode = false,
            _ => (),
        }
    }
    Ok(())
}

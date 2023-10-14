use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    io::{self, Write},
};
static mut ARRAY: [i32; 500] = [0; 500];
static mut POINTER: usize = 0;
static mut INT_MODE: bool = true;

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut handle = io::BufWriter::new(io::stdout());

    let (mut code, mut collecting_code) = (String::new(), false);
    for char in fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_string_lossy()))?
        .chars()
    {
        match char {
            'e' => std::process::exit(0),
            '[' => collecting_code = true,
            ']' => unsafe {
                collecting_code = false;
                loop {
                    for char in code.chars() {
                        eval(char, &mut handle)?;
                    }
                    if ARRAY[POINTER] == 0 {
                        break;
                    }
                }
                code.clear();
            },
            _ => (),
        }

        if !collecting_code {
            unsafe { eval(char, &mut handle) }?;
        } else {
            code.push(char);
        }
    }
    Ok(())
}

// SAFETY: this is a single thread synced operation, so it's impossible to access the same data
// twice at the same time
unsafe fn eval<W: Write>(char: char, handle: &mut io::BufWriter<W>) -> Result<()> {
    match char {
        '>' => POINTER = (POINTER + 1) % 500,
        '<' => {
            if POINTER == 0 {
                POINTER = 500;
            }
            POINTER -= 1;
        }
        '+' => ARRAY[POINTER] += 1,
        '-' => ARRAY[POINTER] -= 1,
        'o' => {
            if INT_MODE {
                write!(*handle, "{}", ARRAY[POINTER])?;
            } else if ARRAY[POINTER] >= 0 {
                write!(*handle, "{}", ARRAY[POINTER] as u8 as char)?;
            }
        }
        'p' => {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)?;
            ARRAY[POINTER] = user_input.trim_end().parse()?;
        }
        'n' => writeln!(*handle)?,
        's' => write!(*handle, " ")?,
        'l' => ARRAY[POINTER] = 125,
        'i' => INT_MODE = true,
        'c' => INT_MODE = false,
        _ => (),
    }
    Ok(())
}

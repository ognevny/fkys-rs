use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader, BufWriter},
};

// SAFETY: all of this is a single thread operation, so it's impossible to access the same data
// twice at the same time
static mut ARRAY: [i32; 500] = [0; 500];
static mut POINTER: usize = 0;
static mut INT_MODE: bool = true;

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let (args, mut handle, mut code, mut collecting) = (
        Args::parse(),
        BufWriter::new(io::stdout()),
        String::new(),
        false,
    );
    for line in BufReader::new(
        File::open(&args.path)
            .with_context(|| format!("Could not read file `{}`", &args.path.to_string_lossy()))?,
    )
    .lines()
    {
        for char in line?.chars() {
            match char {
                '#' => break,
                'e' => std::process::exit(0),
                '[' => collecting = true,
                ']' => unsafe {
                    collecting = false;
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

            if !collecting {
                eval(char, &mut handle)?;
            } else {
                code.push(char);
            }
        }
    }
    handle.flush()?;
    Ok(())
}

fn eval<W: ?Sized + Write>(char: char, handle: &mut W) -> Result<()> {
    match char {
        '>' => unsafe { POINTER = (POINTER + 1) % 500 },
        '<' => unsafe {
            if POINTER == 0 {
                POINTER = 500;
            }
            POINTER -= 1;
        },
        '+' => unsafe { ARRAY[POINTER] += 1 },
        '-' => unsafe { ARRAY[POINTER] -= 1 },
        'o' => unsafe {
            if INT_MODE {
                write!(*handle, "{}", ARRAY[POINTER])?;
            } else if ARRAY[POINTER] >= 0 {
                write!(
                    *handle,
                    "{}",
                    char::from_u32(ARRAY[POINTER] as u32).unwrap()
                )?;
            }
        },
        'p' => unsafe {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)?;
            ARRAY[POINTER] = user_input.trim_end().parse()?;
        },
        'n' => handle.write_all(b"\n")?,
        's' => handle.write_all(b" ")?,
        'l' => unsafe { ARRAY[POINTER] = 125 },
        'i' => unsafe { INT_MODE = true },
        'c' => unsafe { INT_MODE = false },
        _ => (),
    }
    Ok(())
}

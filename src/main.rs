use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    io::{self, Write},
    sync::Mutex,
};

lazy_static::lazy_static! {
    static ref ARRAY: Mutex<[i32; 500]> = Mutex::new([0; 500]);
    static ref POINTER: Mutex<usize> = Mutex::new(0);
    static ref INT_MODE: Mutex<bool> = Mutex::new(true);
}

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
        if char == '[' {
            collecting_code = true
        } else if char == ']' {
            collecting_code = false;
            loop {
                for char in code.chars() {
                    eval(char, &handle)?;
                }
                if ARRAY.lock().unwrap()[*POINTER.lock().unwrap()] == 0 {
                    break;
                }
            }
            code.clear();
        }

        if !collecting_code {
            eval(char, &handle)?;
        } else {
            code.push(char);
        }
    }
    Ok(())
}

fn eval(char: char, handle: &io::BufWriter<W>) -> Result<()> {
    let (mut arr, mut pointer, mut int_mode) = (
        ARRAY.lock().unwrap(),
        POINTER.lock().unwrap(),
        INT_MODE.lock().unwrap(),
    );
    match char {
        '>' => *pointer = (*pointer + 1) % 500,
        '<' => {
            if *pointer == 0 {
                *pointer = 500;
            }
            *pointer -= 1;
        }
        '+' => arr[*pointer] += 1,
        '-' => arr[*pointer] -= 1,
        'o' => {
            if *int_mode {
                write!(*handle, "{}", arr[*pointer])?;
            } else if arr[*pointer] >= 0 {
                write!(*handle, "{}", arr[*pointer] as u8 as char)?;
            }
        }
        'p' => {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)?;
            arr[*pointer] = user_input.trim_end().parse()?;
        }
        'n' => writeln!(*handle)?,
        's' => write!(*handle, " ")?,
        'i' => *int_mode = true,
        'c' => *int_mode = false,
        _ => (),
    }
    Ok(())
}

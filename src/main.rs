//! For now only one file is allowed - main.fkys.

use std::{io, fs};

fn main() -> io::Result<()> {
    let mut arr = [0; 500];
    let mut pointer = 0;
    let mut int_mode = true;
    for char in fs::read_to_string("main.fkys")?.chars() {
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
                    print!("{}", arr[pointer]);
                } else if arr[pointer] >= 0 {
                    print!("{}", arr[pointer] as u8 as char)
                }
            }
            'p' => {
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input)?;
                arr[pointer] = user_input.trim_end().parse().unwrap();
            }
            'n' => println!(),
            'i' => int_mode = true,
            'c' => int_mode = false,
            _ => (),
        }
    }
    print!("\nProgram ended.");
    Ok(())
}

//! A FKYS library for parsing scripts

use {
    anyhow::Result,
    std::io::{Write, stdin},
};

/// Evaluate a script.
///
/// # Errors
///
/// Fails if `eval_char` returns `Err()`
pub fn eval<W: ?Sized + Write>(script: &str, handle: &mut W) -> Result<()> {
    let (mut code, mut collecting, mut arr, mut pointer, mut int_mode) =
        (String::new(), false, [0; 500], 0, true);
    for line in script.lines() {
        for char in line.chars() {
            match char {
                '#' => break,
                'e' => return Ok(()),
                '[' => collecting = true,
                ']' => {
                    collecting = false;
                    loop {
                        for char_code in code.chars() {
                            eval_char(char_code, handle, &mut arr, &mut pointer, &mut int_mode)?;
                        }
                        if arr[pointer] == 0 {
                            break;
                        }
                    }
                    code.clear();
                },
                _ => (),
            }

            if collecting {
                code.push(char);
            } else {
                eval_char(char, handle, &mut arr, &mut pointer, &mut int_mode)?;
            }
        }
    }
    Ok(())
}

/// Evluate a char
fn eval_char<W: ?Sized + Write>(
    char: char,
    handle: &mut W,
    arr: &mut [i32; 500],
    pointer: &mut usize,
    int_mode: &mut bool,
) -> Result<()> {
    match char {
        '>' => *pointer = (*pointer + 1) % 500,
        '<' => {
            if *pointer == 0 {
                *pointer = 500;
            }
            *pointer -= 1;
        },
        '+' => arr[*pointer] += 1,
        '-' => arr[*pointer] -= 1,
        'o' =>
            if *int_mode {
                write!(*handle, "{}", arr[*pointer])?;
            } else {
                write!(*handle, "{}", char::from_u32(arr[*pointer].unsigned_abs()).unwrap())?;
            },
        'p' => {
            let mut user_input = String::new();
            stdin().read_line(&mut user_input)?;
            arr[*pointer] = user_input.trim_end().parse()?;
        },
        'n' => handle.write_all(b"\n")?,
        's' => handle.write_all(b" ")?,
        'l' => arr[*pointer] = 125,
        'i' => *int_mode = true,
        'c' => *int_mode = false,
        _ => (),
    }
    Ok(())
}

//! A FKYS library for parsing scripts

use {
    anyhow::Result,
    std::io::{Write, stdin},
};

/// Evaluate a script.
///
/// A string slice and a handle is passed. Handle must implement [`std::io::Write`] trait. It can be
/// a [`Stdout`].
///
/// # Errors
///
/// Fails if there is a error while evaluating a single char.
///
/// ```no_run
/// use {
///     fkys_rs::eval,
///     std::io::{BufWriter, stdout},
/// };
///
/// let mut handle = BufWriter::new(stdout());
/// eval(
///     r"++++++++++++++++++++++++++++++++++++++++++++++os # number bam-bam
/// ---------------os+++++++++++++o",
///     &mut handle,
/// ); // writes `46 31 44` into handle
/// ```
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
                            eval_char(
                                char_code,
                                handle,
                                &mut arr,
                                &mut pointer,
                                &mut int_mode,
                                false,
                            )?;
                        }
                        // SAFETY: pointer is in range 0..=499
                        if *unsafe { arr.get_unchecked(pointer) } == 0 {
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
                eval_char(char, handle, &mut arr, &mut pointer, &mut int_mode, false)?;
            }
        }
    }
    Ok(())
}

/// Evluate a char.
///
/// Provides a low-level evaluation, which require
///
/// - char to evaluate ([`char`])
/// - handle, which implements [`std::io::Write`],
/// - array which is used to write things (fixed at [`i32`; 500])
/// - pointer, which must be at range `0..=499`
/// - int mode, which defines whether integers or chars are printed to stdout
/// - is interactive, which enables interactive shell mode (example of such is implemented in
///   `fkysoxide` binary)
///
/// # Errors
///
/// Function uses some methods that return [`Result`] value
///
/// # Panics
///
/// Panics when conversion from [`u32`] to [`char`] fails
///
/// [`Result`]: anyhow::Result
pub(crate) fn eval_char<W: ?Sized + Write>(
    char: char,
    handle: &mut W,
    arr: &mut [i32; 500],
    pointer: &mut usize,
    int_mode: &mut bool,
    is_interactive: bool,
) -> Result<()> {
    match char {
        '>' => {
            // SAFETY: pointer is much less than usize::MAX
            *pointer = unsafe { pointer.unchecked_add(1) % 500 };
            if is_interactive {
                write!(*handle, "> Now at {}", *pointer)?;
            }
        },
        '<' => {
            // SAFETY: pointer is much less than usize::MAX
            *pointer = unsafe { pointer.unchecked_add(499) % 500 };
            if is_interactive {
                write!(*handle, "> Now at {}", *pointer)?;
            }
        },
        '+' => {
            // SAFETY: pointer is in range of 0..=499
            unsafe {
                *arr.get_unchecked_mut(*pointer) += 1;
            }
            if is_interactive {
                // SAFETY: pointer is in range of 0..=499
                unsafe {
                    write!(*handle, "> {}", arr.get_unchecked(*pointer))?;
                }
            }
        },
        '-' => {
            // SAFETY: pointer is in range of 0..=499
            unsafe {
                *arr.get_unchecked_mut(*pointer) -= 1;
            }
            if is_interactive {
                // SAFETY: pointer is in range of 0..=499
                unsafe {
                    write!(*handle, "> {}", arr.get_unchecked(*pointer))?;
                }
            }
        },
        'o' => {
            if *int_mode {
                // SAFETY: pointer is in range of 0..=499
                unsafe {
                    write!(*handle, "{}", arr.get_unchecked(*pointer))?;
                }
            } else {
                // SAFETY: pointer is in range of 0..=499
                unsafe {
                    write!(
                        *handle,
                        "{}",
                        char::from_u32(arr.get_unchecked(*pointer).unsigned_abs())
                            .unwrap_or_default()
                    )?;
                }
            }
        },
        'p' => {
            let mut user_input = String::with_capacity(11);
            loop {
                match stdin().read_line(&mut user_input) {
                    Ok(0) => return Ok(()),
                    Err(_) => {
                        handle.write_all(b"> Failed to read input, try again")?;
                        handle.flush()?;
                        continue;
                    },
                    _ => break,
                };
            }
            // SAFETY: pointer is in range of 0..=499
            unsafe {
                *arr.get_unchecked_mut(*pointer) = user_input.trim_end().parse()?;
            }
        },
        'n' if !is_interactive => handle.write_all(b"\n")?,
        's' if !is_interactive => handle.write_all(b" ")?,
        'l' => {
            // SAFETY: pointer is in range of 0..=499
            unsafe {
                *arr.get_unchecked_mut(*pointer) = 125;
            }
            if is_interactive {
                handle.write_all(b"> 125")?;
            }
        },
        'i' => {
            *int_mode = true;
            if is_interactive {
                handle.write_all(b"> Int mode enabled")?;
            }
        },
        'c' => {
            *int_mode = false;
            if is_interactive {
                handle.write_all(b"> Int mode disabled")?;
            }
        },
        'h' if is_interactive => handle.write_all(
            b"> Available commands:

e - exit interactive shell
> - moves pointer right
< - moves pointer left
+ - increments cell
- - decrements cell
i - integer output mode (enabled by default)
c - character output mode
o - prints the contents of the cell to the console
p - accepts input from the user into the cell
l - sets cell value to 125
# - comments the rest of line
h - prints this message",
        )?,
        _ =>
            if is_interactive {
                handle.write_all(
                    b"> Unknown command, type `h` to get list of commands, `e` to exit",
                )?;
            },
    }
    Ok(())
}

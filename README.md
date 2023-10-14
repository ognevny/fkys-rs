# FKYSoxide
this is a [F*cking Kill Yourself lang](https://github.com/eleoelo/fkys) interpretter, independently
rewritten in Rust.

## Syntax
```
> - moves pointer right
< - moves pointer left
+ - increments cell
- - decrements cell
i - integer output mode (enabled by default)
c - character output mode
n - newline
o - prints the contents of the cell to the console
p - accepts input from the user into the cell 
```
tabs, spaces and other symbols are just ignored (but commenting isn't supported now)

## Installing
Simply run `cargo install --locked fkys-rs` in your terminal.

## Usage
To run script, just pass `fkysoxide <path-to-file>`.

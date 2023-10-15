<p align="center">
  <a title="GitHub Actions" href="https://github.com/ognevnydemon/fkys-rs/actions"><img alt="workflow Status" src="https://img.shields.io/github/actions/workflow/status/ognevnydemon/fkys-rs/ci.yml?branch=master&longCache=true&style=flat-square&label=build&logo=github"></a><!--
  -->
  <a title="Crate" href="https://crates.io/crates/fkys-rs"><img src="https://img.shields.io/crates/v/fkys-rs.svg?style=flat-square"></a><!--
  -->
  <a title="WTFPL license" href="https://github.com/ognevnydemon/fkys-rs/blob/master/LICENSE"><img src="https://img.shields.io/badge/License-WTFPL-red.svg?style=flat-square"></a><!--
  -->
</p>

# FKYSoxide
this is a [F*cking Kill Yourself lang](https://github.com/eleoelo/fkys) interpretter, independently
rewritten in Rust.

## Syntax
```
e - exit programm
> - moves pointer right
< - moves pointer left
+ - increments cell
- - decrements cell
i - integer output mode (enabled by default)
c - character output mode
n - inserts newline
s - inserts space
o - prints the contents of the cell to the console
p - accepts input from the user into the cell
l - sets cell value to 125
[] - loop (runs while the cell != 0)
# - comments the rest of line
```
tabs, spaces and other symbols are just ignored

## Installing
Simply run `cargo install --locked fkys-rs` in your terminal.

## Usage
To run script, just pass `fkysoxide <path-to-script>`.

## Code examples
Use `+` and `-` to increment or decrement the value in the current cell respectively.

This example makes the value of the cell equal to 3.
```
++++-
```

Use `l` to set the cell's value to 125
```
l---
```
Now the value of the cell at 0 is equal to 122.

Use `o` to output the current cell's value, use `p` to put the number from the stdin.
```
p---o
```
If we put 20, the code will output 17. Also use `s` to output space, and use `n` to go to the next
line.

What if you want to output a text? You can do that. Use `c` to go into the text output mode, and use
`i` to go back to the number output mode. When you are in the text output mode, numbers are
converted to characters using the ASCII table. For example, the following code outputs `Hello,
world!`
```
cl-----------------------------------------------------o
>l------------------------o+++++++oo+++o>+++++++++++
+++++++++++++++++++++++++++++++++os>l------o<<o+++o------o----
----o>-----------o
```
But if you remove the `c` at the beginning, the code will output this:
```
7210110810811144 11911111410810033
```

Loops repeat until the current cell's value doesn't equal to 0. For example, this sample outputs the
whole ASCII table.
```
l[oscoin-]
```

You can comment the code line using `#`
```
++++ # 4
--- # 1
```

Use `e` to exit programm (code 0)

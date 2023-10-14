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
> - moves pointer right
< - moves pointer left
+ - increments cell
- - decrements cell
i - integer output mode (enabled by default)
c - character output mode
n - newline
o - prints the contents of the cell to the console
p - accepts input from the user into the cell
[] - loop (runs while the cell != 0)
```
tabs, spaces and other symbols are just ignored (but commenting isn't supported now)

## Installing
Simply run `cargo install --locked fkys-rs` in your terminal.

## Usage
To run script, just pass `fkysoxide <path-to-file>`.

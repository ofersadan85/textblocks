# TextBlocks
<!-- BADGES -->
[![Rust Release](https://github.com/ofersadan85/textblocks/actions/workflows/rust.yml/badge.svg)](https://github.com/ofersadan85/textblocks/actions/workflows/rust.yml)
[![Dependency status](https://deps.rs/repo/github/ofersadan85/textblocks/status.svg)](https://deps.rs/repo/github/ofersadan85/textblocks)
[![crates.io](https://img.shields.io/crates/v/textblocks.svg)](https://crates.io/crates/textblocks)
[![Downloads crates.io](https://img.shields.io/crates/d/textblocks.svg?label=crates.io%20downloads)](https://crates.io/crates/textblocks)

<!-- cargo-rdme start -->

A simple crate for parsing text blocks.
Can be used to parse text files with blocks of data separated by blank lines.
Works well with \n or \r\n line endings.

Contains the `TextBlocks` trait which adds the methods `as_blocks`, `block_parse_lines` and `block_parse` to `str` and `String`.

## Install

Run the following command in your project directory:

```bash
cargo add textblocks
```

Or add the following to your `Cargo.toml`:

```toml
[dependencies]
textblocks = "0.1.0"
```

Check the [crates.io](https://crates.io/crates/textblocks) page for the latest version.

## Usage

To parse text into blocks, you need to provide a block delimiter, a line parser and a block parser.

- The *block delimiter* is a string that separates blocks. The default is a blank line (double newline), but you can use any string.
  - `BlockDelimiter::DoubleLineGeneric` (the default) will use `"\r\n\r\n"` if the string contains `"\r\n"` newlines, otherwise `"\n\n"`.
  - `BlockDelimiter::Delimiter(s)` will use `s` (a `String`) as the delimiter.
- The *line parser* is any function or closure that takes a `&str` and returns a value of type `T`. The final result will be a `Vec<Vec<T>>`.
You can use the `block_parse_lines` method if you don't need a block parser and only want to parse the lines.
- The *block parser* is any function or closure that takes a `&[T]` and returns a value of type `U`. The final result will be a `Vec<U>`.

## Examples

- Parse a block into a vector of lines

> [!IMPORTANT]
> This will allocate a vector of vectors of `&str`. If you want to avoid these allocations, use `block_parse_lines` or `block_parse`.
> In that case, A vector will only be allocated for the requested result type.

```rust
use textblocks::*;
let s = "100\n200\n\n300\n400\n\n500\n600";
let block_delimiter = BlockDelimiter::DoubleLineGeneric;
assert_eq!(s.as_blocks(&block_delimiter), vec![vec!["100", "200"], vec!["300", "400"], vec!["500", "600"]]);
assert_eq!(s.as_blocks(&block_delimiter), [["100", "200"], ["300", "400"], ["500", "600"]]);
```

- Parse a block into a vector of lines, where each line is parsed into a number (u32)

```rust
use textblocks::*;
let s = "100\n200\n\n300\n400\n\n500\n600";
let block_delimiter = BlockDelimiter::DoubleLineGeneric;
let result = s.block_parse_lines(&block_delimiter,|line| line.parse::<u32>().unwrap());
assert_eq!(result, [[100, 200], [300, 400], [500, 600]]);
```

- Parse a block into a vector of lines, where each line is parsed into a number (u32), and then sum the numbers

```rust
use textblocks::*;
let s = "100\n200\n\n300\n400\n\n500\n600";
let block_delimiter = BlockDelimiter::DoubleLineGeneric;
let result = s.block_parse(
    &block_delimiter,
    |line| line.parse::<u32>().unwrap(),
    |block| block.iter().sum::<u32>()
);
assert_eq!(result, [300, 700, 1100]);
```

<!-- cargo-rdme end -->

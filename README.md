<!-- BADGES -->

<!-- cargo-rdme start -->

# TextBlocks

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

> Note: Check the [crates.io](https://crates.io/crates/textblocks) page for the latest version.

## Examples

- Parse a block into a vector of lines

```rust
use textblocks::*;
let s = "100\n200\n\n300\n400\n\n500\n600";
assert_eq!(s.as_blocks(), vec![vec!["100", "200"], vec!["300", "400"], vec!["500", "600"]]);
assert_eq!(s.as_blocks(), [["100", "200"], ["300", "400"], ["500", "600"]]);
```

- Parse a block into a vector of lines, where each line is parsed into a number (u32)

```rust
use textblocks::*;
let s = "100\n200\n\n300\n400\n\n500\n600";
let result = s.block_parse_lines(|line| line.parse::<u32>().unwrap());
assert_eq!(result, [[100, 200], [300, 400], [500, 600]]);
```

- Parse a block into a vector of lines, where each line is parsed into a number (u32), and then sum the numbers

```rust
use textblocks::*;
let s = "100\n200\n\n300\n400\n\n500\n600";
let result = s.block_parse(
    |line| line.parse::<u32>().unwrap(),
    |block| block.iter().sum::<u32>()
);
assert_eq!(result, [300, 700, 1100]);
```

<!-- cargo-rdme end -->

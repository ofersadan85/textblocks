//! # TextBlocks
//!
//! A simple crate for parsing text blocks.
//! Can be used to parse text files with blocks of data separated by blank lines.
//! Works well with \n or \r\n line endings.
//!
//! Contains the `TextBlocks` trait which adds the methods `as_blocks`, `block_parse_lines` and `block_parse` to `str` and `String`.
//!
//! ## Install
//!
//! Run the following command in your project directory:
//!
//! ```bash
//! cargo add textblocks
//! ```
//!
//! Or add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! textblocks = "0.1.0"
//! ```
//!
//! > Note: Check the [crates.io](https://crates.io/crates/textblocks) page for the latest version.
//!
//! ## Examples
//!
//! - Parse a block into a vector of lines
//!
//! ```rust
//! use textblocks::*;
//! let s = "100\n200\n\n300\n400\n\n500\n600";
//! assert_eq!(s.as_blocks(), vec![vec!["100", "200"], vec!["300", "400"], vec!["500", "600"]]);
//! assert_eq!(s.as_blocks(), [["100", "200"], ["300", "400"], ["500", "600"]]);
//! ```
//!
//! - Parse a block into a vector of lines, where each line is parsed into a number (u32)
//!
//! ```rust
//! use textblocks::*;
//! let s = "100\n200\n\n300\n400\n\n500\n600";
//! let result = s.block_parse_lines(|line| line.parse::<u32>().unwrap());
//! assert_eq!(result, [[100, 200], [300, 400], [500, 600]]);
//! ```
//!
//! - Parse a block into a vector of lines, where each line is parsed into a number (u32), and then sum the numbers
//!
//! ```rust
//! use textblocks::*;
//! let s = "100\n200\n\n300\n400\n\n500\n600";
//! let result = s.block_parse(
//!     |line| line.parse::<u32>().unwrap(),
//!     |block| block.iter().sum::<u32>()
//! );
//! assert_eq!(result, [300, 700, 1100]);
//! ```

pub trait TextBlocks: AsRef<str> + Sized
where
    Self: AsRef<str> + Sized,
{
    /// Parse a string into blocks, where a block is a vector of lines.
    /// Blocks are separated by a blank line. Works well with \n or \r\n line endings.
    ///
    /// # Example
    /// ```rust
    /// use textblocks::*;
    /// let s = "100\n200\n\n300\n400\n\n500\n600";
    /// assert_eq!(s.as_blocks(), vec![vec!["100", "200"], vec!["300", "400"], vec!["500", "600"]]);
    /// ```
    fn as_blocks(&self) -> Vec<Vec<&str>> {
        let s = self.as_ref();
        if s.is_empty() {
            return vec![];
        }
        let delimiter = if s.contains('\r') { "\r\n" } else { "\n" };
        let double = delimiter.to_owned() + delimiter;
        s.trim()
            .split(&double)
            .map(|x| x.trim().split(delimiter).collect())
            .collect()
    }

    /// Parse a block into a vector of lines, where each line is parsed into a type T, using the provided line parser.
    /// If some lines cannot be parsed, make sure to use a type that can handle that (e.g. `Option<T>` or `Result<T, E>`)
    /// and then use `filter_map` to remove the lines that could not be parsed.
    ///
    /// # Example
    /// ```rust
    /// use textblocks::*;
    /// let s = "100\n200\n\n300\n400\n\n500\n600";
    /// let result = s.block_parse_lines(|line| line.parse::<u32>().unwrap());
    /// assert_eq!(result, vec![vec![100, 200], vec![300, 400], vec![500, 600]]);
    /// ```
    ///
    /// # Panics
    /// Will panic if the line parser function panics.
    fn block_parse_lines<INNER, LP>(&self, line_parser: LP) -> Vec<Vec<INNER>>
    where
        LP: Fn(&str) -> INNER,
    {
        self.as_blocks()
            .iter()
            .map(|block| block.iter().map(|line| line_parser(line)).collect())
            .collect()
    }

    /// Parse a block using the provided block parser. Blocks may be reduced to a single value, or parsed into a vector,
    /// using the provided block parser. Similar to `parse_lines`, if some blocks cannot be parsed, make sure to use a type
    /// that can handle that (e.g. `Option<T>` or `Result<T, E>`) and then use `filter_map` to remove the blocks that could not be parsed.
    ///
    /// # Example
    /// ```rust
    /// use textblocks::*;
    /// let s = "abcde\nwow\n\n11111\n22222\n33333";
    /// let result = s.block_parse(
    ///    |line| line.chars().next().unwrap(),
    ///    |block| block.iter().collect::<String>(),
    /// );
    /// assert_eq!(result, vec!["aw", "123"]);
    /// ```
    /// # Panics
    /// Will panic if the line parser function or the block parser function panics.
    fn block_parse<INNER, BLOCK, LP, BP>(&self, line_parser: LP, block_parser: BP) -> Vec<BLOCK>
    where
        LP: Fn(&str) -> INNER,
        BP: Fn(Vec<INNER>) -> BLOCK,
    {
        self.as_blocks()
            .iter()
            .map(|block| block.iter().map(|line| line_parser(line)).collect())
            .map(block_parser)
            .collect()
    }
}

impl<T> TextBlocks for T where T: AsRef<str> + Sized {}

#[cfg(test)]
mod tests {
    use super::*;
    const INT_EXAMPLE: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_block_split() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".as_blocks();
        let expected = vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"],
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn test_block_split_crlf() {
        let s =
            "abc\r\n\r\na\r\nb\r\nc\r\n\r\nab\r\nac\r\n\r\na\r\na\r\na\r\na\r\n\r\nb".as_blocks();
        let expected = vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"],
        ];
        assert_eq!(s, expected);
    }

    #[test]
    fn test_block_split_empty() {
        let expected: Vec<Vec<&str>> = vec![];
        assert_eq!(String::new().as_blocks(), expected);
        assert_eq!("".as_blocks(), expected);
    }

    #[test]
    fn test_block_split_single() {
        assert_eq!("abc".as_blocks(), [["abc"]]);
    }

    #[test]
    fn test_block_split_single_with_newline() {
        assert_eq!("abc\n".as_blocks(), [["abc"]]);
    }

    #[test]
    fn test_block_split_single_with_newline_and_empty() {
        assert_eq!("abc\n\n".as_blocks(), [["abc"]]);
    }

    #[test]
    fn test_parse_lines_int() {
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let parsed = INT_EXAMPLE.block_parse_lines(|x| x.parse::<u32>().unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_lines_empty() {
        let expected: Vec<Vec<u32>> = vec![];
        let parsed = String::new().block_parse_lines(|x| x.parse::<u32>().unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_blocks_empty() {
        let expected: Vec<Vec<u32>> = vec![];
        let parsed = "".block_parse(|x| x.parse::<u32>().unwrap(), |x| x);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_blocks_non_reduced() {
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let parsed = INT_EXAMPLE.block_parse(|x| x.parse::<u32>().unwrap(), |x| x);
        assert_eq!(parsed, expected);
        let parsed = INT_EXAMPLE.block_parse(
            |x| x.parse::<u32>().unwrap(),
            |x| x.iter().rev().copied().collect::<Vec<u32>>(),
        );
        assert_eq!(
            parsed,
            expected
                .iter()
                .map(|x| x.iter().rev().copied().collect())
                .collect::<Vec<Vec<u32>>>()
        );
        let expected = vec![
            vec![3000, 2000, 1000],
            vec![4000],
            vec![6000, 5000],
            vec![9000, 8000, 7000],
            vec![10000],
        ];
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_blocks_reduced() {
        let expected = vec![2000, 0, 1000, 2000, 0];
        let parsed = INT_EXAMPLE.block_parse(
            |x| x.parse::<u32>().unwrap(),
            |x| x.iter().max().unwrap() - x.iter().min().unwrap(),
        );
        assert_eq!(parsed, expected);
    }
}

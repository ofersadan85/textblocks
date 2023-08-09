//! A simple crate for parsing text blocks.
//! Can be used to parse text files with blocks of data separated by blank lines.
//! Works well with \n or \r\n line endings.
//!
//! Contains the `TextBlocks` trait which adds the methods `as_blocks`, `block_parse_lines` and `block_parse` to `str` and `String`.
//!
//! # Install
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
//! Check the [crates.io](https://crates.io/crates/textblocks) page for the latest version.
//!
//! # Usage
//!
//! To parse text into blocks, you need to provide a block delimiter, a line parser and a block parser.
//!
//! - The *block delimiter* is a string that separates blocks. The default is a blank line (double newline), but you can use any string.
//!   - `BlockDelimiter::DoubleLineGeneric` (the default) will use `"\r\n\r\n"` if the string contains `"\r\n"` newlines, otherwise `"\n\n"`.
//!   - `BlockDelimiter::Delimiter(s)` will use `s` (a `String`) as the delimiter.
//! - The *line parser* is any function or closure that takes a `&str` and returns a value of type `T`. The final result will be a `Vec<Vec<T>>`.
//! You can use the `block_parse_lines` method if you don't need a block parser and only want to parse the lines.
//! - The *block parser* is any function or closure that takes a `&[T]` and returns a value of type `U`. The final result will be a `Vec<U>`.
//!
//! # Examples
//!
//! - Parse a block into a vector of lines
//!
//! > [!IMPORTANT]
//! > This will allocate a vector of vectors of `&str`. If you want to avoid these allocations, use `block_parse_lines` or `block_parse`.
//! > In that case, A vector will only be allocated for the requested result type.
//!
//! ```rust
//! use textblocks::*;
//! let s = "100\n200\n\n300\n400\n\n500\n600";
//! let block_delimiter = BlockDelimiter::DoubleLineGeneric;
//! assert_eq!(s.as_blocks(&block_delimiter), vec![vec!["100", "200"], vec!["300", "400"], vec!["500", "600"]]);
//! assert_eq!(s.as_blocks(&block_delimiter), [["100", "200"], ["300", "400"], ["500", "600"]]);
//! ```
//!
//! - Parse a block into a vector of lines, where each line is parsed into a number (u32)
//!
//! ```rust
//! use textblocks::*;
//! let s = "100\n200\n\n300\n400\n\n500\n600";
//! let block_delimiter = BlockDelimiter::DoubleLineGeneric;
//! let result = s.block_parse_lines(&block_delimiter,|line| line.parse::<u32>().unwrap());
//! assert_eq!(result, [[100, 200], [300, 400], [500, 600]]);
//! ```
//!
//! - Parse a block into a vector of lines, where each line is parsed into a number (u32), and then sum the numbers
//!
//! ```rust
//! use textblocks::*;
//! let s = "100\n200\n\n300\n400\n\n500\n600";
//! let block_delimiter = BlockDelimiter::DoubleLineGeneric;
//! let result = s.block_parse(
//!     &block_delimiter,
//!     |line| line.parse::<u32>().unwrap(),
//!     |block| block.iter().sum::<u32>()
//! );
//! assert_eq!(result, [300, 700, 1100]);
//! ```

/// A block delimiter.
/// Can be a generic double line (the default), a delimiter string, or a regex pattern.
/// If the delimiter is a double line, it will be "\r\n\r\n" if the string contains "\r\n", otherwise "\n\n".
/// If the delimiter is a string, it will be used as is.
#[derive(Default)]
pub enum BlockDelimiter {
    /// A double line delimiter, "\r\n\r\n" if the string contains "\r\n", otherwise "\n\n".
    #[default]
    DoubleLineGeneric,
    /// A custom delimiter string.
    Delimiter(String),
    /// A regex pattern. Not implemented yet.
    Pattern(String),
}

fn delimiters(crlf: bool, block_delimiter: &BlockDelimiter) -> (String, String) {
    let line_delimiter = if crlf { "\r\n" } else { "\n" }.to_owned();
    let block_delimiter = match (block_delimiter, crlf) {
        (BlockDelimiter::Pattern(_), _) => todo!("Pattern / Regex not implemented yet"),
        (BlockDelimiter::DoubleLineGeneric, true) => "\r\n\r\n".to_owned(),
        (BlockDelimiter::DoubleLineGeneric, false) => "\n\n".to_owned(),
        (BlockDelimiter::Delimiter(d), _) => d.clone(),
    };
    (line_delimiter, block_delimiter)
}

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
    /// let block_delimiter = BlockDelimiter::DoubleLineGeneric;
    /// assert_eq!(s.as_blocks(&block_delimiter), vec![vec!["100", "200"], vec!["300", "400"], vec!["500", "600"]]);
    /// ```
    fn as_blocks(&self, block_delimiter: &BlockDelimiter) -> Vec<Vec<&str>> {
        let s = self.as_ref();
        let (line_delimiter, block_delimiter) = delimiters(s.contains('\r'), block_delimiter);
        if s.is_empty() {
            return vec![];
        }
        s.trim()
            .split(&block_delimiter)
            .map(|x| x.trim().split(&line_delimiter).collect())
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
    /// let block_delimiter = BlockDelimiter::DoubleLineGeneric;
    /// let result = s.block_parse_lines(&block_delimiter, |line| line.parse::<u32>().unwrap());
    /// assert_eq!(result, vec![vec![100, 200], vec![300, 400], vec![500, 600]]);
    /// ```
    fn block_parse_lines<INNER, LP>(
        &self,
        block_delimiter: &BlockDelimiter,
        line_parser: LP,
    ) -> Vec<Vec<INNER>>
    where
        LP: Fn(&str) -> INNER,
    {
        let s = self.as_ref();
        let (line_delimiter, block_delimiter) = delimiters(s.contains('\r'), block_delimiter);
        if s.is_empty() {
            return vec![];
        }
        #[allow(clippy::redundant_closure)]
        // The line_parser function cannot be used as it doesn't implement Copy
        s.trim()
            .split(&block_delimiter)
            .map(|x| {
                x.trim()
                    .split(&line_delimiter)
                    .map(|line| line_parser(line))
                    .collect()
            })
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
    /// let block_delimiter = BlockDelimiter::DoubleLineGeneric;
    /// let result = s.block_parse(
    ///    &block_delimiter,
    ///    |line| line.chars().next().unwrap(),
    ///    |block| block.iter().collect::<String>(),
    /// );
    /// assert_eq!(result, vec!["aw", "123"]);
    /// ```
    fn block_parse<INNER, BLOCK, LP, BP>(
        &self,
        block_delimiter: &BlockDelimiter,
        line_parser: LP,
        block_parser: BP,
    ) -> Vec<BLOCK>
    where
        LP: Fn(&str) -> INNER,
        BP: Fn(Vec<INNER>) -> BLOCK,
    {
        let s = self.as_ref();
        let (line_delimiter, block_delimiter) = delimiters(s.contains('\r'), block_delimiter);
        if s.is_empty() {
            return vec![];
        }
        #[allow(clippy::redundant_closure)]
        // The line_parser function cannot be used as it doesn't implement Copy
        s.trim()
            .split(&block_delimiter)
            .map(|block| {
                block
                    .split(&line_delimiter)
                    .map(|line| line_parser(line))
                    .collect()
            })
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
        let block_delimiter = BlockDelimiter::default();
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".as_blocks(&block_delimiter);
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
        let block_delimiter = BlockDelimiter::default();
        let s = "abc\r\n\r\na\r\nb\r\nc\r\n\r\nab\r\nac\r\n\r\na\r\na\r\na\r\na\r\n\r\nb"
            .as_blocks(&block_delimiter);
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
    fn test_string_delimiter() {
        let block_delimiter = BlockDelimiter::Delimiter("***".to_string());
        let s =
            "abc\n***\na\nb\nc\n***\nab\nac\n***\na\na\na\na\n***\nb".as_blocks(&block_delimiter);
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
        let block_delimiter = BlockDelimiter::default();
        let expected: Vec<Vec<&str>> = vec![];
        assert_eq!(String::new().as_blocks(&block_delimiter), expected);
        assert_eq!("".as_blocks(&block_delimiter), expected);
    }

    #[test]
    fn test_block_split_single() {
        let block_delimiter = BlockDelimiter::default();
        assert_eq!("abc".as_blocks(&block_delimiter), [["abc"]]);
    }

    #[test]
    fn test_block_split_single_with_newline() {
        let block_delimiter = BlockDelimiter::default();
        assert_eq!("abc\n".as_blocks(&block_delimiter), [["abc"]]);
    }

    #[test]
    fn test_block_split_single_with_newline_and_empty() {
        let block_delimiter = BlockDelimiter::default();
        assert_eq!("abc\n\n".as_blocks(&block_delimiter), [["abc"]]);
    }

    #[test]
    fn test_parse_lines_int() {
        let block_delimiter = BlockDelimiter::default();
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let parsed = INT_EXAMPLE.block_parse_lines(&block_delimiter, |x| x.parse::<u32>().unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_lines_empty() {
        let block_delimiter = BlockDelimiter::default();
        let expected: Vec<Vec<u32>> = vec![];
        let parsed =
            String::new().block_parse_lines(&block_delimiter, |x| x.parse::<u32>().unwrap());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_blocks_empty() {
        let block_delimiter = BlockDelimiter::default();
        let expected: Vec<Vec<u32>> = vec![];
        let parsed = "".block_parse(&block_delimiter, |x| x.parse::<u32>().unwrap(), |x| x);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_blocks_non_reduced() {
        let block_delimiter = BlockDelimiter::default();
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let parsed =
            INT_EXAMPLE.block_parse(&block_delimiter, |x| x.parse::<u32>().unwrap(), |x| x);
        assert_eq!(parsed, expected);
        let parsed = INT_EXAMPLE.block_parse(
            &block_delimiter,
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
        let block_delimiter = BlockDelimiter::default();
        let expected = vec![2000, 0, 1000, 2000, 0];
        let parsed = INT_EXAMPLE.block_parse(
            &block_delimiter,
            |x| x.parse::<u32>().unwrap(),
            |x| x.iter().max().unwrap() - x.iter().min().unwrap(),
        );
        assert_eq!(parsed, expected);
    }
}

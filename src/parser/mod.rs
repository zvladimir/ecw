//! # Parsers for Floating-Point Numbers and Their Formats
//!
//! This library provides parsers for string data that contains:
//! - floating-point numbers (`double`)
//! - numbers with prefixes and suffixes (`%`, `+/-`, `m`, `k`, etc.`)
//! - tolerance values (`-5%`, `+5%`, `+/-5%`)
//!
//! For example:
//! - `"5%"` is parsed as `TolPlusMinus(5.0)`
//! - `"+5%"` is parsed as `TolPlus(5.0)`
//! - `"-5%"` is parsed as `TolMinus(5.0)`
//! - `"10m"` is parsed as `NumberSuffix(10.0, Dim::Milli)`

use crate::types::Dim;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space1},
    multi::separated_list1,
    number::complete::double,
    IResult,
};

/// Enum for various data types that can be parsed.
#[derive(Debug, PartialEq)]
pub enum Block {
    /// A number with a negative sign (e.g., "-5%")
    TolMinus(f64),
    /// A number with a positive sign (e.g., "+5%")
    TolPlus(f64),
    /// A simple number (e.g., "5%") treated as both positive and negative tolerance
    TolPlusMinus(f64),
    /// A simple number (e.g., "5.0")
    Number(f64),
    /// A number with a suffix (e.g., "5k", "10m")
    NumberSuffix((f64, Dim)),
}

/// Parser for a string in the format "-float%"
///
/// # Example
///
/// ```rust
/// use your_crate::percentage_minus_parser;
/// assert_eq!(percentage_minus_parser("-5%"), Ok(("", Block::TolMinus(5.0))));
/// ```
fn percentage_minus_parser(input: &str) -> IResult<&str, Block> {
    let (input, _) = tag("-")(input)?;
    let (input, number) = double(input)?;
    let (input, _) = tag("%")(input)?;

    Ok((input, Block::TolMinus(number.abs())))
}

/// Parser for a string in the format "+/-float%"
///
/// # Example
///
/// ```rust
/// use your_crate::percentage_plus_minus_parser;
/// assert_eq!(percentage_plus_minus_parser("+/-5%"), Ok(("", Block::TolPlusMinus(5.0))));
/// ```
fn percentage_plus_minus_parser(input: &str) -> IResult<&str, Block> {
    let (input, _) = tag("+/-")(input)?;
    let (input, number) = double(input)?;
    let (input, _) = tag("%")(input)?;

    Ok((input, Block::TolPlusMinus(number)))
}

/// Parser for a string in the format "float%" (e.g., "5%").
/// Returns a block with `TolPlusMinus` where the value is both the positive and negative tolerance.
///
/// # Example
///
/// ```rust
/// use your_crate::percentage_plus_parser2;
/// assert_eq!(percentage_plus_parser2("5%"), Ok(("", Block::TolPlusMinus(5.0))));
/// ```
fn percentage_plus_minus_parser2(input: &str) -> IResult<&str, Block> {
    let (input, number) = double(input)?;
    let (input, _) = tag("%")(input)?;

    Ok((input, Block::TolPlusMinus(number)))
}

/// Parser for a string in the format "+float%"
///
/// # Example
///
/// ```rust
/// use your_crate::percentage_plus_parser;
/// assert_eq!(percentage_plus_parser("+5%"), Ok(("", Block::TolPlus(5.0))));
/// ```
fn percentage_plus_parser(input: &str) -> IResult<&str, Block> {
    let (input, _) = tag("+")(input)?;
    let (input, number) = double(input)?;
    let (input, _) = tag("%")(input)?;

    Ok((input, Block::TolPlus(number)))
}

/// Parser for a simple floating-point number (e.g., "5.67")
///
/// # Example
///
/// ```rust
/// use your_crate::double_parser;
/// assert_eq!(double_parser("5.67"), Ok(("", Block::Number(5.67))));
/// ```
fn double_parser(input: &str) -> IResult<&str, Block> {
    let (input, number) = double(input)?;
    Ok((input, Block::Number(number)))
}

/// Parser for a floating-point number followed by a suffix ('m', 'k', 'M', 'p')
///
/// # Example
///
/// ```rust
/// use your_crate::double_suffix_parser;
/// assert_eq!(double_suffix_parser("5k"), Ok(("", Block::NumberSuffix((5.0, Dim::Kilo)))));
/// ```
fn double_suffix_parser(input: &str) -> IResult<&str, Block> {
    let (input, number) = double(input)?;

    let (input, suffix) = alt((
        char('p'), // p -> Pico
        char('n'), // n -> Nano
        char('u'), // u -> Micro
        char('m'), // m -> Milli
        char('k'), // k -> Kilo
        char('M'), // M -> Mega
        char('G'), // G -> Giga
        char('T'), // T -> Tera
    ))(input)?;

    let suffix: Dim = suffix.into();
    let result = Block::NumberSuffix((number, suffix));

    Ok((input, result))
}

/// Parser that tries multiple parsers in sequence
///
/// # Example
///
/// ```rust
/// use your_crate::try_parsers;
/// assert_eq!(try_parsers("5%"), Ok(("", Block::TolPlusMinus(5.0))));
/// ```
fn try_parsers(input: &str) -> IResult<&str, Block> {
    alt((
        percentage_plus_parser,
        percentage_minus_parser,
        percentage_plus_minus_parser,
        percentage_plus_minus_parser2,
        double_suffix_parser,
        double_parser,
    ))(input)
}

/// Parser that splits a string into blocks and applies parsers to each block
///
/// # Example
///
/// ```rust
/// use your_crate::parse_blocks;
/// assert_eq!(
///     parse_blocks("5% 77m"),
///     Ok(("", vec![Block::TolPlusMinus(5.0), Block::NumberSuffix((77.0, Dim::Milli))]))
/// );
/// ```
pub fn parse_blocks(input: &str) -> IResult<&str, Vec<Block>> {
    separated_list1(space1, try_parsers)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_minus_parser() {
        assert_eq!(
            percentage_minus_parser("-5%"),
            Ok(("", Block::TolMinus(5.0)))
        );
    }

    #[test]
    fn test_percentage_plus_minus_parser() {
        assert_eq!(
            percentage_plus_minus_parser("+/-5%"),
            Ok(("", Block::TolPlusMinus(5.0)))
        );
    }

    #[test]
    fn test_percentage_plus_parser() {
        assert_eq!(percentage_plus_parser("+5%"), Ok(("", Block::TolPlus(5.0))));
    }

    #[test]
    fn test_percentage_plus_minus_parser2() {
        assert_eq!(
            percentage_plus_minus_parser2("5%"),
            Ok(("", Block::TolPlusMinus(5.0)))
        );
        assert!(percentage_plus_minus_parser2("5").is_err());
    }

    #[test]
    fn test_double_parser() {
        assert_eq!(double_parser("5.67"), Ok(("", Block::Number(5.67))));
    }

    #[test]
    fn test_double_suffix_parser() {
        assert_eq!(
            double_suffix_parser("5k"),
            Ok(("", Block::NumberSuffix((5.0, Dim::Kilo))))
        );
        assert_eq!(
            double_suffix_parser("10m"),
            Ok(("", Block::NumberSuffix((10.0, Dim::Milli))))
        );
    }

    #[test]
    fn test_parse_blocks() {
        let input = "5% 77m";
        let result = parse_blocks(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Block::TolPlusMinus(5.0),
                    Block::NumberSuffix((77.0, Dim::Milli))
                ]
            ))
        );
    }

    #[test]
    fn test_combined_blocks() {
        let input = "10m +5% -5% +/-5%";
        let result = parse_blocks(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Block::NumberSuffix((10.0, Dim::Milli)),
                    Block::TolPlus(5.0),
                    Block::TolMinus(5.0),
                    Block::TolPlusMinus(5.0),
                ]
            ))
        );
    }
}

use crate::domain::error::ParseError;
use crate::domain::measure::Measure;
use crate::domain::quantity::Quantity;
use crate::domain::unit::{ENGINEERING_PREFIXES, prefix_by_symbol};

/// Reads a field: `4.7k`, `-5`, `10u 1%`, `10k +1% -2%`. A number, an optional SI prefix, then
/// up to one tolerance per side. Empty text is `Ok(None)`; anything else that does not fit is
/// an error that says which part is wrong.
pub fn parse_field(text: &str, quantity: Quantity) -> Result<Option<Measure>, ParseError> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let (number_str, after_number) =
        split_number(trimmed).ok_or_else(|| ParseError::BadNumber {
            quantity,
            text: trimmed.into(),
        })?;

    let nominal: f64 = number_str.parse().map_err(|_| ParseError::BadNumber {
        quantity,
        text: number_str.into(),
    })?;

    let (prefix_str, after_prefix) = split_prefix(after_number);
    let multiplier = prefix_by_symbol(prefix_str)
        .map(|p| p.multiplier)
        .ok_or_else(|| ParseError::BadUnit {
            quantity,
            text: prefix_str.into(),
        })?;

    let after_prefix = after_prefix.trim_start();
    if after_prefix.chars().next().is_some_and(char::is_alphabetic) {
        return Err(ParseError::BadUnit {
            quantity,
            text: after_prefix.into(),
        });
    }

    let (low, high) = parse_tolerance(after_prefix, quantity)?;

    let nominal = nominal * multiplier;
    if !nominal.is_finite() {
        return Err(ParseError::BadNumber {
            quantity,
            text: trimmed.into(),
        });
    }

    Ok(Some(Measure { nominal, low, high }))
}

fn split_number(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    if i < bytes.len() && matches!(bytes[i], b'+' | b'-') {
        i += 1;
    }

    while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
        i += 1;
    }

    if i < bytes.len() && matches!(bytes[i], b'e' | b'E') {
        let mut j = i + 1;
        if j < bytes.len() && matches!(bytes[j], b'+' | b'-') {
            j += 1;
        }
        let digits_start = j;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        if j > digits_start {
            i = j;
        }
    }

    if i == 0 {
        return None;
    }

    Some((&s[..i], &s[i..]))
}

fn split_prefix(s: &str) -> (&str, &str) {
    for p in ENGINEERING_PREFIXES {
        if !p.symbol.is_empty() && s.starts_with(p.symbol) {
            return s.split_at(p.symbol.len());
        }
    }
    ("", s)
}

fn parse_tolerance(text: &str, quantity: Quantity) -> Result<(f64, f64), ParseError> {
    let mut low = 0.0_f64;
    let mut high = 0.0_f64;
    // A side can be set once: `10k 1% 2%` or `10k 1% +2%` would silently drop one value.
    let (mut set_low, mut set_high) = (false, false);

    for token in text.split_whitespace() {
        let (sign, unsigned) = if let Some(rest) = token.strip_prefix('+') {
            (Sign::Upper, rest)
        } else if let Some(rest) = token.strip_prefix('-') {
            (Sign::Lower, rest)
        } else {
            (Sign::Symmetric, token)
        };

        let percent_str = unsigned
            .strip_suffix('%')
            .ok_or_else(|| ParseError::BadTolerance {
                quantity,
                text: token.into(),
            })?;

        let percent: f64 = percent_str.parse().map_err(|_| ParseError::BadTolerance {
            quantity,
            text: token.into(),
        })?;

        if !percent.is_finite() || !(0.0..=100.0).contains(&percent) {
            return Err(ParseError::BadTolerance {
                quantity,
                text: token.into(),
            });
        }

        let (sets_low, sets_high) = match sign {
            Sign::Upper => (false, true),
            Sign::Lower => (true, false),
            Sign::Symmetric => (true, true),
        };
        if (sets_low && set_low) || (sets_high && set_high) {
            return Err(ParseError::BadTolerance {
                quantity,
                text: token.into(),
            });
        }
        set_low |= sets_low;
        set_high |= sets_high;

        let fraction = percent / 100.0;
        match sign {
            Sign::Upper => high = fraction,
            Sign::Lower => low = fraction,
            Sign::Symmetric => {
                low = fraction;
                high = fraction;
            }
        }
    }

    Ok((low, high))
}

#[derive(Debug, Clone, Copy)]
enum Sign {
    Upper,

    Lower,

    Symmetric,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> Result<Option<Measure>, ParseError> {
        parse_field(text, Quantity::Resistance)
    }

    #[test]
    fn asymmetric_tolerance_sets_each_side_once() {
        let m = parse("10k +1% -2%").unwrap().unwrap();
        assert_eq!((m.low, m.high), (0.02, 0.01));
        let m = parse("10k 1%").unwrap().unwrap();
        assert_eq!((m.low, m.high), (0.01, 0.01));
    }

    #[test]
    fn two_tolerances_on_one_side_are_rejected() {
        for text in [
            "10k 1% 2%",
            "10k 1% +2%",
            "10k -1% 2%",
            "10k +1% +2%",
            "10k -1% -2%",
        ] {
            assert!(
                matches!(parse(text), Err(ParseError::BadTolerance { .. })),
                "{text}"
            );
        }
    }

    #[test]
    fn numbers_that_overflow_are_not_numbers() {
        assert!(parse_field("1e999", Quantity::Resistance).is_err());
        assert!(parse_field("1e300G", Quantity::Resistance).is_err());
    }
}

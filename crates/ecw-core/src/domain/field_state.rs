use crate::domain::error::ParseError;
use crate::domain::measure::Measure;
use crate::domain::parse::parse_field;
use crate::domain::quantity::Quantity;

/// What one input field holds.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum FieldState {
    /// Nothing typed.
    #[default]
    Empty,
    /// A value that was read.
    Ok(Measure),
    /// Text that could not be read.
    Err(ParseError),
}

impl FieldState {
    /// Reads `text` as a value of `quantity` (with optional tolerances).
    pub fn parse(text: &str, quantity: Quantity) -> Self {
        parse_field(text, quantity).into()
    }

    /// Like `parse`, but a tolerance is an error: for values that must be exact.
    pub fn parse_exact(text: &str, quantity: Quantity) -> Self {
        match Self::parse(text, quantity) {
            Self::Ok(measure) if !measure.is_exact() => Self::Err(ParseError::BadTolerance {
                quantity,
                text: text.to_owned(),
            }),
            other => other,
        }
    }

    /// An exact value that must be zero or negative.
    pub fn parse_non_positive(text: &str, quantity: Quantity) -> Self {
        match Self::parse_exact(text, quantity) {
            Self::Ok(measure) if measure.to_interval_f64().nominal > 0.0 => {
                Self::Err(ParseError::NotNegative {
                    quantity,
                    text: text.to_owned(),
                })
            }
            other => other,
        }
    }

    /// True if the field holds a value.
    pub const fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    /// True if the field holds text that could not be read.
    pub const fn has_error(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    /// The value, if the field has one.
    pub const fn measure(&self) -> Option<Measure> {
        match self {
            Self::Ok(measure) => Some(*measure),
            _ => None,
        }
    }
}

impl From<Result<Option<Measure>, ParseError>> for FieldState {
    fn from(result: Result<Option<Measure>, ParseError>) -> Self {
        match result {
            Ok(Some(measure)) => Self::Ok(measure),
            Ok(None) => Self::Empty,
            Err(error) => Self::Err(error),
        }
    }
}

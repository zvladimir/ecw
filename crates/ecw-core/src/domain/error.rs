//! Errors of the calculators. They carry what went wrong, not the words: `text()` makes a
//! [`Text`] message that the UI renders in its language.

use std::fmt;

use crate::domain::quantity::Quantity;
use crate::domain::text::Text;
use crate::msg;

/// A field whose text could not be read. Shown under the field.
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum ParseError {
    /// Not a number.
    BadNumber { quantity: Quantity, text: String },

    /// A prefix or a unit that is not known.
    BadUnit { quantity: Quantity, text: String },

    /// A tolerance that is not `N%`, is out of 0…100 %, or is given twice for one side.
    BadTolerance { quantity: Quantity, text: String },

    /// A value that must be zero or negative (for example a reference below the supply).
    NotNegative { quantity: Quantity, text: String },
}

impl ParseError {
    /// The message for this error.
    pub fn text(&self) -> Text {
        let (id, quantity, text) = match self {
            Self::BadNumber { quantity, text } => ("error.parse.bad_number", quantity, text),
            Self::BadUnit { quantity, text } => ("error.parse.bad_unit", quantity, text),
            Self::BadTolerance { quantity, text } => ("error.parse.bad_tolerance", quantity, text),
            Self::NotNegative { quantity, text } => ("error.parse.not_negative", quantity, text),
        };
        msg!(id, quantity = quantity.symbol(), text = text)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text().fmt(f)
    }
}

impl std::error::Error for ParseError {}

/// What an out-of-range value was expected to be.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Requirement {
    /// Below zero.
    Negative,
    /// Not zero.
    NonZero,
    /// Above one.
    GreaterThanOne,
    /// `Vin`, `Vout` and the gain contradict each other.
    ConsistentVinVoutGain,
}

impl Requirement {
    fn text(self) -> Text {
        match self {
            Self::Negative => msg!("error.requirement.negative"),
            Self::NonZero => msg!("error.requirement.non_zero"),
            Self::GreaterThanOne => msg!("error.requirement.greater_than_one"),
            Self::ConsistentVinVoutGain => msg!("error.requirement.consistent_vin_vout_gain"),
        }
    }
}

/// The input is read but the calculation cannot be done with it. Shown in the results.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SolveError {
    /// A calculator needs `expected` values, `got` were given.
    WrongCount { expected: usize, got: usize },

    /// The quantity must be greater than zero over its whole tolerance range.
    NotPositive(Quantity),

    /// A computed quantity came out not positive (or negative under a root).
    ComputedNotPositive { name: Text },

    /// The period is zero or its tolerance range contains zero.
    BadPeriod,

    /// A pulse width or a ramp time is not positive.
    BadPulseWidth,

    /// The pulse is longer than the period.
    PulseExceedsPeriod,

    /// Two values must be in the order `left < right`.
    Ordered { left: Text, right: Text },

    /// The given values leave a free parameter (several circuits fit).
    Underdetermined,

    /// A value the chosen mode needs is not given.
    MissingInput { name: Text },

    /// A value is out of the range that its formula allows.
    OutOfRange {
        name: Text,
        requirement: Requirement,
    },
}

impl SolveError {
    /// The message for this error.
    pub fn text(&self) -> Text {
        match self {
            Self::WrongCount { expected, got } => {
                msg!("error.solve.wrong_count", expected = *expected, got = *got)
            }
            Self::NotPositive(quantity) => {
                msg!("error.solve.not_positive", quantity = quantity.symbol())
            }
            Self::ComputedNotPositive { name } => {
                msg!("error.solve.computed_not_positive", name = name)
            }
            Self::BadPeriod => msg!("error.solve.bad_period"),
            Self::BadPulseWidth => msg!("error.solve.bad_pulse_width"),
            Self::PulseExceedsPeriod => msg!("error.solve.pulse_exceeds_period"),
            Self::Ordered { left, right } => {
                msg!("error.solve.ordered", left = left, right = right)
            }
            Self::Underdetermined => msg!("error.solve.underdetermined"),
            Self::MissingInput { name } => msg!("error.solve.missing_input", name = name),
            Self::OutOfRange { name, requirement } => {
                msg!(
                    "error.solve.out_of_range",
                    name = name,
                    requirement = requirement.text()
                )
            }
        }
    }
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text().fmt(f)
    }
}

impl std::error::Error for SolveError {}

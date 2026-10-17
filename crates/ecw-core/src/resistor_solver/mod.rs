//! Resistor Solver: the standard resistor, or a pair of them in series or in parallel, that
//! is the closest to the wanted value; from an E-series or from the resistors the user has.

pub mod calc;

use crate::domain::ESeries;

/// The wanted resistance of the empty page.
pub const DEFAULT_TARGET: &str = "10.12k";

/// The resistors the user has, for the empty page.
pub const DEFAULT_POOL: &str = "10k, 2k, 100";

/// The E-series the page starts with.
pub const DEFAULT_ESERIES: ESeries = ESeries::E24;

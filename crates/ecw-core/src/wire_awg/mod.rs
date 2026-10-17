//! Wire AWG table: for the current, the length and the thermal resistance, the resistance,
//! voltage drop, power loss and temperature rise of copper wire of every AWG size.

pub mod calc;

/// Default current for the empty page, A.
pub const DEFAULT_CURRENT: &str = "1";

/// Default length of the wire, m.
pub const DEFAULT_LENGTH: &str = "1";

/// Default thermal resistance of the wire to the air, K/W.
pub const DEFAULT_RTH: &str = "50";

/// Default number of parallel wires.
pub const DEFAULT_N: f64 = 1.0;

/// The fewest parallel wires.
pub const MIN_N: f64 = 1.0;

/// The most parallel wires.
pub const MAX_N: f64 = 1000.0;

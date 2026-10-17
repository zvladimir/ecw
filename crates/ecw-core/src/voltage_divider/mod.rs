//! Voltage Divider: from enough of Vin, the outputs and the resistors, the rest of a divider
//! of two resistors (one output) or three (two outputs, tapped divider).

pub mod calc;

use crate::msg;

use crate::domain::{Quantity, Text};

/// How many resistors the divider has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    /// R1 over R2: one output.
    Two,
    /// R1, R2 and R3 in series: two outputs, Vout1 after R1 and Vout2 after R2.
    Three,
}

impl Mode {
    /// The mode with this id (`three`); anything else is the two-resistor divider.
    pub fn parse(text: &str) -> Self {
        match text {
            "three" => Mode::Three,
            _ => Mode::Two,
        }
    }

    /// How many of the quantities the user has to give for this mode.
    pub const fn needed(self) -> usize {
        match self {
            Mode::Two => 3,
            Mode::Three => 4,
        }
    }
}

/// A quantity of the divider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Var {
    /// The input voltage.
    Vin,
    /// The output after R1 (the only output of the two-resistor divider).
    Vout1,
    /// The second output, after R2 (three resistors only).
    Vout2,
    /// The top resistor, from Vin.
    R1,
    /// The second resistor (the bottom one with two).
    R2,
    /// The bottom resistor (three resistors only).
    R3,
}

impl Var {
    /// The key of the field in the request.
    pub const fn key(self) -> &'static str {
        match self {
            Var::Vin => "vin",
            Var::Vout1 => "vout1",
            Var::Vout2 => "vout2",
            Var::R1 => "r1",
            Var::R2 => "r2",
            Var::R3 => "r3",
        }
    }

    /// The caption of the field: it depends on the mode (`Vout` or `Vout1`, `R2` at the bottom or
    /// in the middle).
    pub fn label(self, mode: Mode) -> Text {
        match (self, mode) {
            (Var::Vin, _) => msg!("voltage_divider.label.vin"),
            (Var::Vout1, Mode::Two) => msg!("voltage_divider.label.vout"),
            (Var::Vout1, Mode::Three) => msg!("voltage_divider.label.vout1"),
            (Var::Vout2, _) => msg!("voltage_divider.label.vout2"),
            (Var::R1, _) => msg!("voltage_divider.label.r1_top"),
            (Var::R2, Mode::Two) => msg!("voltage_divider.label.r2_bottom"),
            (Var::R2, Mode::Three) => msg!("voltage_divider.label.r2_mid"),
            (Var::R3, _) => msg!("voltage_divider.label.r3_bottom"),
        }
    }

    /// The unit of the field.
    pub const fn unit(self) -> &'static str {
        match self {
            Var::Vin | Var::Vout1 | Var::Vout2 => "V",
            Var::R1 | Var::R2 | Var::R3 => "Ω",
        }
    }

    /// The physical quantity of the field.
    pub const fn quantity(self) -> Quantity {
        match self {
            Var::Vin | Var::Vout1 | Var::Vout2 => Quantity::Voltage,
            Var::R1 | Var::R2 | Var::R3 => Quantity::Resistance,
        }
    }
}

/// The fields of a mode, in the order of the form.
pub const fn vars_for(mode: Mode) -> &'static [Var] {
    match mode {
        Mode::Two => &[Var::Vin, Var::Vout1, Var::R1, Var::R2],
        Mode::Three => &[Var::Vin, Var::Vout1, Var::Vout2, Var::R1, Var::R2, Var::R3],
    }
}

use std::fmt;

/// The physical quantity of an input field: it sets the symbol, the unit and the hint shown to
/// the user, and is named in the error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quantity {
    Voltage,

    Current,

    Resistance,

    Power,

    Length,

    ThermalResistance,

    Time,

    TemperatureDelta,

    Capacitance,

    Frequency,

    Ratio,

    SlewRate,
}

impl Quantity {
    /// The short name used for the field and in messages: `U`, `I`, `R`.
    #[inline]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Voltage => "U",
            Self::Current => "I",
            Self::Resistance => "R",
            Self::Power => "P",
            Self::Length => "L",
            Self::ThermalResistance => "Rth",
            Self::Time => "T",
            Self::TemperatureDelta => "ΔT",
            Self::Capacitance => "C",
            Self::Frequency => "f",
            Self::Ratio => "G",
            Self::SlewRate => "SR",
        }
    }

    /// The unit of the quantity: `V`, `A`, `Ω`.
    #[inline]
    pub const fn unit(self) -> &'static str {
        match self {
            Self::Voltage => "V",
            Self::Current => "A",
            Self::Resistance => "Ω",
            Self::Power => "W",
            Self::Length => "m",
            Self::ThermalResistance => "K/W",
            Self::Time => "s",
            Self::TemperatureDelta => "K",
            Self::Capacitance => "F",
            Self::Frequency => "Hz",
            Self::Ratio => "V/V",
            Self::SlewRate => "V/µs",
        }
    }

    /// Example values for the empty field.
    #[inline]
    pub const fn input_hint(self) -> &'static str {
        match self {
            Self::Voltage => "12, 12m",
            Self::Current => "0.5, 500m",
            Self::Resistance => "4.7k, 100, 2.2M",
            Self::Power => "3, 250m",
            Self::Length => "1, 100m",
            Self::ThermalResistance => "10, 1.5",
            Self::Time => "10u, 5m",
            Self::TemperatureDelta => "25, 5",
            Self::Capacitance => "10n, 1u, 100p",
            Self::Frequency => "1M, 1k, 100",
            Self::Ratio => "10, -10, 10 1%",
            Self::SlewRate => "0.5, 13, 0.5 20%",
        }
    }

    /// The caption of the field: `R [Ω]`.
    pub fn label(self) -> String {
        format!("{} [{}]", self.symbol(), self.unit())
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.symbol())
    }
}

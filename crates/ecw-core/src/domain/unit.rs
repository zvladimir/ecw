use uom::si::f64::{
    Area, Capacitance, ElectricCurrent, ElectricPotential, ElectricalConductance,
    ElectricalResistance, Frequency, Length, Power, Ratio, TemperatureInterval, ThermalResistance,
    Time,
};

/// A physical quantity with its base unit. `to_base` and `from_base` are for the edges only:
/// parsing, formatting, plotting and the storage of `infer`; calculations stay on the types.
pub trait Measured: Copy + PartialOrd + uom::num_traits::Zero {
    /// The unit symbol of the base unit: `V`, `Ω`; empty for a ratio.
    const UNIT: &'static str;

    /// The value in the base unit of the quantity (SI).
    fn to_base(self) -> f64;

    /// The quantity from a number in the base unit.
    fn from_base(v: f64) -> Self;
}

macro_rules! impl_measured {
    ($ty:ty, $unit:path, $label:literal) => {
        impl Measured for $ty {
            const UNIT: &'static str = $label;

            #[inline]
            fn to_base(self) -> f64 {
                self.get::<$unit>()
            }

            #[inline]
            fn from_base(v: f64) -> Self {
                <$ty>::new::<$unit>(v)
            }
        }
    };
}

impl_measured!(ElectricPotential, uom::si::electric_potential::volt, "V");
impl_measured!(ElectricCurrent, uom::si::electric_current::ampere, "A");
impl_measured!(
    ElectricalResistance,
    uom::si::electrical_resistance::ohm,
    "Ω"
);
impl_measured!(
    ElectricalConductance,
    uom::si::electrical_conductance::siemens,
    "S"
);
impl_measured!(Power, uom::si::power::watt, "W");
impl_measured!(Length, uom::si::length::meter, "m");
impl_measured!(Area, uom::si::area::square_meter, "m²");
impl_measured!(
    ThermalResistance,
    uom::si::thermal_resistance::kelvin_per_watt,
    "K/W"
);
impl_measured!(
    TemperatureInterval,
    uom::si::temperature_interval::kelvin,
    "K"
);
impl_measured!(Time, uom::si::time::second, "s");
impl_measured!(Capacitance, uom::si::capacitance::farad, "F");
impl_measured!(Frequency, uom::si::frequency::hertz, "Hz");
impl_measured!(Ratio, uom::si::ratio::ratio, "");

/// An SI prefix: the symbol, the power of ten and the multiplier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Prefix {
    /// The power of ten: 3 for `k`.
    pub exponent: i32,
    /// The text written before the unit: `k`, `m`, `u`.
    pub symbol: &'static str,
    /// `10^exponent`.
    pub multiplier: f64,
}

/// The prefixes of the input and of the output: powers of 1000 from `G` to `p`, the biggest first.
/// The prefix is looked up by its symbol, so a longer symbol must come before a shorter one.
pub const ENGINEERING_PREFIXES: &[Prefix] = &[
    Prefix {
        exponent: 9,
        symbol: "G",
        multiplier: 1e9,
    },
    Prefix {
        exponent: 6,
        symbol: "M",
        multiplier: 1e6,
    },
    Prefix {
        exponent: 3,
        symbol: "k",
        multiplier: 1e3,
    },
    Prefix {
        exponent: 0,
        symbol: "",
        multiplier: 1.0,
    },
    Prefix {
        exponent: -3,
        symbol: "m",
        multiplier: 1e-3,
    },
    Prefix {
        exponent: -6,
        symbol: "u",
        multiplier: 1e-6,
    },
    Prefix {
        exponent: -9,
        symbol: "n",
        multiplier: 1e-9,
    },
    Prefix {
        exponent: -12,
        symbol: "p",
        multiplier: 1e-12,
    },
];

/// The prefix written as `symbol` (empty for none).
pub fn prefix_by_symbol(symbol: &str) -> Option<&'static Prefix> {
    ENGINEERING_PREFIXES.iter().find(|p| p.symbol == symbol)
}

/// The prefix of the power of ten `exponent`.
pub fn prefix_by_exponent(exponent: i32) -> Option<&'static Prefix> {
    ENGINEERING_PREFIXES.iter().find(|p| p.exponent == exponent)
}

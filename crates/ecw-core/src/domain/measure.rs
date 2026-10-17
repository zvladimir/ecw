use crate::domain::interval::Interval;
use crate::domain::unit::Measured;

/// What the user typed in a field: a number in base units and its tolerances as fractions of the
/// nominal (`10k +1% -2%`: `high = 0.01`, `low = 0.02`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Measure {
    /// The value in base units (V, A, Ω …).
    pub nominal: f64,

    /// The tolerance below the nominal, a fraction.
    pub low: f64,

    /// The tolerance above the nominal, a fraction.
    pub high: f64,
}

impl Measure {
    /// A value without tolerances.
    #[inline]
    pub const fn exact(nominal: f64) -> Self {
        Self {
            nominal,
            low: 0.0,
            high: 0.0,
        }
    }

    /// True if both tolerances are zero.
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.low == 0.0 && self.high == 0.0
    }

    /// The lowest value: `nominal - |nominal| · low`.
    #[inline]
    pub fn lower_bound(&self) -> f64 {
        self.nominal - self.nominal.abs() * self.low
    }

    /// The highest value: `nominal + |nominal| · high`.
    #[inline]
    pub fn upper_bound(&self) -> f64 {
        self.nominal + self.nominal.abs() * self.high
    }

    /// The value as an interval of numbers in base units.
    #[inline]
    pub fn to_interval_f64(&self) -> Interval<f64> {
        Interval {
            nominal: self.nominal,
            lo: self.lower_bound(),
            hi: self.upper_bound(),
        }
    }

    /// The value as an interval of a physical type.
    #[inline]
    pub fn to_interval<T: Measured>(&self) -> Interval<T> {
        Interval {
            nominal: T::from_base(self.nominal),
            lo: T::from_base(self.lower_bound()),
            hi: T::from_base(self.upper_bound()),
        }
    }
}

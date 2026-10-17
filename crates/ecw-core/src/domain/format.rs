use crate::domain::interval::Interval;
use crate::domain::unit::{ENGINEERING_PREFIXES, Measured, prefix_by_exponent};

/// Formats `value` with an SI prefix: `4.700 kΩ`. `decimals` is the number of digits after the
/// point. Prefixes are the engineering ones (powers of 1000); a value beyond them keeps the
/// nearest.
pub fn format_engineering(value: f64, unit: &str, decimals: u32) -> String {
    if !value.is_finite() {
        return format!("{value} {unit}");
    }

    let magnitude = value.abs();
    let width = decimals as usize;
    let (smallest, largest) = (
        ENGINEERING_PREFIXES
            .iter()
            .map(|p| p.exponent)
            .min()
            .unwrap_or(0),
        ENGINEERING_PREFIXES
            .iter()
            .map(|p| p.exponent)
            .max()
            .unwrap_or(0),
    );
    let mut exponent = if magnitude == 0.0 {
        0
    } else {
        let e = magnitude.log10().floor() as i32;
        (e.div_euclid(3) * 3).clamp(smallest, largest)
    };

    // A value that rounds up to 1000 belongs to the next prefix: 999.9996 Ω is 1.000 kΩ.
    let scaled = |exponent: i32| value / 10f64.powi(exponent);
    if exponent < largest && format!("{:.width$}", scaled(exponent).abs()).starts_with("1000") {
        exponent += 3;
    }

    match prefix_by_exponent(exponent) {
        Some(prefix) => {
            let mantissa = value / prefix.multiplier;
            format!("{mantissa:.width$} {}{unit}", prefix.symbol)
        }
        None => format!("{value:e} {unit}"),
    }
}

/// A result as texts for the table: the nominal value, the worst-case minimum and maximum, and
/// the deviation from the nominal in percent and in units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Breakdown {
    /// The nominal value: `4.700 kΩ`.
    pub nominal: String,
    /// The lowest value in the tolerance range.
    pub min: String,
    /// The highest value in the tolerance range.
    pub max: String,
    /// The upper deviation in percent: `+1.000 %`.
    pub upper_percent: String,
    /// The upper deviation in units.
    pub upper_absolute: String,
    /// The lower deviation in percent: `-1.000 %`.
    pub lower_percent: String,
    /// The lower deviation in units.
    pub lower_absolute: String,
}

/// Formats an interval as a [`Breakdown`].
pub fn breakdown<T: Measured>(interval: Interval<T>, decimals: u32) -> Breakdown {
    let unit = T::UNIT;
    let width = decimals as usize;

    let nominal = interval.nominal.to_base();
    let lo = interval.lo.to_base();
    let hi = interval.hi.to_base();

    let upper_percent_value = if nominal != 0.0 {
        (hi - nominal) / nominal * 100.0
    } else {
        0.0
    };
    let lower_percent_value = if nominal != 0.0 {
        (nominal - lo) / nominal * 100.0
    } else {
        0.0
    };

    Breakdown {
        nominal: format_engineering(nominal, unit, decimals),
        min: format_engineering(lo, unit, decimals),
        max: format_engineering(hi, unit, decimals),

        upper_percent: format!("+{upper_percent_value:.width$} %"),
        upper_absolute: format!("+{}", format_engineering(hi - nominal, unit, decimals)),

        lower_percent: format!("-{lower_percent_value:.width$} %"),
        lower_absolute: format!("-{}", format_engineering(nominal - lo, unit, decimals)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_value_that_rounds_up_takes_the_next_prefix() {
        assert_eq!(format_engineering(999.9996, "Ω", 3), "1.000 kΩ");
        assert_eq!(format_engineering(-999.9996, "Ω", 3), "-1.000 kΩ");
        assert_eq!(format_engineering(999.0, "Ω", 3), "999.000 Ω");
    }

    #[test]
    fn values_beyond_the_prefixes_keep_the_nearest_one() {
        assert!(!format_engineering(3e-14, "V", 4).contains('e'));
        assert!(!format_engineering(2e12, "Ω", 4).contains('e'));
    }
}

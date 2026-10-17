//! Standard values for the design pages: the E24 and E96 tables of [`ESeries`] and E192,
//! which is generated.

use std::sync::OnceLock;

use super::ESeries;
use super::eseries::nearest_in;

/// The series of standard values offered by the design pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Series {
    /// 5 % series, 24 values per decade.
    E24,
    /// 1 % series, 96 values per decade.
    E96,
    /// 0.5 % series, 192 values per decade.
    E192,
}

impl Series {
    /// Every series, from the coarsest to the finest.
    pub const ALL: [Series; 3] = [Series::E24, Series::E96, Series::E192];

    /// The series with this name; an unknown name reads as E192.
    pub fn parse(text: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|s| s.name() == text)
            .unwrap_or(Series::E192)
    }

    /// The name of the series: `E96`.
    pub fn name(self) -> &'static str {
        match self {
            Series::E24 => "E24",
            Series::E96 => "E96",
            Series::E192 => "E192",
        }
    }

    /// Standard values over many decades, ascending.
    pub fn values(self) -> &'static [f64] {
        static E192: OnceLock<Vec<f64>> = OnceLock::new();
        match self {
            Series::E24 => ESeries::E24.values(),
            Series::E96 => ESeries::E96.values(),
            Series::E192 => E192.get_or_init(|| generate(192, -3..=9)),
        }
    }

    /// The standard value closest to `target`.
    pub fn nearest(self, target: f64) -> f64 {
        nearest_in(self.values(), target)
    }
}

/// E48, E96 and E192 are the values `10^(k/N)` rounded to three significant digits.
fn generate(n: u32, decades: std::ops::RangeInclusive<i32>) -> Vec<f64> {
    let mut out = Vec::new();
    for decade in decades {
        let scale = 10f64.powi(decade);
        for k in 0..n {
            let mantissa = (10f64.powf(f64::from(k) / f64::from(n)) * 100.0).round() / 100.0;
            // IEC 60063 lists 9.20 although 10^(185/192) = 9.195 rounds down to 9.19.
            let mantissa = if n == 192 && k == 185 { 9.20 } else { mantissa };
            out.push(mantissa * scale);
        }
    }
    out.sort_by(f64::total_cmp);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e192_has_the_values_of_the_task_and_e96_does_not() {
        assert_eq!(Series::E192.nearest(1040.0), 1040.0);
        assert_eq!(Series::E192.nearest(5541.0), 5560.0);
        assert_eq!(Series::E96.nearest(5541.0), 5490.0);
        assert_eq!(Series::E96.nearest(1040.0), 1050.0);
    }

    #[test]
    fn e192_follows_iec_60063_at_9_20() {
        assert_eq!(Series::E192.nearest(9200.0), 9200.0);
        assert_eq!(Series::E192.nearest(9190.0), 9200.0);
    }

    #[test]
    fn generated_e96_equals_the_table() {
        let table = ESeries::E96.values();
        let made = generate(96, -3..=9);
        assert_eq!(table.len(), made.len());
        for (a, b) in table.iter().zip(&made) {
            assert!((a - b).abs() <= 1e-9 * a.abs(), "{a} vs {b}");
        }
    }
}

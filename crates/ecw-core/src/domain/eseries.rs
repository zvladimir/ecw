use std::sync::OnceLock;

/// A series of preferred values of resistors and capacitors (IEC 60063).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    strum::IntoStaticStr,
    strum::EnumCount,
    strum::VariantArray,
)]
#[repr(u8)]
pub enum ESeries {
    /// ±20 %, 6 values per decade.
    E6 = 0,

    /// ±10 %, 12 values per decade.
    E12 = 1,

    /// ±5 %, 24 values per decade (the default).
    #[default]
    E24 = 2,

    /// ±2 %, 48 values per decade.
    E48 = 3,

    /// ±1 %, 96 values per decade.
    E96 = 4,
}

impl ESeries {
    /// Every series, from the coarsest to the finest.
    pub const ALL: &'static [ESeries] = <ESeries as strum::VariantArray>::VARIANTS;

    /// The name of the series: `E24`.
    #[inline]
    pub fn name(self) -> &'static str {
        self.into()
    }

    /// The tolerance class of the series as a fraction (E24: 0.05).
    #[inline]
    pub const fn tolerance(self) -> f64 {
        match self {
            Self::E6 => 0.20,
            Self::E12 => 0.10,
            Self::E24 => 0.05,
            Self::E48 => 0.02,
            Self::E96 => 0.01,
        }
    }

    /// The values over the decades 10⁻³ … 10⁹, ascending. Computed once.
    pub fn values(self) -> &'static [f64] {
        const N: usize = <ESeries as strum::EnumCount>::COUNT;
        static CACHE: [OnceLock<Box<[f64]>>; N] = [const { OnceLock::new() }; N];

        CACHE[self as usize]
            .get_or_init(|| compute(self).into_boxed_slice())
            .as_ref()
    }

    fn base_values(self) -> &'static [f64] {
        match self {
            Self::E6 => &[1.0, 1.5, 2.2, 3.3, 4.7, 6.8],

            Self::E12 => &[1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2],

            Self::E24 => &[
                1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3,
                4.7, 5.1, 5.6, 6.2, 6.8, 7.5, 8.2, 9.1,
            ],

            Self::E48 => &[
                1.00, 1.05, 1.10, 1.15, 1.21, 1.27, 1.33, 1.40, 1.47, 1.54, 1.62, 1.69, 1.78, 1.87,
                1.96, 2.05, 2.15, 2.26, 2.37, 2.49, 2.61, 2.74, 2.87, 3.01, 3.16, 3.32, 3.48, 3.65,
                3.83, 4.02, 4.22, 4.42, 4.64, 4.87, 5.11, 5.36, 5.62, 5.90, 6.19, 6.49, 6.81, 7.15,
                7.50, 7.87, 8.25, 8.66, 9.09, 9.53,
            ],

            Self::E96 => &[
                1.00, 1.02, 1.05, 1.07, 1.10, 1.13, 1.15, 1.18, 1.21, 1.24, 1.27, 1.30, 1.33, 1.37,
                1.40, 1.43, 1.47, 1.50, 1.54, 1.58, 1.62, 1.65, 1.69, 1.74, 1.78, 1.82, 1.87, 1.91,
                1.96, 2.00, 2.05, 2.10, 2.15, 2.21, 2.26, 2.32, 2.37, 2.43, 2.49, 2.55, 2.61, 2.67,
                2.74, 2.80, 2.87, 2.94, 3.01, 3.09, 3.16, 3.24, 3.32, 3.40, 3.48, 3.57, 3.65, 3.74,
                3.83, 3.92, 4.02, 4.12, 4.22, 4.32, 4.42, 4.53, 4.64, 4.75, 4.87, 4.99, 5.11, 5.23,
                5.36, 5.49, 5.62, 5.76, 5.90, 6.04, 6.19, 6.34, 6.49, 6.65, 6.81, 6.98, 7.15, 7.32,
                7.50, 7.68, 7.87, 8.06, 8.25, 8.45, 8.66, 8.87, 9.09, 9.31, 9.53, 9.76,
            ],
        }
    }
}

const DECADE_MIN: i32 = -3;

const DECADE_MAX: i32 = 9;

fn compute(series: ESeries) -> Vec<f64> {
    let base = series.base_values();
    let mut out = Vec::with_capacity(base.len() * (DECADE_MAX - DECADE_MIN + 1) as usize);

    for decade in DECADE_MIN..=DECADE_MAX {
        let multiplier = 10f64.powi(decade);
        out.extend(base.iter().map(|&v| v * multiplier));
    }

    out.sort_by(f64::total_cmp);
    out
}

/// The value of `sorted` closest to `target` (the lower one on a tie). Works on any quantity:
/// the two neighbours `lo < target <= hi` are compared by their distances.
pub fn nearest_in<T: Copy + PartialOrd + std::ops::Sub<Output = T>>(sorted: &[T], target: T) -> T {
    match sorted.len() {
        0 => target,
        1 => sorted[0],
        _ => {
            let hi_idx = sorted.partition_point(|&v| v < target);

            if hi_idx == 0 {
                return sorted[0];
            }
            if hi_idx == sorted.len() {
                return sorted[sorted.len() - 1];
            }

            let lo = sorted[hi_idx - 1];
            let hi = sorted[hi_idx];

            if hi - target < target - lo { hi } else { lo }
        }
    }
}

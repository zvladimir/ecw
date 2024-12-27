pub mod current;
pub mod power;
pub mod resistance;
pub mod voltage;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    EmptyInput,
    IncorrectInput(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tolerance {
    pub plus: f64,
    pub minus: f64,
}

#[derive(Debug, PartialEq)]
pub enum Dim {
    Pico,
    Nano,
    Micro,
    Milli,
    None,
    Kilo,
    Mega,
    Giga,
    Tera,
}

impl From<char> for Dim {
    fn from(c: char) -> Self {
        match c {
            'p' => Dim::Pico,
            'n' => Dim::Nano,
            'u' => Dim::Micro,
            'm' => Dim::Milli,
            'k' => Dim::Kilo,
            'M' => Dim::Mega,
            'G' => Dim::Giga,
            'T' => Dim::Tera,
            _ => Dim::None,
        }
    }
}

impl Dim {
    /// Converts the `Dim` variant to its corresponding coefficient (as a power of 10).
    pub fn coefficient(&self) -> f64 {
        match self {
            Dim::Pico => 1e-12,
            Dim::Nano => 1e-9,
            Dim::Micro => 1e-6,
            Dim::Milli => 1e-3,
            Dim::None => 1.0, // No scaling factor
            Dim::Kilo => 1e3,
            Dim::Mega => 1e6,
            Dim::Giga => 1e9,
            Dim::Tera => 1e12,
        }
    }
}

pub trait Measurement {
    fn get_nominal_value(&self) -> f64;
    fn get_tolerance(&self) -> Option<Tolerance>;
    fn get_unit(&self) -> &'static str;

    fn normalize(&self, value: f64) -> String {
        let unit = self.get_unit();
        let prefixes = [
            (1e-12, "p"),
            (1e-9, "n"),
            (1e-6, "u"),
            (1e-3, "m"),
            (1.0, ""),
            (1e3, "k"),
            (1e6, "M"),
            (1e9, "G"),
            (1e12, "T"),
        ];

        for &(threshold, prefix) in prefixes.iter().rev() {
            if value.abs() >= threshold {
                let formatted_value = value / threshold;
                return format!("{:.2}{}{}", formatted_value, prefix, unit);
            }
        }

        format!("{}", value)
    }

    fn get_value_nom(&self) -> String {
        let value = self.get_nominal_value();

        self.normalize(value)
    }

    fn get_value_min(&self) -> String {
        if let Some(tol) = self.get_tolerance() {
            let min = self.get_nominal_value() * (100.0 - tol.minus) / 100.0;
            self.normalize(min)
        } else {
            "N/A".to_string()
        }
    }

    fn get_value_max(&self) -> String {
        if let Some(tol) = self.get_tolerance() {
            let max = self.get_nominal_value() * (100.0 + tol.plus) / 100.0;
            self.normalize(max)
        } else {
            "N/A".to_string()
        }
    }

    fn get_tol_value_plus(&self) -> String {
        if let Some(tol) = self.get_tolerance() {
            let delta = self.get_nominal_value() * tol.plus / 100.0;
            self.normalize(delta)
        } else {
            "N/A".to_string()
        }
    }

    fn get_tol_value_minus(&self) -> String {
        if let Some(tol) = self.get_tolerance() {
            let delta = self.get_nominal_value() * tol.minus / 100.0;
            let result = self.normalize(delta);
            format!("-{}", result)
        } else {
            "N/A".to_string()
        }
    }

    fn get_tol_percent_plus(&self) -> String {
        if let Some(tol) = self.get_tolerance() {
            format!("{:.2}%", tol.plus)
        } else {
            "N/A".to_string()
        }
    }

    fn get_tol_percent_minus(&self) -> String {
        if let Some(tol) = self.get_tolerance() {
            format!("-{:.2}%", tol.minus)
        } else {
            "N/A".to_string()
        }
    }
}

pub fn calculate_multiplication_with_tolerance<M: Measurement, N: Measurement>(
    factor1: &M,
    factor2: &N,
) -> (f64, Option<Tolerance>) {
    let operand1_nom = factor1.get_nominal_value();
    let operand2_nom = factor2.get_nominal_value();

    let result = operand1_nom * operand2_nom;

    let operand1_tol = factor1.get_tolerance();
    let operand2_tol = factor2.get_tolerance();

    if operand1_tol.is_none() && operand2_tol.is_none() {
        return (result, None);
    }

    let (operand1_min, operand1_max) = match operand1_tol {
        Some(tol) => (tol.minus, tol.plus),
        None => (0.0, 0.0),
    };

    let (operand2_min, operand2_max) = match operand2_tol {
        Some(tol) => (tol.minus, tol.plus),
        None => (0.0, 0.0),
    };
    let tol = Tolerance {
        plus: operand1_max + operand2_max,
        minus: operand1_min + operand2_min,
    };

    (result, Some(tol))
}

pub fn calculate_division_with_tolerance<M: Measurement, N: Measurement>(
    factor1: &M,
    factor2: &N,
) -> (f64, Option<Tolerance>) {
    if factor2.get_nominal_value() == 0.0 {
        panic!("Division by zero is not allowed.");
    }

    let operand1_nom = factor1.get_nominal_value();
    let operand2_nom = factor2.get_nominal_value();

    let result = operand1_nom / operand2_nom;

    let operand1_tol = factor1.get_tolerance();
    let operand2_tol = factor2.get_tolerance();

    if operand1_tol.is_none() && operand2_tol.is_none() {
        return (result, None);
    }

    let (operand1_min, operand1_max) = match operand1_tol {
        Some(tol) => (tol.minus, tol.plus),
        None => (0.0, 0.0),
    };

    let (operand2_min, operand2_max) = match operand2_tol {
        Some(tol) => (tol.minus, tol.plus),
        None => (0.0, 0.0),
    };

    let tol = Tolerance {
        plus: operand1_max + operand2_min,
        minus: operand1_min + operand2_max,
    };

    (result, Some(tol))
}

pub fn calculate_addition_with_tolerance<M: Measurement, N: Measurement>(
    factor1: &M,
    factor2: &N,
) -> (f64, Option<Tolerance>) {
    let operand1_nom = factor1.get_nominal_value();
    let operand2_nom = factor2.get_nominal_value();

    let result = operand1_nom + operand2_nom;

    let operand1_tol = factor1.get_tolerance();
    let operand2_tol = factor2.get_tolerance();

    if operand1_tol.is_none() && operand2_tol.is_none() {
        return (result, None);
    }

    let (operand1_min, operand1_max) = match operand1_tol {
        Some(tol) => (
            operand1_nom - operand1_nom * (1.0 - tol.minus / 100.0),
            operand1_nom * (1.0 + tol.plus / 100.0) - operand1_nom,
        ),
        None => (0.0, 0.0),
    };

    let (operand2_min, operand2_max) = match operand2_tol {
        Some(tol) => (
            operand2_nom - operand2_nom * (1.0 - tol.minus / 100.0),
            operand2_nom * (1.0 + tol.plus / 100.0) - operand2_nom,
        ),
        None => (0.0, 0.0),
    };

    let max_result = operand1_max + operand2_max;
    let min_result = operand1_min + operand2_min;

    let tol_plus = (max_result / result) * 100.0;
    let tol_minus = (min_result / result) * 100.0;

    let tol = Tolerance {
        plus: tol_plus,
        minus: tol_minus,
    };

    (result, Some(tol))
}

pub fn calculate_subtraction_with_tolerance<M: Measurement, N: Measurement>(
    factor1: &M,
    factor2: &N,
) -> (f64, Option<Tolerance>) {
    let operand1_nom = factor1.get_nominal_value();
    let operand2_nom = factor2.get_nominal_value();

    let result = operand1_nom - operand2_nom;

    let operand1_tol = factor1.get_tolerance();
    let operand2_tol = factor2.get_tolerance();

    if operand1_tol.is_none() && operand2_tol.is_none() {
        return (result, None);
    }

    let (operand1_min, operand1_max) = match operand1_tol {
        Some(tol) => (
            operand1_nom - operand1_nom * (1.0 - tol.minus / 100.0),
            operand1_nom * (1.0 + tol.plus / 100.0) - operand1_nom,
        ),
        None => (0.0, 0.0),
    };

    let (operand2_min, operand2_max) = match operand2_tol {
        Some(tol) => (
            operand2_nom - operand2_nom * (1.0 - tol.minus / 100.0),
            operand2_nom * (1.0 + tol.plus / 100.0) - operand2_nom,
        ),
        None => (0.0, 0.0),
    };

    let max_result = operand1_max + operand2_max;
    let min_result = operand1_min + operand2_min;

    let tol_plus = (max_result / result) * 100.0;
    let tol_minus = (min_result / result) * 100.0;

    let tol = Tolerance {
        plus: tol_plus,
        minus: tol_minus,
    };

    (result, Some(tol))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_measurement() {
        struct Test;

        impl Measurement for Test {
            fn get_nominal_value(&self) -> f64 {
                220.0
            }

            fn get_tolerance(&self) -> Option<Tolerance> {
                Some(Tolerance {
                    plus: 5.0,
                    minus: 3.3,
                })
            }

            fn get_unit(&self) -> &'static str {
                "TEST"
            }
        }

        let test = Test;

        assert_eq!(test.get_unit(), "TEST");
        assert_eq!(
            test.get_tolerance(),
            Some(Tolerance {
                plus: 5.0,
                minus: 3.3
            })
        );
        assert_eq!(test.get_nominal_value(), 220.0);
        assert_eq!(test.get_value_nom(), "220.00TEST");
        assert_eq!(test.get_value_min(), "212.74TEST");
        assert_eq!(test.get_value_max(), "231.00TEST");
        assert_eq!(test.get_tol_value_plus(), "11.00TEST");
        assert_eq!(test.get_tol_value_minus(), "-7.26TEST");
        assert_eq!(test.get_tol_percent_plus(), "5.00%");
        assert_eq!(test.get_tol_percent_minus(), "-3.30%");
    }

    #[test]
    fn test_trait_calculation() {
        struct Value1;
        impl Measurement for Value1 {
            fn get_nominal_value(&self) -> f64 {
                300.0
            }

            fn get_tolerance(&self) -> Option<Tolerance> {
                Some(Tolerance {
                    plus: 5.0,
                    minus: 3.3,
                })
            }

            fn get_unit(&self) -> &'static str {
                "V1"
            }
        }

        let value1 = Value1;

        struct Value2;
        impl Measurement for Value2 {
            fn get_nominal_value(&self) -> f64 {
                150.0
            }

            fn get_tolerance(&self) -> Option<Tolerance> {
                Some(Tolerance {
                    plus: 1.0,
                    minus: 2.5,
                })
            }

            fn get_unit(&self) -> &'static str {
                "V2"
            }
        }

        let value2 = Value2;

        // *
        let a = calculate_multiplication_with_tolerance(&value1, &value2);
        assert_eq!(a.0, 45000.0);
        assert_eq!(
            a.1,
            Some(Tolerance {
                plus: 6.0,
                minus: 5.8
            })
        );
        // /
        let b = calculate_division_with_tolerance(&value1, &value2);
        assert_eq!(b.0, 2.0);
        assert_eq!(
            b.1,
            Some(Tolerance {
                plus: 7.5,
                minus: 4.3
            })
        );
        // +
        let c = calculate_addition_with_tolerance(&value1, &value2);
        assert_eq!(c.0, 450.0);
        assert_eq!(
            c.1,
            Some(Tolerance {
                plus: 3.6666666666666665,
                minus: 3.033333333333341
            })
        );
        // -
        let d = calculate_subtraction_with_tolerance(&value1, &value2);
        assert_eq!(d.0, 150.0);
        assert_eq!(
            d.1,
            Some(Tolerance {
                plus: 11.0,
                minus: 9.100000000000023
            })
        );

        struct Value3;
        impl Measurement for Value3 {
            fn get_nominal_value(&self) -> f64 {
                150.0
            }

            fn get_tolerance(&self) -> Option<Tolerance> {
                None
            }

            fn get_unit(&self) -> &'static str {
                "V3"
            }
        }

        let value3 = Value3;

        // *
        let a = calculate_multiplication_with_tolerance(&value1, &value3);
        assert_eq!(a.0, 45000.0);
        assert_eq!(
            a.1,
            Some(Tolerance {
                plus: 5.0,
                minus: 3.3
            })
        );

        // /
        let b = calculate_division_with_tolerance(&value1, &value3);
        assert_eq!(b.0, 2.0);
        assert_eq!(
            b.1,
            Some(Tolerance {
                plus: 5.0,
                minus: 3.3
            })
        );

        // +
        let c = calculate_addition_with_tolerance(&value1, &value3);
        assert_eq!(c.0, 450.0);
        assert_eq!(
            c.1,
            Some(Tolerance {
                plus: 3.3333333333333335,
                minus: 2.2000000000000073
            })
        );

        // -
        let d = calculate_subtraction_with_tolerance(&value1, &value3);
        assert_eq!(d.0, 150.0);
        assert_eq!(
            d.1,
            Some(Tolerance {
                plus: 10.0,
                minus: 6.600000000000023
            })
        );
    }
}

use crate::{
    parser,
    parser::Block,
    types::{
        calculate_addition_with_tolerance, calculate_division_with_tolerance,
        calculate_multiplication_with_tolerance, calculate_subtraction_with_tolerance,
        current::Current, power::Power, resistance::Resistance, Measurement, ParserError,
        Tolerance,
    },
};

use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voltage {
    pub value: f64,
    pub tolerance: Option<Tolerance>,
}

impl Default for Voltage {
    fn default() -> Self {
        Self {
            value: 0.0,
            tolerance: None,
        }
    }
}

impl Measurement for Voltage {
    fn get_nominal_value(&self) -> f64 {
        self.value
    }

    fn get_tolerance(&self) -> Option<Tolerance> {
        self.tolerance
    }

    fn get_unit(&self) -> &'static str {
        "V"
    }
}

impl FromStr for Voltage {
    type Err = ParserError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        if input.trim().is_empty() {
            return Err(ParserError::EmptyInput);
        }

        match parser::parse_blocks(input) {
            Ok((input, result)) => {
                // If there is any remaining unparsed input, it's an error
                if !input.is_empty() {
                    return Err(ParserError::IncorrectInput(input.to_string()));
                }

                let mut value = f64::NAN;
                let mut tol: Option<Tolerance> = None;

                // Process each parsed block
                for block in result {
                    match block {
                        Block::Number(n) => value = n,
                        Block::NumberSuffix((n, s)) => value = n * s.coefficient(),
                        Block::TolMinus(t) => {
                            tol = if let Some(tt) = tol {
                                Some(Tolerance {
                                    plus: tt.plus,
                                    minus: t,
                                })
                            } else {
                                Some(Tolerance {
                                    plus: 0.0,
                                    minus: t,
                                })
                            };
                        }
                        Block::TolPlus(t) => {
                            tol = if let Some(tt) = tol {
                                Some(Tolerance {
                                    plus: t,
                                    minus: tt.minus,
                                })
                            } else {
                                Some(Tolerance {
                                    plus: t,
                                    minus: 0.0,
                                })
                            };
                        }
                        Block::TolPlusMinus(t) => {
                            tol = Some(Tolerance { plus: t, minus: t });
                        }
                    }
                }

                Ok(Voltage {
                    value,
                    tolerance: tol,
                })
            }
            Err(e) => Err(ParserError::IncorrectInput(e.to_string())),
        }
    }
}

impl Add for Voltage {
    type Output = Voltage;

    fn add(self, rhs: Self) -> Self::Output {
        let result = calculate_addition_with_tolerance(&self, &rhs);

        Voltage {
            value: result.0,
            tolerance: result.1,
        }
    }
}

impl Sub for Voltage {
    type Output = Voltage;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = calculate_subtraction_with_tolerance(&self, &rhs);

        Voltage {
            value: result.0,
            tolerance: result.1,
        }
    }
}

impl Div<Current> for Voltage {
    type Output = Resistance;

    fn div(self, rhs: Current) -> Self::Output {
        let (value, tol) = calculate_division_with_tolerance(&self, &rhs);

        Resistance {
            value: value,
            tolerance: tol,
        }
    }
}

impl Div<Power> for Voltage {
    type Output = Resistance;

    fn div(self, rhs: Power) -> Self::Output {
        let voltage2 = calculate_multiplication_with_tolerance(&self, &self);
        let voltage2 = Voltage {
            value: voltage2.0,
            tolerance: voltage2.1,
        };
        let (value, tol) = calculate_division_with_tolerance(&voltage2, &rhs);

        Resistance {
            value: value,
            tolerance: tol,
        }
    }
}

impl Div<Resistance> for Voltage {
    type Output = Current;

    fn div(self, rhs: Resistance) -> Self::Output {
        let (value, tol) = calculate_division_with_tolerance(&self, &rhs);

        Current {
            value: value,
            tolerance: tol,
        }
    }
}

impl Mul<Current> for Voltage {
    type Output = Power;

    fn mul(self, rhs: Current) -> Self::Output {
        let (value, tol) = calculate_multiplication_with_tolerance(&self, &rhs);

        Power {
            value: value,
            tolerance: tol,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voltage_parser() {
        //assert_eq!("12p".parse::<Voltage>(), Ok(Voltage { value: 1.2e-11, tolerance: None }));
        //assert_eq!("12n".parse::<Voltage>(), Ok(Voltage { value: 12e-9, tolerance: None }));
        assert_eq!(
            "12u".parse::<Voltage>(),
            Ok(Voltage {
                value: 12e-6,
                tolerance: None
            })
        );
        assert_eq!(
            "12m".parse::<Voltage>(),
            Ok(Voltage {
                value: 12e-3,
                tolerance: None
            })
        );
        assert_eq!(
            "12".parse::<Voltage>(),
            Ok(Voltage {
                value: 12.0,
                tolerance: None
            })
        );
        assert_eq!(
            "12k".parse::<Voltage>(),
            Ok(Voltage {
                value: 12e3,
                tolerance: None
            })
        );
        assert_eq!(
            "12M".parse::<Voltage>(),
            Ok(Voltage {
                value: 12e6,
                tolerance: None
            })
        );
        assert_eq!(
            "12G".parse::<Voltage>(),
            Ok(Voltage {
                value: 12e9,
                tolerance: None
            })
        );
        assert_eq!(
            "12T".parse::<Voltage>(),
            Ok(Voltage {
                value: 12e12,
                tolerance: None
            })
        );
    }

    #[test]
    fn test_voltage_with_tolerance_parser() {
        assert_eq!(
            "12 +5%".parse::<Voltage>(),
            Ok(Voltage {
                value: 12.0,
                tolerance: Some(Tolerance {
                    plus: 5.0,
                    minus: 0.0
                })
            })
        );
        assert_eq!(
            "12 -5%".parse::<Voltage>(),
            Ok(Voltage {
                value: 12.0,
                tolerance: Some(Tolerance {
                    plus: 0.0,
                    minus: 5.0
                })
            })
        );
        assert_eq!(
            "12 5.25%".parse::<Voltage>(),
            Ok(Voltage {
                value: 12.0,
                tolerance: Some(Tolerance {
                    plus: 5.25,
                    minus: 5.25
                })
            })
        );
        assert_eq!(
            "12 +5% -3%".parse::<Voltage>(),
            Ok(Voltage {
                value: 12.0,
                tolerance: Some(Tolerance {
                    plus: 5.0,
                    minus: 3.0
                })
            })
        );
    }
}

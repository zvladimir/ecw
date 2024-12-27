use crate::types::{
    calculate_division_with_tolerance, calculate_multiplication_with_tolerance, current::Current,
    resistance::Resistance, voltage::Voltage, Measurement, ParserError, Tolerance,
};
use crate::{parser, parser::Block};
use std::{
    ops::{Div, Mul},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub struct Power {
    pub value: f64,
    pub tolerance: Option<Tolerance>,
}

impl Default for Power {
    fn default() -> Self {
        Self {
            value: 0.0,
            tolerance: None,
        }
    }
}

impl Measurement for Power {
    fn get_nominal_value(&self) -> f64 {
        self.value
    }

    fn get_tolerance(&self) -> Option<Tolerance> {
        self.tolerance
    }

    fn get_unit(&self) -> &'static str {
        "W"
    }
}

impl FromStr for Power {
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

                Ok(Power {
                    value,
                    tolerance: tol,
                })
            }
            Err(e) => Err(ParserError::IncorrectInput(e.to_string())),
        }
    }
}

impl Div<Voltage> for Power {
    type Output = Current;

    fn div(self, rhs: Voltage) -> Self::Output {
        let (value, tol) = calculate_division_with_tolerance(&self, &rhs);

        Current {
            value: value,
            tolerance: tol,
        }
    }
}

impl Div<Current> for Power {
    type Output = Resistance;

    fn div(self, rhs: Current) -> Self::Output {
        let current2 = calculate_multiplication_with_tolerance(&rhs, &rhs);
        let current2 = Current {
            value: current2.0,
            tolerance: current2.1,
        };
        let (value, tol) = calculate_division_with_tolerance(&self, &current2);

        Resistance {
            value: value,
            tolerance: tol,
        }
    }
}

impl Mul<Current> for Power {
    type Output = Voltage;

    fn mul(self, rhs: Current) -> Self::Output {
        let (value, tol) = calculate_division_with_tolerance(&self, &rhs);

        Voltage {
            value: value,
            tolerance: tol,
        }
    }
}

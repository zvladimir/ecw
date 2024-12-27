use crate::types::{
    calculate_addition_with_tolerance, calculate_division_with_tolerance,
    calculate_multiplication_with_tolerance, current::Current, power::Power, Measurement,
    ParserError, Tolerance,
};
use crate::{parser, parser::Block};
use std::{ops::Add, ops::AddAssign, ops::Mul, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct Resistance {
    pub value: f64,
    pub tolerance: Option<Tolerance>,
}

impl Default for Resistance {
    fn default() -> Self {
        Self {
            value: 0.0,
            tolerance: None,
        }
    }
}

impl Measurement for Resistance {
    fn get_nominal_value(&self) -> f64 {
        self.value
    }

    fn get_tolerance(&self) -> Option<Tolerance> {
        self.tolerance
    }

    fn get_unit(&self) -> &'static str {
        "R"
    }
}

impl FromStr for Resistance {
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

                Ok(Resistance {
                    value,
                    tolerance: tol,
                })
            }
            Err(e) => Err(ParserError::IncorrectInput(e.to_string())),
        }
    }
}

impl AddAssign for Resistance {
    fn add_assign(&mut self, rhs: Self) {
        let result = calculate_addition_with_tolerance(self, &rhs);

        self.value = result.0;
        self.tolerance = result.1;
    }
}

impl Add for Resistance {
    type Output = Resistance;

    fn add(self, rhs: Self) -> Self::Output {
        let result = calculate_addition_with_tolerance(&self, &rhs);

        Resistance {
            value: result.0,
            tolerance: result.1,
        }
    }
}

impl Mul<Current> for Resistance {
    type Output = Power;

    fn mul(self, rhs: Current) -> Self::Output {
        let current2 = calculate_multiplication_with_tolerance(&rhs, &rhs);
        let current2 = Current {
            value: current2.0,
            tolerance: current2.1,
        };
        let (value, tol) = calculate_division_with_tolerance(&current2, &self);

        Power {
            value: value,
            tolerance: tol,
        }
    }
}

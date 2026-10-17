//! The calculation of the divider by the rules of Ohm's law and Kirchhoff's voltage law, and the
//! spread of the result from the tolerances.

use uom::num_traits::Zero;
use uom::si::f64::{ElectricCurrent, ElectricPotential, ElectricalResistance};

use crate::domain::error::SolveError;
use crate::domain::infer::{Rule, Values, Var, infer, rule2, rule3, rule4};
use crate::domain::interval::Interval;
use crate::domain::quantity::Quantity;
use crate::domain::solve::{check_positive, checked_div, impl_count_given};
use crate::domain::unit::Measured;

use super::Mode;

impl_count_given!(Input {
    vin,
    vout1,
    vout2,
    r1,
    r2,
    r3
});

/// The given values of the divider, `None` for each one left empty. Vout2 and R3 are used in
/// the three-resistor mode only.
#[derive(Debug, Clone, Copy, Default)]
pub struct Input {
    pub vin: Option<Interval<ElectricPotential>>,
    pub vout1: Option<Interval<ElectricPotential>>,
    pub vout2: Option<Interval<ElectricPotential>>,
    pub r1: Option<Interval<ElectricalResistance>>,
    pub r2: Option<Interval<ElectricalResistance>>,
    pub r3: Option<Interval<ElectricalResistance>>,
}

/// All values of the divider, given or computed. Vout2 and R3 exist in the three-resistor mode
/// only. Resistor tolerances widen the voltages (and the other way round).
#[derive(Debug, Clone, Copy)]
pub struct Report {
    pub vin: Interval<ElectricPotential>,
    pub vout1: Interval<ElectricPotential>,
    pub vout2: Option<Interval<ElectricPotential>>,
    pub r1: Interval<ElectricalResistance>,
    pub r2: Interval<ElectricalResistance>,
    pub r3: Option<Interval<ElectricalResistance>>,
}

/// Solves the divider for the given values: the exact number of them that the mode needs, all
/// positive, outputs below the input in order. The result is computed at the nominal values; the
/// spread is then found from the tolerances.
pub fn solve(input: Input, mode: Mode) -> Result<Report, SolveError> {
    validate_count(&input, mode)?;
    validate_positive(&input)?;

    let nominal = solve_nominal(input, mode)?;
    Ok(widen_by_tolerance(nominal, input, mode))
}

fn solve_nominal(input: Input, mode: Mode) -> Result<Report, SolveError> {
    let mut values = Values::new(9);
    values.set(VIN, input.vin);
    values.set(VOUT1, input.vout1);
    values.set(VOUT2, input.vout2);
    values.set(R1, input.r1);
    values.set(R2, input.r2);
    values.set(R3, input.r3);
    let given = values.clone();
    check_order(&values, mode)?;
    infer(&mut values, &rules(mode))?;

    let three = mode == Mode::Three;
    let volts: &[(Var<ElectricPotential>, &str)] = if three {
        &[(VIN, "Vin"), (VOUT1, "Vout1"), (VOUT2, "Vout2")]
    } else {
        &[(VIN, "Vin"), (VOUT1, "Vout")]
    };
    let ohms: &[(Var<ElectricalResistance>, &str)] = if three {
        &[(R1, "R1"), (R2, "R2"), (R3, "R3")]
    } else {
        &[(R1, "R1"), (R2, "R2")]
    };
    // A quantity that was computed must be physical.
    for &(var, name) in volts {
        computed_positive(&values, &given, var, name)?;
    }
    for &(var, name) in ohms {
        computed_positive(&values, &given, var, name)?;
    }
    let get = |var| values.get(var).ok_or(SolveError::Underdetermined);
    let ohm = |var| values.get(var).ok_or(SolveError::Underdetermined);
    if three {
        require_less(get(VOUT2)?, get(VOUT1)?, "Vout2", "Vout1")?;
        require_less(get(VOUT1)?, get(VIN)?, "Vout1", "Vin")?;
    }

    Ok(Report {
        vin: get(VIN)?,
        vout1: get(VOUT1)?,
        vout2: if three { Some(get(VOUT2)?) } else { None },
        r1: ohm(R1)?,
        r2: ohm(R2)?,
        r3: if three { Some(ohm(R3)?) } else { None },
    })
}

/// A value that was not given but computed must be positive.
fn computed_positive<T: Measured + Zero>(
    values: &Values,
    given: &Values,
    var: Var<T>,
    name: &str,
) -> Result<(), SolveError> {
    if !given.is_known(var)
        && values.get(var).ok_or(SolveError::Underdetermined)?.nominal <= T::zero()
    {
        return Err(SolveError::ComputedNotPositive {
            name: name.to_owned().into(),
        });
    }
    Ok(())
}

// The quantities. `I` is the series current, `D1` and `D2` the drops across R1 and R2 (the
// drop across R3 is Vout2 itself).
const VIN: Var<ElectricPotential> = Var::new(0);
const VOUT1: Var<ElectricPotential> = Var::new(1);
const VOUT2: Var<ElectricPotential> = Var::new(2);
const R1: Var<ElectricalResistance> = Var::new(3);
const R2: Var<ElectricalResistance> = Var::new(4);
const R3: Var<ElectricalResistance> = Var::new(5);
const I: Var<ElectricCurrent> = Var::new(6);
const D1: Var<ElectricPotential> = Var::new(7);
const D2: Var<ElectricPotential> = Var::new(8);

/// Drop = current · resistance, in its three forms.
macro_rules! ohm {
    ($drop:expr, $resistor:expr) => {
        [
            rule2(I, $drop, $resistor, |d, r| {
                checked_div(d, r, Quantity::Resistance)
            }),
            rule2($resistor, $drop, I, |d, i| {
                checked_div(d, i, Quantity::Current)
            }),
            rule2($drop, I, $resistor, |i, r| Ok(i * r)),
        ]
    };
}

/// `total = a + b`, in its three forms.
macro_rules! sum {
    ($total:expr, $a:expr, $b:expr) => {
        [
            rule2($total, $a, $b, |a, b| Ok(a + b)),
            rule2($a, $total, $b, |t, b| Ok(t - b)),
            rule2($b, $total, $a, |t, a| Ok(t - a)),
        ]
    };
}

/// How each quantity follows from the others. The current is found from the drop across a
/// known resistor, or from the few combinations below that fix it without one.
fn rules(mode: Mode) -> Vec<Rule> {
    let mut rules = Vec::new();
    match mode {
        Mode::Two => {
            rules.push(rule3(I, VIN, R1, R2, |v, a, b| {
                checked_div(v, a + b, Quantity::Resistance)
            }));
            rules.extend(sum!(VIN, VOUT1, D1));
            rules.extend(ohm!(D1, R1));
            rules.extend(ohm!(VOUT1, R2));
        }
        Mode::Three => {
            // The current from Vin and all resistors, from Vout1 and R2 + R3, and from the
            // span Vin − Vout2 across R1 + R2.
            rules.push(rule4(I, VIN, R1, R2, R3, |v, a, b, c| {
                checked_div(v, a + b + c, Quantity::Resistance)
            }));
            rules.push(rule3(I, VOUT1, R2, R3, |v, b, c| {
                checked_div(v, b + c, Quantity::Resistance)
            }));
            rules.push(rule4(I, VIN, VOUT2, R1, R2, |v, o, a, b| {
                checked_div(v - o, a + b, Quantity::Resistance)
            }));
            rules.extend(sum!(VIN, VOUT1, D1));
            rules.extend(sum!(VOUT1, VOUT2, D2));
            rules.extend(ohm!(D1, R1));
            rules.extend(ohm!(D2, R2));
            rules.extend(ohm!(VOUT2, R3));
        }
    }
    rules
}

/// The outputs must lie below the input, in order. Checked on the given values first, so
/// that a mistake in the data is named as such and not as a nonsensical result.
fn check_order(values: &Values, mode: Mode) -> Result<(), SolveError> {
    let pairs: &[(
        Var<ElectricPotential>,
        Var<ElectricPotential>,
        &'static str,
        &'static str,
    )] = match mode {
        Mode::Two => &[(VOUT1, VIN, "Vout", "Vin")],
        Mode::Three => &[
            (VOUT1, VIN, "Vout1", "Vin"),
            (VOUT2, VIN, "Vout2", "Vin"),
            (VOUT2, VOUT1, "Vout2", "Vout1"),
        ],
    };
    for &(low, high, low_name, high_name) in pairs {
        if let (Some(low), Some(high)) = (values.get(low), values.get(high)) {
            require_less(low, high, low_name, high_name)?;
        }
    }
    Ok(())
}

fn require_less(
    left: Interval<ElectricPotential>,
    right: Interval<ElectricPotential>,
    left_name: &'static str,
    right_name: &'static str,
) -> Result<(), SolveError> {
    if left.nominal >= right.nominal {
        return Err(SolveError::Ordered {
            left: left_name.into(),
            right: right_name.into(),
        });
    }
    Ok(())
}

/// The lowest and the highest value seen.
#[derive(Clone, Copy)]
struct Span<T>(T, T);

impl<T: Copy + PartialOrd> Span<T> {
    fn of(iv: Interval<T>) -> Self {
        Span(iv.nominal, iv.nominal)
    }

    fn widen(&mut self, x: T) {
        if x < self.0 {
            self.0 = x;
        }
        if x > self.1 {
            self.1 = x;
        }
    }

    fn around(self, nominal: T) -> Interval<T> {
        Interval::new(nominal, self.0, self.1)
    }
}

/// One end of a toleranced value, as an exact one.
fn end<T: Copy>(iv: Option<Interval<T>>, upper: bool) -> Option<Interval<T>> {
    iv.map(|i| Interval::exact(if upper { i.hi } else { i.lo }))
}

/// Exact range of every result. The formulas use some inputs twice (`R2` in
/// `Vin·R2/(R1+R2)`), so plain interval arithmetic is too pessimistic. Every result is
/// monotonic in each input, so its extremes are at the corners of the input box.
fn widen_by_tolerance(nominal: Report, input: Input, mode: Mode) -> Report {
    let volts = [input.vin, input.vout1, input.vout2];
    let ohms = [input.r1, input.r2, input.r3];
    // The inputs with a tolerance, by their place: 0…2 the voltages, 3…5 the resistors.
    let free: Vec<usize> = (0..6)
        .filter(|&k| {
            if k < 3 {
                volts[k].is_some_and(|i| i.lo != i.hi)
            } else {
                ohms[k - 3].is_some_and(|i| i.lo != i.hi)
            }
        })
        .collect();
    if free.is_empty() {
        return nominal;
    }

    let mut v = [
        Span::of(nominal.vin),
        Span::of(nominal.vout1),
        nominal.vout2.map_or(Span::of(nominal.vin), Span::of),
    ];
    let mut r = [
        Span::of(nominal.r1),
        Span::of(nominal.r2),
        nominal.r3.map_or(Span::of(nominal.r1), Span::of),
    ];

    for mask in 0..(1u32 << free.len()) {
        let upper = |k: usize| {
            free.iter()
                .position(|&f| f == k)
                .is_some_and(|b| mask >> b & 1 == 1)
        };
        let corner = Input {
            vin: end(input.vin, upper(0)),
            vout1: end(input.vout1, upper(1)),
            vout2: end(input.vout2, upper(2)),
            r1: end(input.r1, upper(3)),
            r2: end(input.r2, upper(4)),
            r3: end(input.r3, upper(5)),
        };
        let Ok(report) = solve_nominal(corner, mode) else {
            continue;
        };
        for (span, value) in v
            .iter_mut()
            .zip([Some(report.vin), Some(report.vout1), report.vout2])
        {
            if let Some(x) = value {
                span.widen(x.nominal);
            }
        }
        for (span, value) in r
            .iter_mut()
            .zip([Some(report.r1), Some(report.r2), report.r3])
        {
            if let Some(x) = value {
                span.widen(x.nominal);
            }
        }
    }

    Report {
        vin: v[0].around(nominal.vin.nominal),
        vout1: v[1].around(nominal.vout1.nominal),
        vout2: nominal.vout2.map(|o| v[2].around(o.nominal)),
        r1: r[0].around(nominal.r1.nominal),
        r2: r[1].around(nominal.r2.nominal),
        r3: nominal.r3.map(|o| r[2].around(o.nominal)),
    }
}

fn validate_count(input: &Input, mode: Mode) -> Result<(), SolveError> {
    let given = input.count_given();
    let needed = mode.needed();
    if given != needed {
        return Err(SolveError::WrongCount {
            expected: needed,
            got: given,
        });
    }
    Ok(())
}

fn validate_positive(input: &Input) -> Result<(), SolveError> {
    check_positive(input.vin, Quantity::Voltage)?;
    check_positive(input.vout1, Quantity::Voltage)?;
    check_positive(input.vout2, Quantity::Voltage)?;
    check_positive(input.r1, Quantity::Resistance)?;
    check_positive(input.r2, Quantity::Resistance)?;
    check_positive(input.r3, Quantity::Resistance)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::electric_potential::volt;
    use uom::si::electrical_resistance::ohm;

    fn v(x: f64) -> Option<Interval<ElectricPotential>> {
        Some(Interval::exact(ElectricPotential::new::<volt>(x)))
    }

    fn r(x: f64) -> Option<Interval<ElectricalResistance>> {
        Some(Interval::exact(ElectricalResistance::new::<ohm>(x)))
    }

    fn r_tol(x: f64, tol: f64) -> Option<Interval<ElectricalResistance>> {
        let at = |k: f64| ElectricalResistance::new::<ohm>(x * k);
        Some(Interval::new(at(1.0), at(1.0 - tol), at(1.0 + tol)))
    }

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9 * b.abs().max(1.0), "{a} vs {b}");
    }

    #[test]
    fn three_resistors_from_both_outputs_and_two_resistors() {
        // Vin 12, R1 10k, R2 5k, R3 2k: Vout1 = 12·7/17, Vout2 = 12·2/17.
        let (v1, v2) = (12.0 * 7.0 / 17.0, 12.0 * 2.0 / 17.0);
        let base = Input {
            vout1: v(v1),
            vout2: v(v2),
            ..Input::default()
        };

        let a = solve(
            Input {
                r1: r(10e3),
                r2: r(5e3),
                ..base
            },
            Mode::Three,
        )
        .unwrap();
        close(a.vin.nominal.to_base(), 12.0);
        close(a.r3.unwrap().nominal.to_base(), 2e3);

        let b = solve(
            Input {
                r1: r(10e3),
                r3: r(2e3),
                ..base
            },
            Mode::Three,
        )
        .unwrap();
        close(b.vin.nominal.to_base(), 12.0);
        close(b.r2.nominal.to_base(), 5e3);
    }

    #[test]
    fn outputs_with_r2_and_r3_leave_vin_and_r1_free() {
        let input = Input {
            vout1: v(4.0),
            vout2: v(1.0),
            r2: r(3e3),
            r3: r(1e3),
            ..Input::default()
        };
        assert_eq!(
            solve(input, Mode::Three).unwrap_err(),
            SolveError::Underdetermined
        );
    }

    #[test]
    fn tolerance_range_is_exact_not_pessimistic() {
        // Vout = Vin·R2/(R1+R2): R2 is used twice, the true range is narrower than
        // interval arithmetic gives.
        let input = Input {
            vin: v(12.0),
            r1: r_tol(10e3, 0.01),
            r2: r_tol(5e3, 0.05),
            ..Input::default()
        };
        let out = solve(input, Mode::Two).unwrap().vout1;
        close(out.lo.to_base(), 12.0 * 4750.0 / (4750.0 + 10100.0));
        close(out.hi.to_base(), 12.0 * 5250.0 / (5250.0 + 9900.0));
    }
}

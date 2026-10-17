//! The calculation of Ohm's law: every quantity from any two.

use uom::si::f64::{
    ElectricCurrent, ElectricPotential, ElectricalConductance, ElectricalResistance, Power,
};

use crate::domain::error::SolveError;
use crate::domain::infer::{Rule, Values, Var, infer, rule2};
use crate::domain::interval::Interval;
use crate::domain::quantity::Quantity;
use crate::domain::solve::{check_positive, checked_div, impl_count_given};

impl_count_given!(Input { u, i, r, p });

/// The request: exactly two of the four quantities are given, the other two are `None`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Input {
    pub u: Option<Interval<ElectricPotential>>,

    pub i: Option<Interval<ElectricCurrent>>,

    pub r: Option<Interval<ElectricalResistance>>,

    pub p: Option<Interval<Power>>,
}

/// All quantities of the circuit, given or computed, with the spread of their tolerances.
#[derive(Debug, Clone, Copy)]
pub struct Report {
    pub u: Interval<ElectricPotential>,
    pub i: Interval<ElectricCurrent>,
    pub r: Interval<ElectricalResistance>,
    /// The conductance `G = I/U`.
    pub g: Interval<ElectricalConductance>,
    pub p: Interval<Power>,
}

const U: Var<ElectricPotential> = Var::new(0);
const I: Var<ElectricCurrent> = Var::new(1);
const R: Var<ElectricalResistance> = Var::new(2);
const P: Var<Power> = Var::new(3);
const G: Var<ElectricalConductance> = Var::new(4);

/// Every way a quantity follows from two others, in the order they are tried.
fn rules() -> Vec<Rule> {
    vec![
        rule2(U, I, R, |i, r| Ok(i * r)),
        rule2(I, U, R, |u, r| checked_div(u, r, Quantity::Resistance)),
        rule2(R, U, I, |u, i| checked_div(u, i, Quantity::Current)),
        rule2(P, U, I, |u, i| Ok(u * i)),
        rule2(I, P, U, |p, u| checked_div(p, u, Quantity::Voltage)),
        rule2(U, P, I, |p, i| checked_div(p, i, Quantity::Current)),
        // P = U²/R = I²·R
        rule2(U, P, R, |p, r| {
            (p * r)
                .try_sqrt(|x| x.sqrt())
                .ok_or(SolveError::NotPositive(Quantity::Power))
        }),
        rule2(I, P, R, |p, r| {
            checked_div(p, r, Quantity::Resistance)?
                .try_sqrt(|x| x.sqrt())
                .ok_or(SolveError::NotPositive(Quantity::Power))
        }),
        rule2(G, I, U, |i, u| checked_div(i, u, Quantity::Voltage)),
    ]
}

/// Finds U, I, R, P and G from two given quantities.
pub fn solve(input: Input) -> Result<Report, SolveError> {
    validate(&input)?;

    let mut values = Values::new(5);
    values.set(U, input.u);
    values.set(I, input.i);
    values.set(R, input.r);
    values.set(P, input.p);
    infer(&mut values, &rules())?;

    let two = "two inputs determine every quantity";
    Ok(Report {
        u: values.get(U).expect(two),
        i: values.get(I).expect(two),
        r: values.get(R).expect(two),
        g: values.get(G).expect(two),
        p: values.get(P).expect(two),
    })
}

fn validate(input: &Input) -> Result<(), SolveError> {
    let given = input.count_given();
    if given != 2 {
        return Err(SolveError::WrongCount {
            expected: 2,
            got: given,
        });
    }

    check_positive(input.u, Quantity::Voltage)?;
    check_positive(input.i, Quantity::Current)?;
    check_positive(input.r, Quantity::Resistance)?;
    check_positive(input.p, Quantity::Power)?;

    Ok(())
}

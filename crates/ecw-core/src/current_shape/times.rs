//! Triangular waveforms take any three of t1, t2, t3 and T; the fourth follows.

use uom::si::f64::Time;

use crate::domain::FieldSet;
use crate::domain::infer::{Rule, Values, Var as Quantity, infer, rule3};
use crate::domain::interval::Interval;

use super::Var;

/// The four times of a triangle: any three of them give the fourth.
pub const TIME_VARS: [Var; 4] = [Var::T1, Var::T2, Var::T3, Var::Period];

/// All four times of a triangle.
#[derive(Debug, Clone, Copy)]
pub struct Times {
    pub t1: Time,
    pub t2: Time,
    pub t3: Time,
    pub period: Time,
}

impl Times {
    /// The time that belongs to `var`; `None` for a field that is not a time.
    pub fn get(&self, var: Var) -> Option<Time> {
        match var {
            Var::T1 => Some(self.t1),
            Var::T2 => Some(self.t2),
            Var::T3 => Some(self.t3),
            Var::Period => Some(self.period),
            _ => None,
        }
    }
}

const T1: Quantity<Time> = Quantity::new(0);
const T2: Quantity<Time> = Quantity::new(1);
const T3: Quantity<Time> = Quantity::new(2);
const PERIOD: Quantity<Time> = Quantity::new(3);

/// `T = t1 + t2 + t3`, in its four forms.
fn rules() -> Vec<Rule> {
    vec![
        rule3(PERIOD, T1, T2, T3, |a, b, c| Ok(a + b + c)),
        rule3(T3, PERIOD, T1, T2, |p, a, b| Ok(p - a - b)),
        rule3(T2, PERIOD, T1, T3, |p, a, c| Ok(p - a - c)),
        rule3(T1, PERIOD, T2, T3, |p, b, c| Ok(p - b - c)),
    ]
}

/// How many of the four times are given.
pub fn filled_count(fields: &FieldSet<Var>) -> usize {
    TIME_VARS.iter().filter(|&&var| fields.is_ok(var)).count()
}

/// The one missing time, if exactly three are given.
pub fn missing_var(fields: &FieldSet<Var>) -> Option<Var> {
    if filled_count(fields) != TIME_VARS.len() - 1 {
        return None;
    }
    TIME_VARS.into_iter().find(|&var| !fields.is_ok(var))
}

/// The nominal value of a time field.
fn given(fields: &FieldSet<Var>, var: Var) -> Option<Time> {
    fields.interval::<Time>(var).map(|iv| iv.nominal)
}

/// All four times are given and t1 + t2 + t3 is not the period.
pub fn is_inconsistent(fields: &FieldSet<Var>) -> bool {
    match (
        given(fields, Var::T1),
        given(fields, Var::T2),
        given(fields, Var::T3),
        given(fields, Var::Period),
    ) {
        (Some(a), Some(b), Some(c), Some(p)) => {
            (a + b + c - p).abs() > (a.abs() + b.abs() + c.abs()).max(p.abs()) * 1e-9
        }
        _ => false,
    }
}

/// All four times from the given ones (nominal values), if three or four are known.
pub fn resolve(fields: &FieldSet<Var>) -> Option<Times> {
    let mut values = Values::new(4);
    for (quantity, var) in [
        (T1, Var::T1),
        (T2, Var::T2),
        (T3, Var::T3),
        (PERIOD, Var::Period),
    ] {
        values.set(quantity, given(fields, var).map(Interval::exact));
    }
    infer(&mut values, &rules()).ok()?;
    let get = |quantity| values.get(quantity).map(|iv| iv.nominal);
    Some(Times {
        t1: get(T1)?,
        t2: get(T2)?,
        t3: get(T3)?,
        period: get(PERIOD)?,
    })
}

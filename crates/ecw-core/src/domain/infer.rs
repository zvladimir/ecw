//! "Any N of M known, find the rest": a calculator lists the ways a quantity follows from
//! others and `infer` applies them until nothing new can be derived.
//!
//! Each quantity is a typed variable (`Var<ElectricPotential>`), and a rule is a typed function
//! of its inputs, so the units of every formula are checked by the compiler. A rule fires when
//! its target is unknown and every input is known. Rules are tried in the order listed, and
//! each fires at once, so a later rule in the same pass can build on an earlier one. Where two
//! routes lead to the same quantity the first one in the list wins, which also fixes the
//! interval arithmetic that is used.

use std::marker::PhantomData;

use crate::domain::error::SolveError;
use crate::domain::interval::Interval;
use crate::domain::unit::Measured;

/// A quantity of a calculator: its place among the values and its type.
pub struct Var<T> {
    index: usize,
    kind: PhantomData<fn() -> T>,
}

impl<T> Var<T> {
    /// The quantity number `index` of the calculator; the numbers are the calculator's own.
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            kind: PhantomData,
        }
    }
}

impl<T> Clone for Var<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Var<T> {}

/// The values of a calculator's quantities, known or not. They are stored in base units, which
/// only `get` and `set` see.
#[derive(Debug, Clone)]
pub struct Values(Vec<Option<Interval<f64>>>);

impl Values {
    /// A store for `count` quantities, all unknown.
    pub fn new(count: usize) -> Self {
        Self(vec![None; count])
    }

    /// The value of a quantity, if known.
    pub fn get<T: Measured>(&self, var: Var<T>) -> Option<Interval<T>> {
        self.0[var.index].map(|iv| iv.map(T::from_base))
    }

    /// Sets or clears a value.
    pub fn set<T: Measured>(&mut self, var: Var<T>, value: Option<Interval<T>>) {
        self.0[var.index] = value.map(|iv| iv.map(T::to_base));
    }

    /// Sets a value known exactly (a nominal value without tolerance), or unknown.
    pub fn set_exact<T: Measured>(&mut self, var: Var<T>, value: Option<T>) {
        self.set(var, value.map(Interval::exact));
    }

    /// The nominal value of a quantity, if known.
    pub fn nominal<T: Measured>(&self, var: Var<T>) -> Option<T> {
        self.get(var).map(|iv| iv.nominal)
    }

    /// True if the quantity has a value.
    pub fn is_known<T>(&self, var: Var<T>) -> bool {
        self.0[var.index].is_some()
    }
}

type Apply = Box<dyn Fn(&Values) -> Option<Result<Interval<f64>, SolveError>>>;

/// One way to compute a quantity; build it with `rule1` … `rule4` or `when1` … `when4`.
pub struct Rule {
    target: usize,
    apply: Apply,
}

macro_rules! rule_n {
    ($name:ident, $when:ident, $($arg:ident: $ty:ident),+) => {
        /// `target` is computed by `f` from the given quantities, once all of them are known.
        pub fn $name<T: Measured + 'static, $($ty: Measured + 'static),+>(
            target: Var<T>,
            $($arg: Var<$ty>,)+
            f: fn($(Interval<$ty>),+) -> Result<Interval<T>, SolveError>,
        ) -> Rule {
            Rule {
                target: target.index,
                apply: Box::new(move |values| {
                    Some(f($(values.get($arg)?),+).map(|iv| iv.map(T::to_base)))
                }),
            }
        }

        /// Like the plain rule, but `f` may decline (`None`) when its formula does not apply to
        /// these values, for example a division by zero; another rule may still find the target.
        pub fn $when<T: Measured + 'static, $($ty: Measured + 'static),+>(
            target: Var<T>,
            $($arg: Var<$ty>,)+
            f: fn($(Interval<$ty>),+) -> Option<Interval<T>>,
        ) -> Rule {
            Rule {
                target: target.index,
                apply: Box::new(move |values| {
                    f($(values.get($arg)?),+).map(|iv| Ok(iv.map(T::to_base)))
                }),
            }
        }
    };
}

rule_n!(rule1, when1, a: A);
rule_n!(rule2, when2, a: A, b: B);
rule_n!(rule3, when3, a: A, b: B, c: C);
rule_n!(rule4, when4, a: A, b: B, c: C, d: D);

/// Derives every unknown value that the rules can reach.
pub fn infer(values: &mut Values, rules: &[Rule]) -> Result<(), SolveError> {
    loop {
        let mut changed = false;
        for rule in rules {
            if values.0[rule.target].is_some() {
                continue;
            }
            if let Some(result) = (rule.apply)(values) {
                values.0[rule.target] = Some(result?);
                changed = true;
            }
        }
        if !changed {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::quantity::Quantity;
    use crate::domain::solve::checked_div;
    use uom::si::electric_current::ampere;
    use uom::si::electric_potential::volt;
    use uom::si::electrical_resistance::ohm;
    use uom::si::f64::{ElectricCurrent, ElectricPotential, ElectricalResistance};

    const U: Var<ElectricPotential> = Var::new(0);
    const I: Var<ElectricCurrent> = Var::new(1);
    const R: Var<ElectricalResistance> = Var::new(2);

    // U = I·R in the three ways.
    fn rules() -> Vec<Rule> {
        vec![
            rule2(U, I, R, |i, r| Ok(i * r)),
            rule2(R, U, I, |u, i| checked_div(u, i, Quantity::Current)),
            rule2(I, U, R, |u, r| checked_div(u, r, Quantity::Resistance)),
        ]
    }

    fn exact<T: Measured>(v: f64) -> Option<Interval<T>> {
        Some(Interval::exact(T::from_base(v)))
    }

    #[test]
    fn any_two_give_the_third() {
        let mut v = Values::new(3);
        v.set(I, exact(2.0));
        v.set(U, exact(6.0));
        infer(&mut v, &rules()).unwrap();
        assert_eq!(v.get(R).unwrap().nominal.get::<ohm>(), 3.0);

        let mut v = Values::new(3);
        v.set(R, exact(2.0));
        v.set(U, exact(6.0));
        infer(&mut v, &rules()).unwrap();
        assert_eq!(v.get(I).unwrap().nominal.get::<ampere>(), 3.0);
        assert_eq!(v.get(U).unwrap().nominal.get::<volt>(), 6.0);
    }

    #[test]
    fn a_rule_that_declines_leaves_the_target_to_others() {
        let rules = vec![
            when2(R, U, I, |u, i| {
                (i.nominal.get::<ampere>() != 0.0).then(|| (u / i).unwrap())
            }),
            rule2(I, U, R, |u, r| checked_div(u, r, Quantity::Resistance)),
        ];
        let mut v = Values::new(3);
        v.set(I, exact(0.0));
        v.set(U, exact(6.0));
        infer(&mut v, &rules).unwrap();
        assert!(!v.is_known(R));
    }

    #[test]
    fn errors_stop_the_run() {
        let mut v = Values::new(3);
        v.set(I, exact(0.0));
        v.set(U, exact(6.0));
        assert!(matches!(
            infer(&mut v, &rules()),
            Err(SolveError::NotPositive(Quantity::Current))
        ));
    }
}

//! Ohm's Law page: four fields (U, I, R, P); any two give the others, which show the result.

use crate::msg;
use super::Form;
use serde::Serialize;

use super::{FieldInfo, FormField, Outcome, Table, breakdown_table, form_field};
use crate::domain::format::breakdown;
use crate::domain::{FieldSet, FieldState, Quantity};
use crate::ohms_law::calc::{self, Input};

const REQUIRED_FIELDS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Var {
    U,
    I,
    R,
    P,
}

impl Var {
    const ALL: [Var; 4] = [Var::U, Var::I, Var::R, Var::P];

    fn key(self) -> &'static str {
        match self {
            Var::U => "u",
            Var::I => "i",
            Var::R => "r",
            Var::P => "p",
        }
    }

    fn quantity(self) -> Quantity {
        match self {
            Var::U => Quantity::Voltage,
            Var::I => Quantity::Current,
            Var::R => Quantity::Resistance,
            Var::P => Quantity::Power,
        }
    }
}

/// The reply of the Ohm's Law page.
#[derive(Debug, Serialize)]
pub struct OhmsResponse {
    /// The input fields with their state: parse result, hint, and the computed value of a locked field.
    pub fields: Vec<FormField>,
    /// What to show in the results area.
    pub outcome: Outcome,
    /// Every quantity with its spread, once the page is solved.
    pub table: Option<Table>,
}

/// Parses the form and solves it when exactly two fields are given; the other fields are
/// locked and the results table shows all quantities with their spread.
pub fn solve(req: &Form) -> OhmsResponse {
    let d = req.decimals;
    let fields = FieldSet::collect(&Var::ALL, |var| {
        FieldState::parse(req.text(var.key()), var.quantity())
    });

    let result = (!fields.has_error() && fields.valid_count() == REQUIRED_FIELDS).then(|| {
        calc::solve(Input {
            u: fields.interval(Var::U),
            i: fields.interval(Var::I),
            r: fields.interval(Var::R),
            p: fields.interval(Var::P),
        })
    });

    let blocked = match result {
        Some(Ok(_)) => fields.unfilled(&Var::ALL),
        _ => Vec::new(),
    };

    let form = Var::ALL
        .iter()
        .map(|&var| {
            let q = var.quantity();
            let info = FieldInfo::input(fields.state(var), q.unit(), d, blocked.contains(&var));
            form_field(var.key(), q.label(), q.input_hint(), info)
        })
        .collect();

    let table = match &result {
        Some(Ok(r)) => Some(breakdown_table(&[
            ("U [V]", breakdown(r.u, d)),
            ("I [A]", breakdown(r.i, d)),
            ("R [Ω]", breakdown(r.r, d)),
            ("G [S]", breakdown(r.g, d)),
            ("P [W]", breakdown(r.p, d)),
        ])),
        _ => None,
    };

    let n = fields.valid_count();
    let outcome = Outcome::by_count(result.as_ref(), fields.has_error(), n, REQUIRED_FIELDS, |given| {
        match given {
            0 => msg!("ohms_law.enter_two"),
            _ => msg!("ohms_law.enter_one_more"),
        }
    });

    OhmsResponse {
        fields: form,
        outcome,
        table,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(u: &str, i: &str, r: &str, p: &str) -> Form {
        Form::new(4, &[("u", u), ("i", i), ("r", r), ("p", p)])
    }

    #[test]
    fn two_inputs_solve_and_block_the_rest() {
        let resp = solve(&req("12", "", "4", ""));
        assert!(matches!(resp.outcome, Outcome::Solved));
        assert!(resp.fields[1].info.computed.is_some());
        assert!(resp.fields[3].info.computed.is_some());
        assert!(resp.fields[0].info.computed.is_none());
        assert_eq!(resp.table.unwrap().rows.len(), 7);
    }

    #[test]
    fn three_inputs_is_too_many() {
        let resp = solve(&req("12", "3", "4", ""));
        assert!(matches!(resp.outcome, Outcome::TooMany(_)));
    }

    #[test]
    fn one_input_asks_for_more() {
        let resp = solve(&req("12", "", "", ""));
        assert!(matches!(resp.outcome, Outcome::Pending(_)));
    }
}

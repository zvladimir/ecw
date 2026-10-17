//! Voltage Divider page: two or three resistors, the fields of the chosen mode and the
//! table of the values with their spread.

use crate::msg;
use super::Form;
use serde::Serialize;

use super::{FieldInfo, FormField, Outcome, Table, breakdown_table, form_field};
use crate::domain::format::{Breakdown, breakdown};
use crate::domain::{FieldSet, FieldState};
use crate::voltage_divider::calc::{self, Input};
use crate::voltage_divider::{Mode, Var, vars_for};

/// The reply of the Voltage Divider page.
#[derive(Debug, Serialize)]
pub struct DividerResponse {
    /// The input fields with their state: parse result, hint, and the computed value of a locked field.
    pub fields: Vec<FormField>,
    /// What to show in the results area.
    pub outcome: Outcome,
    /// Every quantity with its spread, once the page is solved.
    pub table: Option<Table>,
}

/// Parses the form for the chosen mode, solves it when the right number of fields is given and
/// lays out the page.
pub fn solve(req: &Form) -> DividerResponse {
    let d = req.decimals;
    let mode = Mode::parse(req.text("mode"));
    let vars = vars_for(mode);

    let fields = FieldSet::collect(vars, |var| FieldState::parse(req.text(var.key()), var.quantity()));
    let needed = mode.needed();

    let result = (!fields.has_error() && fields.valid_count() == needed).then(|| {
        calc::solve(
            Input {
                vin: fields.interval(Var::Vin),
                vout1: fields.interval(Var::Vout1),
                vout2: fields.interval(Var::Vout2),
                r1: fields.interval(Var::R1),
                r2: fields.interval(Var::R2),
                r3: fields.interval(Var::R3),
            },
            mode,
        )
    });
    let blocked = match result {
        Some(Ok(_)) => fields.unfilled(vars),
        _ => Vec::new(),
    };

    let form = vars
        .iter()
        .map(|&var| {
            let info = FieldInfo::input(fields.state(var), var.unit(), d, blocked.contains(&var));
            form_field(
                var.key(),
                var.label(mode),
                var.quantity().input_hint(),
                info,
            )
        })
        .collect();

    let table = match &result {
        Some(Ok(r)) => {
            let mut items: Vec<(&str, Breakdown)> = vec![
                ("Vin [V]", breakdown(r.vin, d)),
                ("Vout1 [V]", breakdown(r.vout1, d)),
            ];
            if let Some(v) = r.vout2 {
                items.push(("Vout2 [V]", breakdown(v, d)));
            }
            items.push(("R1 [Ω]", breakdown(r.r1, d)));
            items.push(("R2 [Ω]", breakdown(r.r2, d)));
            if let Some(r3) = r.r3 {
                items.push(("R3 [Ω]", breakdown(r3, d)));
            }
            Some(breakdown_table(&items))
        }
        _ => None,
    };

    let n = fields.valid_count();
    let outcome = Outcome::by_count(result.as_ref(), fields.has_error(), n, needed, |given| {
        msg!("voltage_divider.provide", needed = needed, total = vars.len(), given = given)
    });

    DividerResponse {
        fields: form,
        outcome,
        table,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(mode: &str) -> Form {
        Form::new(4, &[("mode", mode)])
    }

    #[test]
    fn two_resistor_divider() {
        let mut r = req("two");
        r.set("vin", "12");
        r.set("r1", "10k");
        r.set("r2", "10k");
        let resp = solve(&r);
        assert!(matches!(resp.outcome, Outcome::Solved));
        assert_eq!(resp.fields.len(), 4);
        assert_eq!(resp.fields[1].info.computed.as_deref(), Some(""));
    }

    #[test]
    fn three_mode_lists_six_fields_and_asks_for_four() {
        let mut r = req("three");
        r.set("vin", "12");
        let resp = solve(&r);
        assert_eq!(resp.fields.len(), 6);
        assert!(matches!(resp.outcome, Outcome::Pending(_)));
    }
}

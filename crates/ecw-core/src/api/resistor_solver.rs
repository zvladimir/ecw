//! Resistor Solver page: the target, the E-series and the user's own resistors, and the table
//! of the closest single resistors and pairs.

use crate::msg;
use super::Form;
use serde::Serialize;
use uom::si::electrical_resistance::ohm;
use uom::si::f64::ElectricalResistance;

use super::{FieldInfo, Outcome};
use crate::domain::format::format_engineering;
use crate::domain::unit::Measured;
use crate::domain::{ESeries, FieldState, Quantity, Text};
use crate::resistor_solver::calc::{self, Report};

/// One answer: the resistors as text, how far it is from the target, and whether the deviation
/// is within the tolerance of the series.
#[derive(Debug, Serialize)]
pub struct ResistorCell {
    /// The answer as text: one resistor, or a pair with its total (`a + b = sum`, `a ‖ b = sum`).
    pub text: String,
    /// How far it is from the target, as text.
    pub deviation: String,
    /// True if the deviation is within the tolerance of the series.
    pub in_tolerance: bool,
}

/// A row of the answers table: its caption and a cell for each column (`None` if there is
/// none).
#[derive(Debug, Serialize)]
pub struct ResistorRow {
    /// The caption of the row.
    pub label: Text,
    /// The answers of the row, one per column; `None` where there is none.
    pub cells: Vec<Option<ResistorCell>>,
}

/// The reply of the Resistor Solver page.
#[derive(Debug, Serialize)]
pub struct ResistorResponse {
    /// The names of the E-series for the selector.
    pub series_options: Vec<&'static str>,
    /// An example for the target field.
    pub target_hint: Text,
    /// The target field.
    pub target: FieldInfo,
    /// What to show in the results area.
    pub outcome: Outcome,
    /// The captions of the columns.
    pub header: Vec<Text>,
    /// The rows of the table.
    pub rows: Vec<ResistorRow>,
}

/// Finds the closest resistors to the target and lays out the table.
pub fn solve(req: &Form) -> ResistorResponse {
    let d = req.decimals;
    let eseries = ESeries::ALL
        .iter()
        .copied()
        .find(|s| s.name() == req.text("series"))
        .unwrap_or_default();

    let state = FieldState::parse_exact(req.text("target"), Quantity::Resistance);
    let target_info = FieldInfo::from_state(&state, Quantity::Resistance.unit(), d, true);

    let mut resp = ResistorResponse {
        series_options: ESeries::ALL.iter().map(|s| s.name()).collect(),
        target_hint: msg!("resistor_solver.target_hint"),
        target: target_info,
        outcome: Outcome::Solved,
        header: Vec::new(),
        rows: Vec::new(),
    };

    let Some(measure) = state.measure() else {
        resp.outcome = if state.has_error() {
            Outcome::FixErrors(msg!("resistor_solver.fix_target"))
        } else {
            Outcome::Pending(msg!("resistor_solver.enter_target"))
        };
        return resp;
    };

    let target = ElectricalResistance::new::<ohm>(measure.nominal);
    let pool = calc::parse_pool(req.text("pool"));
    let report = calc::solve(target, eseries, &pool);

    resp.header = vec![
        Text::default(),
        msg!("resistor_solver.standard_series", series = eseries.name()),
        msg!("resistor_solver.custom_pool"),
    ];
    resp.rows = rows(&report, target.to_base(), eseries, d);
    resp
}

fn rows(report: &Report, target: f64, eseries: ESeries, d: u32) -> Vec<ResistorRow> {
    let tolerance = eseries.tolerance();
    let cell = |entry: Option<(String, f64)>| {
        entry.map(|(text, value)| {
            let (deviation, in_tolerance) = deviation_text(value, target, tolerance);
            ResistorCell {
                text,
                deviation,
                in_tolerance,
            }
        })
    };

    vec![
        ResistorRow {
            label: msg!("resistor_solver.single"),
            cells: vec![
                cell(Some(single_data(report.eseries_single, d))),
                cell(report.pool_single.map(|r| single_data(r, d))),
            ],
        },
        ResistorRow {
            label: msg!("resistor_solver.series_pair"),
            cells: vec![
                cell(pair_data(report.eseries_series, '+', d)),
                cell(pair_data(report.pool_series, '+', d)),
            ],
        },
        ResistorRow {
            label: msg!("resistor_solver.parallel_pair"),
            cells: vec![
                cell(pair_data(report.eseries_parallel, '‖', d)),
                cell(pair_data(report.pool_parallel, '‖', d)),
            ],
        },
    ]
}

fn deviation_text(value: f64, target: f64, tolerance: f64) -> (String, bool) {
    if target == 0.0 {
        return ("n/a".to_owned(), false);
    }
    let ratio = (value - target) / target;
    let in_tol = ratio.abs() <= tolerance;
    let mark = if in_tol { '✓' } else { '⚠' };
    (format!("{:+.2} %  {}", ratio * 100.0, mark), in_tol)
}

fn single_data(r: ElectricalResistance, decimals: u32) -> (String, f64) {
    (format_engineering(r.to_base(), "Ω", decimals), r.to_base())
}

fn pair_data(
    pair: Option<(ElectricalResistance, ElectricalResistance)>,
    op: char,
    decimals: u32,
) -> Option<(String, f64)> {
    let (a, b) = pair?;
    let combined = if op == '+' { a + b } else { calc::parallel(a, b) };
    // Formatting is the edge: from here on plain numbers in ohms.
    let (a, b, combined) = (a.to_base(), b.to_base(), combined.to_base());
    let text = format!(
        "{} {} {} = {}",
        format_engineering(a, "Ω", decimals),
        op,
        format_engineering(b, "Ω", decimals),
        format_engineering(combined, "Ω", decimals),
    );
    Some((text, combined))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_target_gives_three_rows() {
        let resp = solve(&Form::new(4, &[("target", "10.12k"), ("pool", "10k, 2k, 100"), ("series", "E24")]));
        assert!(matches!(resp.outcome, Outcome::Solved));
        assert_eq!(resp.rows.len(), 3);
        assert!(resp.rows[0].cells[0].as_ref().unwrap().in_tolerance);
    }

    #[test]
    fn empty_target_is_pending() {
        let resp = solve(&Form::new(4, &[("series", "E24")]));
        assert!(matches!(resp.outcome, Outcome::Pending(_)));
    }
}

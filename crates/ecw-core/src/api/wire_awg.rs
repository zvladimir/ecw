//! Wire AWG page: the current, the length and the thermal resistance in, a table of all AWG
//! sizes out.

use crate::msg;
use serde::{Deserialize, Serialize};
use uom::si::area::square_millimeter;
use uom::si::f64::{Area, Length};
use uom::si::length::millimeter;

use super::{FieldInfo, Outcome};
use crate::domain::{FieldState, Text};
use crate::domain::error::ParseError;
use crate::domain::format::format_engineering;
use crate::domain::interval::Interval;
use crate::domain::measure::Measure;
use crate::domain::quantity::Quantity;
use crate::domain::unit::Measured;
use crate::wire_awg::{self as awg, calc as awg_calc};

/// Unlike the others this calculator also takes a list (strands per gauge); a field left out
/// reads as empty, as in `Form`.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct AwgRequest {
    /// The current, as typed.
    pub current: String,
    /// The length of the wire, as typed.
    pub length: String,
    /// The thermal resistance of the wire to the air, as typed (no tolerance).
    pub rth: String,
    /// Strands per gauge, in table order.
    pub n: Vec<f64>,
    /// The number of digits after the point in the results.
    pub decimals: u32,
}

/// A row of the table: the texts of the numbers of one AWG size.
#[derive(Debug, Serialize)]
pub struct AwgRow {
    /// The AWG size.
    pub awg: String,
    /// The diameter.
    pub d: String,
    /// The cross section.
    pub a: String,
    /// The resistance of the length.
    pub r: String,
    /// The voltage drop at the current.
    pub vdrop: String,
    /// The power loss.
    pub p: String,
    /// The temperature rise over the air.
    pub dt: String,
}

/// The reply of the Wire AWG page: the fields, the rows and the limits of the strands counter.
#[derive(Debug, Serialize)]
pub struct AwgResponse {
    /// The current field.
    pub current: FieldInfo,
    /// The length field.
    pub length: FieldInfo,
    /// The thermal resistance field.
    pub rth: FieldInfo,
    /// What to show in the results area.
    pub outcome: Outcome,
    /// The table: a row for each AWG size.
    pub rows: Vec<AwgRow>,
    /// The number of rows of the full table.
    pub rows_total: usize,
    /// The least number of parallel wires.
    pub min_n: f64,
    /// The most parallel wires.
    pub max_n: f64,
    /// The number of parallel wires of a row the user has not changed.
    pub default_n: f64,
    /// The note under the table.
    pub note: Text,
}

/// Parses the fields, calculates the table and lays it out. The number of parallel wires of
/// each row comes with the request.
pub fn solve(req: &AwgRequest) -> AwgResponse {
    let d = req.decimals;

    let current = FieldState::parse_exact(&req.current, Quantity::Current);
    let length = FieldState::parse_exact(&req.length, Quantity::Length);
    let rth = parse_plain_number(&req.rth, Quantity::ThermalResistance);

    let all_valid = current.is_ok() && length.is_ok() && rth.is_ok();
    let has_error = current.has_error() || length.has_error() || rth.has_error();

    let result = all_valid.then(|| {
        awg_calc::solve(awg_calc::Input {
            i: measure_interval(&current),
            l: measure_interval(&length),
            rth: measure_interval(&rth),
            n: &req.n,
        })
    });

    let rows = match &result {
        Some(Ok(rows)) => rows
            .iter()
            .map(|row| AwgRow {
                awg: row.awg.to_owned(),
                d: fmt_fixed(row.d_mm(), "mm", d),
                a: fmt_fixed(row.a_mm2(), "mm²", d),
                r: format_engineering(row.r.nominal.to_base(), "Ω", d),
                vdrop: format_engineering(row.vdrop.nominal.to_base(), "V", d),
                p: format_engineering(row.p.nominal.to_base(), "W", d),
                dt: format_engineering(row.dt.nominal.to_base(), "K", d),
            })
            .collect(),
        _ => Vec::new(),
    };

    AwgResponse {
        current: FieldInfo::from_state(&current, Quantity::Current.unit(), d, true),
        length: FieldInfo::from_state(&length, Quantity::Length.unit(), d, true),
        rth: FieldInfo::from_state(&rth, Quantity::ThermalResistance.unit(), d, true),
        outcome: Outcome::new(
            result.as_ref(),
            has_error,
            msg!("wire_awg.fill_all"),
        ),
        rows,
        rows_total: awg_calc::AWG_ROWS,
        min_n: awg::MIN_N,
        max_n: awg::MAX_N,
        default_n: awg::DEFAULT_N,
        note: msg!("wire_awg.data_source_note"),
    }
}

pub(crate) fn parse_plain_number(text: &str, quantity: Quantity) -> FieldState {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return FieldState::Empty;
    }
    match trimmed.parse::<f64>() {
        Ok(v) if v.is_finite() => FieldState::Ok(Measure::exact(v)),
        _ => FieldState::Err(ParseError::BadNumber {
            quantity,
            text: trimmed.to_owned(),
        }),
    }
}

fn measure_interval<T: Measured>(state: &FieldState) -> Interval<T> {
    state
        .measure()
        .map_or_else(|| Interval::exact(T::from_base(0.0)), |m| m.to_interval())
}

fn fmt_fixed(value: f64, unit: &str, decimals: u32) -> String {
    let prec = decimals as usize;
    format!("{value:.prec$} {unit}")
}

impl awg_calc::Row {
    fn d_mm(&self) -> f64 {
        Length::get::<millimeter>(&self.d)
    }

    fn a_mm2(&self) -> f64 {
        Area::get::<square_millimeter>(&self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn awg_defaults_give_full_table() {
        let resp = solve(&AwgRequest {
            current: "1".into(),
            length: "1".into(),
            rth: "50".into(),
            n: vec![],
            decimals: 4,
        });
        assert!(matches!(resp.outcome, Outcome::Solved));
        assert_eq!(resp.rows.len(), resp.rows_total);
        assert_eq!(resp.rows[0].awg, "40");
    }

    #[test]
    fn awg_reports_bad_field() {
        let resp = solve(&AwgRequest {
            current: "abc".into(),
            length: "1".into(),
            rth: "50".into(),
            n: vec![],
            decimals: 4,
        });
        assert_eq!(resp.current.status, "error");
        assert!(matches!(resp.outcome, Outcome::FixErrors(_)));
    }
}

//! Inverting and non-inverting amplifier pages: the ranges, the resistors and the gain, the
//! results table and the transfer plot.

use super::Form;
use crate::msg;
use serde::Serialize;
use uom::si::electric_potential::volt;
use uom::si::f64::{ElectricPotential, ElectricalResistance};

use super::report::{PLOT_DEFAULT_RANGE, empty_plot};
use super::{FieldInfo, GroupedField, Outcome, PlotData, PlotLine, Table};
use crate::domain::{FieldState, Text};
use crate::domain::format::format_engineering;
use crate::domain::interval::Interval;
use crate::domain::quantity::Quantity;
use crate::domain::unit::Measured;
use crate::opamp::Topology;
use crate::opamp::calc as opamp_calc;

const PLOT_RANGE_PAD: f64 = 0.05;

/// A range field: `1; 5` is the two ends, one value is both ends, an empty half leaves that
/// end open (`1;` or `; 5`).
pub(super) fn split_range(text: &str) -> (&str, &str) {
    match text.split_once(';') {
        Some((min, max)) => (min.trim(), max.trim()),
        None => (text.trim(), text.trim()),
    }
}

/// The reply of an amplifier page. The `table` has Vin, Vout, R1, R2 and G; `no_data` says why
/// there is nothing to show.
#[derive(Debug, Serialize)]
pub struct OpampResponse {
    /// The input fields with their state: parse result, hint, and the computed value of a locked field.
    pub fields: Vec<GroupedField>,
    /// What to show in the results area.
    pub outcome: Outcome,
    /// Vin, Vout, R1, R2 and G in one table: Nom, Min, Max, Δ.
    pub table: Option<Table>,
    /// Why there is no table and no plot yet, if so.
    pub no_data: Option<Text>,
    /// Vout against Vin.
    pub plot: PlotData,
}

struct Inputs {
    vin_min: FieldState,
    vin_max: FieldState,
    vout_min: FieldState,
    vout_max: FieldState,
    r1: FieldState,
    r2: FieldState,
    gain: FieldState,
}

impl Inputs {
    fn parse(req: &Form, topology: Topology) -> Self {
        let exact = |text: &str, q| FieldState::parse_exact(text, q);
        let (vin_min, vin_max) = split_range(req.text("vin"));
        let (vout_min, vout_max) = split_range(req.text("vout"));
        Self {
            vin_min: exact(vin_min, Quantity::Voltage),
            vin_max: exact(vin_max, Quantity::Voltage),
            vout_min: exact(vout_min, Quantity::Voltage),
            vout_max: exact(vout_max, Quantity::Voltage),
            r1: FieldState::parse(req.text("r1"), Quantity::Resistance),
            r2: FieldState::parse(req.text("r2"), Quantity::Resistance),
            gain: match topology {
                Topology::Inverting => FieldState::parse_non_positive(req.text("gain"), Quantity::Ratio),
                Topology::NonInverting => exact(req.text("gain"), Quantity::Ratio),
            },
        }
    }

    fn all(&self) -> [(&'static str, &FieldState, &'static str); 7] {
        [
            ("vin_min", &self.vin_min, "V"),
            ("vin_max", &self.vin_max, "V"),
            ("vout_min", &self.vout_min, "V"),
            ("vout_max", &self.vout_max, "V"),
            ("r1", &self.r1, "Ω"),
            ("r2", &self.r2, "Ω"),
            ("gain", &self.gain, ""),
        ]
    }

    fn has_error(&self) -> bool {
        self.all().iter().any(|(_, s, _)| s.has_error())
    }

    fn to_calc(&self) -> opamp_calc::Input {
        let voltage = |state: &FieldState| state.measure().map(|m| m.to_interval());
        opamp_calc::Input {
            vin_min: voltage(&self.vin_min),
            vin_max: voltage(&self.vin_max),
            vout_min: voltage(&self.vout_min),
            vout_max: voltage(&self.vout_max),
            r1: self.r1.measure().map(|m| m.to_interval()),
            r2: self.r2.measure().map(|m| m.to_interval()),
            gain: self.gain.measure().map(|m| m.to_interval()),
        }
    }
}

#[derive(Default)]
struct ComputedValues {
    vin_min: Option<Interval<f64>>,
    vin_max: Option<Interval<f64>>,
    vout_min: Option<Interval<f64>>,
    vout_max: Option<Interval<f64>>,
    r1: Option<Interval<f64>>,
    r2: Option<Interval<f64>>,
    gain: Option<Interval<f64>>,
}

impl ComputedValues {
    fn from_solved(solved: Option<&opamp_calc::Solved>) -> Self {
        let Some(s) = solved else {
            return Self::default();
        };

        fn pick<T: Measured>(
            is_computed: bool,
            value: Option<Interval<T>>,
        ) -> Option<Interval<f64>> {
            value
                .filter(|_| is_computed)
                .map(|interval| interval.map(T::to_base))
        }

        Self {
            vin_min: pick(s.computed.vin_min, s.vin_min),
            vin_max: pick(s.computed.vin_max, s.vin_max),
            vout_min: pick(s.computed.vout_min, s.vout_min),
            vout_max: pick(s.computed.vout_max, s.vout_max),
            r1: pick(s.computed.r1, s.r1),
            r2: pick(s.computed.r2, s.r2),
            gain: s.gain.filter(|_| s.computed.gain).map(|iv| iv.map(Measured::to_base)),
        }
    }

    fn get(&self, key: &str) -> Option<Interval<f64>> {
        match key {
            "vin_min" => self.vin_min,
            "vin_max" => self.vin_max,
            "vout_min" => self.vout_min,
            "vout_max" => self.vout_max,
            "r1" => self.r1,
            "r2" => self.r2,
            "gain" => self.gain,
            _ => None,
        }
    }
}

/// Solves the amplifier of the `topology` field (`inverting` or `non_inverting`) from the
/// given fields and lays out the page.
pub fn solve(req: &Form) -> OpampResponse {
    let topology = match req.text("topology") {
        "non_inverting" => Topology::NonInverting,
        _ => Topology::Inverting,
    };
    let d = req.decimals;

    let inputs = Inputs::parse(req, topology);
    let result = (!inputs.has_error()).then(|| opamp_calc::solve(inputs.to_calc(), topology));
    let solved = result.as_ref().and_then(|r| r.as_ref().ok());
    let computed = ComputedValues::from_solved(solved);

    // All five fields sit on the schematic, so the group is only a formality.
    let field = |key: &'static str, label: &str, hint: &str, info: FieldInfo| GroupedField {
        key,
        label: label.into(),
        hint: hint.into(),
        group: "signal",
        info,
    };
    let single = |key: &'static str, label: &str, hint: &str, state: &FieldState, unit: &str| {
        let mut info = FieldInfo::from_state(state, unit, d, false);
        // A value the user typed stays in its field; only an empty one shows the result.
        info.computed = computed
            .get(key)
            .filter(|_| !state.is_ok())
            .map(|iv| format_engineering(iv.nominal, unit, d));
        field(key, label, hint, info)
    };
    // `Vin` and `Vout` are one field each: the error of either end, and the computed ends
    // as `min; max` once both are known.
    let range = |key: &'static str, label: &str, hint: &str, min: (&'static str, &FieldState), max: (&'static str, &FieldState)| {
        let shown = [min.1, max.1]
            .into_iter()
            .find(|s| s.has_error())
            .or([min.1, max.1].into_iter().find(|s| s.is_ok()))
            .unwrap_or(min.1);
        let mut info = FieldInfo::from_state(shown, "V", d, false);
        let typed = min.1.is_ok() || max.1.is_ok();
        info.computed = computed.get(min.0).zip(computed.get(max.0)).filter(|_| !typed).map(|(lo, hi)| {
            format!(
                "{}; {}",
                format_engineering(lo.nominal, "V", d),
                format_engineering(hi.nominal, "V", d)
            )
        });
        field(key, label, hint, info)
    };
    let fields = vec![
        range("vin", "Vin min; max [V]", "-7; 7", ("vin_min", &inputs.vin_min), ("vin_max", &inputs.vin_max)),
        range("vout", "Vout min; max [V]", "-14; 14", ("vout_min", &inputs.vout_min), ("vout_max", &inputs.vout_max)),
        single("r1", "R1 [Ω]", "1k", &inputs.r1, "Ω"),
        single("r2", "R2 [Ω]", "10k", &inputs.r2, "Ω"),
        single("gain", "G [V/V]", "-10", &inputs.gain, ""),
    ];

    let (mut table, mut no_data) = (None, None);
    if let Some(s) = solved {
        let has_signals = [s.vin_min, s.vin_max, s.vout_min, s.vout_max]
            .iter()
            .any(Option::is_some);
        let has_params = s.r1.is_some() || s.r2.is_some() || s.gain.is_some();
        if has_signals || has_params {
            table = Some(results_table(s, d));
        } else {
            no_data = Some(msg!("common.no_data"));
        }
    }

    OpampResponse {
        fields,
        outcome: Outcome::new(result.as_ref(), inputs.has_error(), Text::default()),
        table,
        no_data,
        plot: plot_data(solved),
    }
}

/// Everything on one table. A voltage entered or found as a single value goes to Nom; for two
/// values (a range) Nom is their mean and Min and Max are the ends. For Vout, Nom is the value
/// without resistor tolerances and Min/Max are the worst case with them.
fn results_table(solved: &opamp_calc::Solved, d: u32) -> Table {
    let to_f64 = |iv: Interval<ElectricalResistance>| iv.map(ElectricalResistance::to_base);
    let volts = |iv: Option<Interval<ElectricPotential>>| iv.map(|i| i.map(|v| v.get::<volt>()));

    let mut rows = Vec::new();
    // Vin and Vout are the ends of the range in order of Vin; for an inverting stage the lower
    // Vin end meets the higher Vout end, which the Min/Max columns do not care about.
    let (vin_lo, vin_hi) = (volts(solved.vin_min), volts(solved.vin_max));
    let (vout_lo, vout_hi) = (volts(solved.vout_min), volts(solved.vout_max));
    if vin_lo.is_some() || vin_hi.is_some() {
        rows.push(voltage_row("Vin [V]".into(), vin_lo, vin_hi, d));
    }
    if vout_lo.is_some() || vout_hi.is_some() {
        // As in the difference amplifier: the ideal Vout, and below it Vout with the tolerances.
        let ideal = |iv: Option<Interval<f64>>| iv.map(|i| Interval::new(i.nominal, i.nominal, i.nominal));
        rows.push(voltage_row(msg!("opamp.row.vout_ideal"), ideal(vout_lo), ideal(vout_hi), d));
        if [vout_lo, vout_hi].iter().flatten().any(|i| !i.is_exact()) {
            rows.push(voltage_row(msg!("opamp.row.vout_with_tolerances"), vout_lo, vout_hi, d));
        }
    }
    rows.extend(
        [
            solved.r1.map(|iv| param_row("R1 [Ω]", to_f64(iv), "Ω", d)),
            solved.r2.map(|iv| param_row("R2 [Ω]", to_f64(iv), "Ω", d)),
            solved.gain.map(|iv| param_row("G [V/V]", iv.map(Measured::to_base), "", d)),
        ]
        .into_iter()
        .flatten(),
    );

    Table {
        header: vec![
            Text::default(),
            msg!("common.nom"),
            msg!("common.min"),
            msg!("common.max"),
            "Δ, %".into(),
        ],
        rows,
    }
}

/// A voltage given by its two ends (either may be missing).
fn voltage_row(label: Text, a: Option<Interval<f64>>, b: Option<Interval<f64>>, d: u32) -> Vec<Text> {
    let dash = || Text::from("—");
    let f = |v: f64| Text::from(format_engineering(v, "V", d));
    let ends: Vec<Interval<f64>> = [a, b].into_iter().flatten().collect();
    let lo = ends.iter().map(|i| i.lo).fold(f64::INFINITY, f64::min);
    let hi = ends.iter().map(|i| i.hi).fold(f64::NEG_INFINITY, f64::max);

    let single_value = ends.len() == 2 && ends[0].nominal == ends[1].nominal;
    if single_value {
        let nominal = Interval::new(ends[0].nominal, lo, hi);
        let tolerance = !nominal.is_exact();
        return vec![
            label,
            f(nominal.nominal),
            if tolerance { f(lo) } else { dash() },
            if tolerance { f(hi) } else { dash() },
            format_delta(nominal).into(),
        ];
    }
    // A range: Nom is the mean of the two entered values, Min and Max are the ends (for Vout
    // with the tolerances). With only one end known there is no mean.
    let (nom_cell, min_cell, max_cell) = match (a, b) {
        (Some(x), Some(y)) => (f((x.nominal + y.nominal) / 2.0), f(lo), f(hi)),
        (Some(_), None) => (dash(), f(lo), dash()),
        _ => (dash(), dash(), f(hi)),
    };
    vec![label, nom_cell, min_cell, max_cell, dash()]
}

fn param_row(label: &str, interval: Interval<f64>, unit: &str, d: u32) -> Vec<Text> {
    let nominal = format_engineering(interval.nominal, unit, d);
    let (lo, hi) = if interval.is_exact() {
        ("—".to_owned(), "—".to_owned())
    } else {
        (
            format_engineering(interval.lo, unit, d),
            format_engineering(interval.hi, unit, d),
        )
    };
    vec![label.into(), nominal.into(), lo.into(), hi.into(), format_delta(interval).into()]
}

pub(super) fn format_delta(interval: Interval<f64>) -> String {
    if interval.is_exact() || interval.nominal == 0.0 {
        return "—".to_owned();
    }
    let n = interval.nominal.abs();
    let up = (interval.hi - interval.nominal).abs() / n * 100.0;
    let dn = (interval.nominal - interval.lo).abs() / n * 100.0;
    let sum = up + dn;
    if sum < 1e-9 {
        return "—".to_owned();
    }
    if (up - dn).abs() / sum < 0.10 {
        format!("±{:.2}", sum / 2.0)
    } else {
        format!("+{up:.2} / -{dn:.2}")
    }
}

/// DC transfer Vout(Vin): nominal line plus the worst-case tolerance lines.
fn plot_data(solved: Option<&opamp_calc::Solved>) -> PlotData {
    let base = || empty_plot("Vin", "Vout");
    // The plot is an edge: the gain goes in as a plain number.
    let Some((solved, g)) = solved.and_then(|s| s.gain.map(|g| (s, g.map(Measured::to_base)))) else {
        return base();
    };

    let volts = |iv: Option<Interval<ElectricPotential>>| iv.map(|v| v.nominal.get::<volt>());
    let ends: Vec<f64> = [volts(solved.vin_min), volts(solved.vin_max)]
        .into_iter()
        .flatten()
        .collect();
    let (x_lo, x_hi) = match ends.as_slice() {
        [] => (-PLOT_DEFAULT_RANGE, PLOT_DEFAULT_RANGE),
        [only] => (only.min(0.0), only.max(0.0)),
        [a, b] => (a.min(*b), a.max(*b)),
        _ => unreachable!("at most two Vin ends"),
    };
    let (x_lo, x_hi) = if x_hi - x_lo < 1e-9 {
        (-PLOT_DEFAULT_RANGE, PLOT_DEFAULT_RANGE)
    } else {
        (x_lo, x_hi)
    };

    let y_values = [x_lo, x_hi]
        .into_iter()
        .flat_map(|x| [g.lo, g.nominal, g.hi].map(|gain| x * gain));
    let y_lo = y_values.clone().fold(0.0_f64, f64::min);
    let y_hi = y_values.fold(0.0_f64, f64::max);

    let x_pad = (x_hi - x_lo) * PLOT_RANGE_PAD;
    let y_pad = ((y_hi - y_lo) * PLOT_RANGE_PAD).max(1e-9);

    let line = |kind, name: Text, gain: f64| PlotLine {
        kind,
        name,
        points: vec![[x_lo, x_lo * gain], [x_hi, x_hi * gain]],
        width: if kind == "nominal" { 2.4 } else { 1.4 },
    };

    let mut lines = vec![line(
        "nominal",
        msg!("opamp.legend.nominal"),
        g.nominal,
    )];
    if !g.is_exact() {
        lines.push(line("max", msg!("opamp.legend.max"), g.hi));
        lines.push(line("min", msg!("opamp.legend.min"), g.lo));
    }

    PlotData {
        x: [x_lo - x_pad, x_hi + x_pad],
        y: [y_lo - y_pad, y_hi + y_pad],
        lines,
        ..base()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opamp(topology: &str) -> Form {
        Form::new(4, &[("topology", topology)])
    }

    fn field<'a>(resp: &'a OpampResponse, key: &str) -> &'a FieldInfo {
        &resp.fields.iter().find(|f| f.key == key).expect("field").info
    }

    #[test]
    fn inverting_computes_gain_and_vout() {
        let mut req = opamp("inverting");
        req.set("vin", "-1; 1");
        req.set("r1", "1k");
        req.set("r2", "5k");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved));
        assert!(field(&resp, "gain").computed.is_some());
        assert_eq!(field(&resp, "vout").computed.as_deref(), Some("-5.0000 V; 5.0000 V"));
        assert_eq!(resp.plot.lines.len(), 1);
    }

    #[test]
    fn one_value_is_both_ends_and_units_are_understood() {
        let mut req = opamp("non_inverting");
        req.set("vin", "500m");
        req.set("r1", "9k");
        req.set("r2", "1k");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved));
        assert_eq!(field(&resp, "vout").computed.as_deref(), Some("5.0000 V; 5.0000 V"));
        assert!(field(&resp, "vin").computed.is_none());
    }

    #[test]
    fn one_table_puts_points_in_nom_and_ranges_in_min_max() {
        // One value for Vin: Nom. Vout: Nom without tolerance, Min/Max with it.
        let mut req = opamp("non_inverting");
        req.set("vin", "1");
        req.set("r1", "9k 1%");
        req.set("r2", "1k 1%");
        let table = solve(&req).table.unwrap();
        let row = |name: &str| table.rows.iter().find(|r| r[0] == name).unwrap().clone();
        let vin = row("Vin [V]");
        assert_eq!((vin[1].to_string().as_str(), vin[2].to_string().as_str(), vin[3].to_string().as_str()), ("1.0000 V", "—", "—"));
        let rows = table.rows.iter().filter(|r| r[0] == msg!("opamp.row.vout_ideal") || r[0] == msg!("opamp.row.vout_with_tolerances")).collect::<Vec<_>>();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0], msg!("opamp.row.vout_ideal"));
        assert_eq!((rows[0][1].to_string().as_str(), rows[0][2].to_string().as_str()), ("10.0000 V", "—"));
        assert_eq!(rows[1][0], msg!("opamp.row.vout_with_tolerances"));
        assert_eq!((rows[1][2].to_string().as_str(), rows[1][3].to_string().as_str()), ("9.8218 V", "10.1818 V"));

        // Two values: Nom is their mean, Min and Max the ends.
        req.set("vin", "1; 2");
        let table = solve(&req).table.unwrap();
        let vin = table.rows.iter().find(|r| r[0] == "Vin [V]").unwrap();
        assert_eq!((vin[1].to_string().as_str(), vin[2].to_string().as_str(), vin[3].to_string().as_str()), ("1.5000 V", "1.0000 V", "2.0000 V"));
        assert_eq!(table.rows.len(), 6);
    }

    #[test]
    fn a_bad_end_marks_the_whole_field() {
        let mut req = opamp("non_inverting");
        req.set("vin", "1; 5 V");
        let resp = solve(&req);
        assert_eq!(field(&resp, "vin").status, "error");
        assert!(matches!(resp.outcome, Outcome::FixErrors(_)));
    }

    #[test]
    fn a_typed_gain_stays_in_its_field_when_r1_has_a_tolerance() {
        let mut req = opamp("inverting");
        req.set("vin", "1");
        req.set("gain", "-5");
        req.set("r1", "1k 5%");
        let resp = solve(&req);
        assert!(field(&resp, "gain").computed.is_none());
        assert!(field(&resp, "r2").computed.is_some());
        assert!(resp.table.is_some());
    }

    #[test]
    fn positive_inverting_gain_is_input_error() {
        let mut req = opamp("inverting");
        req.set("gain", "5");
        let resp = solve(&req);
        assert_eq!(field(&resp, "gain").status, "error");
        assert!(matches!(resp.outcome, Outcome::FixErrors(_)));
    }

    #[test]
    fn a_gain_of_zero_that_follows_from_the_voltages_is_refused() {
        let mut req = opamp("inverting");
        req.set("vin", "1");
        req.set("vout", "0");
        req.set("r1", "1k");
        assert!(matches!(solve(&req).outcome, Outcome::Error(_)));
    }
}

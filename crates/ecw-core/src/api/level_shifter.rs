//! Level shifter on an inverting amplifier with a reference voltage, laid out like the
//! difference amplifier: the fields sit on the schematic, any two of Vin, Vout and G (with the
//! offset from Vref) give the third, one resistor gives the others, and the tolerances of the
//! resistors and Vref widen the output into a worst-case band.

use super::Form;
use crate::msg;
use serde::Serialize;
use uom::num_traits::Zero;
use uom::si::electric_potential::volt;
use uom::si::f64::{ElectricPotential, ElectricalResistance, Ratio};

use super::report::{
    DesignForm, Input, Spec, VrefSource, add_vref_and_band, corner, empty_plot, extremes, param_row, range_row,
    range_text, results_header, sig, spec, spec_in, transfer_plot, value_text, vout_rows, vref_row,
};
use super::{Check, GroupedField, Outcome, PlotData, Table};
use crate::domain::infer::{Values, infer};
use crate::domain::unit::Measured;
use crate::domain::{FieldState, Quantity, Text};
use crate::level_shifter::inverting::{A, COUNT, R1, R2, R3, VI_MAX, VI_MIN, VO_MAX, VO_MIN, VREF, rules};
use crate::level_shifter::{Error, ideal, output, recommend_inverting_reference};

type Volts = ElectricPotential;
type Ohms = ElectricalResistance;

const VIN: Spec = spec("vin", "Vin min; max [V]", "-5; -1", "signal", Quantity::Voltage, "V");
const VOUT: Spec = spec("vout", "Vout min; max [V]", "0.05; 3.3", "signal", Quantity::Voltage, "V");
const GAIN: Spec = spec("gain", "G [V/V]", "-0.81", "signal", Quantity::Ratio, "");

const SPECS: [Spec; 4] = [
    // Vref and the resistors may carry a tolerance (`10k 1%`).
    spec_in("vref", "Vref [V]", "5", "circuit", Input::Tolerant, Quantity::Voltage, "V"),
    spec_in("r1", "R1 [Ω]", "845", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
    spec_in("r2", "R2 [Ω]", "1.04k", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
    spec_in("r3", "R3 [Ω]", "5.56k", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
];

/// The reply of the inverting level shifter page.
#[derive(Debug, Serialize)]
pub struct LevelShifterResponse {
    /// The fields on the schematic: the signal ranges, the reference and the resistors.
    pub fields: Vec<GroupedField>,
    /// What to show in the results area.
    pub outcome: Outcome,
    /// The results table (Nom, Min, Max, Δ), once the page is solved.
    pub steps: Option<Table>,
    /// The checks under the results. Empty for now: they were taken out and may come back.
    pub checks: Vec<Check>,
    /// The notes under the results. Empty for now, as the checks.
    pub notes: Vec<Text>,
    /// Vo against Vin.
    pub plot: Option<PlotData>,
}

/// Solves the inverting level shifter from the given fields and lays out the page: fields on
/// the schematic, results, and the plot of Vo against Vin with the worst-case band.
pub fn solve(req: &Form) -> LevelShifterResponse {
    let gain_state = FieldState::parse_non_positive(req.text("gain"), Quantity::Ratio);
    let mut form = DesignForm::parse(req, &SPECS, &[VIN, VOUT], (&GAIN, gain_state));
    let (outcome, steps, plot) = match design(&mut form, req.decimals) {
        Ok((steps, plot)) => (Outcome::Solved, Some(steps), plot),
        // Empty axes until the form is solved, as for the other amplifiers.
        Err(outcome) => (outcome, None, empty_plot("Vin", "Vo")),
    };
    LevelShifterResponse { fields: form.fields, outcome, steps, checks: Vec::new(), notes: Vec::new(), plot: Some(plot) }
}

/// What the user typed, in the types of the circuit.
struct Given {
    vin: Option<(Volts, Volts)>,
    vout: Option<(Volts, Volts)>,
    vref: Option<Volts>,
    /// The gain magnitude `R1/R2` (the field holds `G = −R1/R2`).
    a: Option<Ratio>,
    r1: Option<Ohms>,
    r2: Option<Ohms>,
    r3: Option<Ohms>,
}

impl Given {
    /// Runs the rules from the given values, with `vref` in place of an empty reference. With
    /// both ranges given they fix the design and a typed G is not used.
    fn infer(&self, vref: Option<Volts>) -> Result<Values, Outcome> {
        let mut values = Values::new(COUNT);
        let both = self.vin.is_some() && self.vout.is_some();
        values.set_exact(VI_MIN, self.vin.map(|r| r.0));
        values.set_exact(VI_MAX, self.vin.map(|r| r.1));
        values.set_exact(VO_MIN, self.vout.map(|r| r.0));
        values.set_exact(VO_MAX, self.vout.map(|r| r.1));
        values.set_exact(VREF, self.vref.or(vref));
        values.set_exact(A, self.a.filter(|_| !both));
        values.set_exact(R1, self.r1);
        values.set_exact(R2, self.r2);
        values.set_exact(R3, self.r3);
        infer(&mut values, &rules()).map_err(|e| Outcome::Error(e.text()))?;
        Ok(values)
    }
}

fn design(form: &mut DesignForm, d: u32) -> Result<(Table, PlotData), Outcome> {
    if form.has_error() {
        return Err(Outcome::FixErrors(msg!("common.fix_errors")));
    }
    let given = Given {
        vin: form.range("vin"),
        vout: form.range("vout"),
        vref: form.value("vref"),
        a: form.value::<Ratio>("gain").map(|g| -g),
        r1: form.value("r1"),
        r2: form.value("r2"),
        r3: form.value("r3"),
    };
    for (r, name) in [(given.r1, "R1"), (given.r2, "R2"), (given.r3, "R3")] {
        if r.is_some_and(|r| r <= Ohms::zero()) {
            return Err(Outcome::Error(msg!("level_shifter.err.positive", name = name)));
        }
    }
    if given.a == Some(Ratio::zero()) {
        return Err(Outcome::Error(msg!("level_shifter.err.gain_negative")));
    }

    // An empty Vref is found from the two ranges: the value the given resistors require, or
    // else a standard one of the sign the offset needs.
    let mut values = given.infer(None)?;
    let source = if given.vref.is_some() {
        VrefSource::Given
    } else {
        let (Some(vi), Some(vo)) = (given.vin, given.vout) else {
            return Err(Outcome::Pending(msg!("level_shifter.fill_in_vref")));
        };
        if values.is_known(VREF) {
            VrefSource::Required
        } else {
            let (vref, source) = match recommend_inverting_reference(vi, vo) {
                Some(vref) => (vref, VrefSource::Recommended),
                // Any reference will do: R3 stays open.
                None => (Volts::new::<volt>(1.0), VrefSource::NotNeeded),
            };
            values = given.infer(Some(vref))?;
            source
        }
    };
    let vref = values.nominal(VREF).expect("the reference is given or chosen");
    if matches!(source, VrefSource::Required | VrefSource::Recommended) {
        form.set_hint("vref", sig(vref.to_base()));
    }

    let range = |lo, hi| values.nominal(lo).zip(values.nominal(hi));
    let (Some(vi), Some(vo)) = (range(VI_MIN, VI_MAX), range(VO_MIN, VO_MAX)) else {
        return Err(Outcome::Pending(msg!("level_shifter.fill_in")));
    };
    // Whatever the other fields settled and the user left empty is locked and shown computed.
    if given.vin.is_none() {
        form.set_computed("vin", range_text(vi, d));
    }
    if given.vout.is_none() {
        form.set_computed("vout", range_text(vo, d));
    }
    let design = ideal(vi, vo, vref).map_err(|error| {
        Outcome::Error(match error {
            Error::EmptySpan => msg!("level_shifter.err.empty_span"),
            Error::NoReference => msg!("level_shifter.err.no_reference"),
            Error::WrongReferenceSign { .. } => {
                msg!("level_shifter.err.wrong_reference_sign", vref = sig(vref.to_base()), opposite = sig(-vref.to_base()))
            }
        })
    })?;
    if given.a.is_none() {
        form.set_computed("gain", value_text(-design.g_in, d));
    }

    // One resistor fixes the others; the first one given is the anchor of their tolerance.
    let Some(r1) = values.nominal(R1) else {
        return Err(Outcome::Pending(msg!("level_shifter.fill_in_resistor")));
    };
    let anchor = [("r1", given.r1), ("r2", given.r2), ("r3", given.r3)]
        .into_iter()
        .find_map(|(key, r)| r.map(|_| key))
        .expect("a resistor is given");
    let r2 = values.nominal(R2).expect("R1 and the gain give R2");
    // No offset needed: no R3 (an open reference path) unless one is given.
    let r3 = values.nominal(R3);
    if given.r1.is_none() {
        form.set_computed("r1", value_text(r1, d));
    }
    if given.r2.is_none() {
        form.set_computed("r2", value_text(r2, d));
    }
    if given.r3.is_none() {
        form.set_computed("r3", r3.map_or_else(|| "∞".to_owned(), |r3| value_text(r3, d)));
    }

    let circuit = Circuit { vi, vo, vref, source, a: design.g_in, r1, r2, r3, anchor };
    Ok(build(form, &circuit, d))
}

struct Circuit {
    vi: (Volts, Volts),
    vo: (Volts, Volts),
    vref: Volts,
    source: VrefSource,
    /// The gain magnitude of the design (from the ranges).
    a: Ratio,
    r1: Ohms,
    r2: Ohms,
    r3: Option<Ohms>,
    /// The given resistor whose tolerance the calculated ones take.
    anchor: &'static str,
}

/// Worst case: R1, R2, R3 and Vref each move within their own tolerance; a calculated resistor
/// takes the relative tolerance of the anchor. Vo is monotonic in every one of them, so the
/// extremes are at the corners.
fn build(form: &DesignForm, c: &Circuit, d: u32) -> (Table, PlotData) {
    let Circuit { vi, vo, vref, source, a, r1, r2, r3, anchor } = *c;
    let rel = form.relative(anchor);
    let ends = |key: &str, value: Ohms| {
        if form.value::<Ohms>(key).is_some() { form.ends(key, value) } else { (value * rel.0, value * rel.1) }
    };
    let (r1_b, r2_b) = (ends("r1", r1), ends("r2", r2));
    let r3_b = r3.map(|r3| ends("r3", r3));
    let vref_b = form.ends("vref", vref);
    let toleranced = |(lo, hi): (Ohms, Ohms)| lo != hi;
    let has_tolerance = toleranced(r1_b) || toleranced(r2_b) || r3_b.is_some_and(toleranced) || vref_b.0 != vref_b.1;
    let corner_vo = |mask: u32, vi: Volts| {
        let r3 = r3_b.map(|r3| corner(mask, 2, r3));
        output(vi, corner(mask, 3, vref_b), corner(mask, 0, r1_b), corner(mask, 1, r2_b), r3)
    };
    let vo_band = |vi: Volts| extremes(4, |mask| corner_vo(mask, vi));
    let g_band = extremes(4, |mask| (corner_vo(mask, vi.1) - corner_vo(mask, vi.0)) / (vi.1 - vi.0));
    let vo_at = |x: Volts| output(x, vref, r1, r2, r3);
    let nominal = (vo_at(vi.0), vo_at(vi.1));
    let band = has_tolerance.then(|| {
        let (lo, hi) = (vo_band(vi.0), vo_band(vi.1));
        (if hi.0 < lo.0 { hi.0 } else { lo.0 }, if hi.1 > lo.1 { hi.1 } else { lo.1 })
    });

    let mut rows = vec![
        param_row("R1 [Ω]", (r1, r1_b.0, r1_b.1), d),
        param_row("R2 [Ω]", (r2, r2_b.0, r2_b.1), d),
        match (r3, r3_b) {
            (Some(r3), Some(b)) => param_row("R3 [Ω]", (r3, b.0, b.1), d),
            _ => ["R3 [Ω]", "∞", "—", "—", "—"].map(Text::from).to_vec(),
        },
        vref_row(source, vref, vref_b, d),
        range_row("Vin [V]", vi, vi, d),
    ];
    rows.extend(vout_rows((msg!("opamp.row.vout_ideal"), msg!("opamp.row.vout_with_tolerances")), nominal, band, d));
    rows.push(param_row("G [V/V]", (-r1 / r2, g_band.0, g_band.1), d));

    // The graph works in plain volts: the edge where the types end.
    let volts = Volts::new::<volt>;
    let base = |(a, b): (Volts, Volts)| (a.to_base(), b.to_base());
    let mut plot = transfer_plot("Vin", base(vi), base(vo), None, &|x| (vo.1 - a * (volts(x) - vi.0)).to_base(), &|x| {
        vo_at(volts(x)).to_base()
    });
    let band_at = |x: f64| base(vo_band(volts(x)));
    let level = (source != VrefSource::NotNeeded).then_some(vref.to_base());
    add_vref_and_band(&mut plot, level, has_tolerance.then_some(&band_at as &dyn Fn(f64) -> (f64, f64)));
    (Table { header: results_header(), rows }, plot)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> Form {
        Form::new(4, &[("vin", "-5; -1"), ("vout", "0.05; 3.3"), ("vref", "5"), ("r1", "845")])
    }

    /// The row of the results table that starts with `name`.
    fn row(table: &Table, name: &str) -> Vec<String> {
        let row = table.rows.iter().find(|r| r[0].to_string().starts_with(name)).unwrap_or_else(|| panic!("no row {name}"));
        row.iter().map(Text::to_string).collect()
    }

    fn computed(resp: &LevelShifterResponse, key: &str) -> Option<String> {
        resp.fields.iter().find(|f| f.key == key).unwrap().info.computed.clone()
    }

    #[test]
    fn control_example() {
        let resp = solve(&request());
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        let steps = resp.steps.as_ref().unwrap();
        let names: Vec<String> = steps.rows.iter().map(|r| r[0].to_string()).collect();
        assert_eq!(names, ["R1 [Ω]", "R2 [Ω]", "R3 [Ω]", "Vref [V]", "Vin [V]", "opamp.row.vout_ideal", "G [V/V]"]);
        assert_eq!(row(steps, "R2")[1], "1.0400 kΩ");
        assert_eq!(row(steps, "R3")[1], "5.5410 kΩ");
        assert_eq!(row(steps, "opamp.row.vout_ideal")[2..4], ["50.0000 mV", "3.3000 V"]);
        assert_eq!(row(steps, "G")[1], "-0.8125");
        assert_eq!(computed(&resp, "gain").as_deref(), Some("-0.8125"));
        assert_eq!(computed(&resp, "r2").as_deref(), Some("1.0400 kΩ"));
        assert_eq!(resp.plot.unwrap().lines.len(), 2);
    }

    #[test]
    fn standard_resistors_give_the_real_output() {
        let mut req = request();
        req.set("r2", "1.04k");
        req.set("r3", "5.56k");
        let steps = solve(&req).steps.unwrap();
        assert_eq!(row(&steps, "opamp.row.vout_ideal")[2..4], ["52.6079 mV", "3.3026 V"]);
    }

    #[test]
    fn any_resistor_is_the_anchor() {
        let mut req = request();
        req.set("r1", "");
        req.set("r2", "1.04k");
        let resp = solve(&req);
        assert_eq!(computed(&resp, "r1").as_deref(), Some("845.0000 Ω"));
        req.set("r2", "");
        req.set("r3", "5.541k");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(computed(&resp, "r1").unwrap().starts_with("845"));
        req.set("r3", "");
        assert!(matches!(solve(&req).outcome, Outcome::Pending(_)));
    }

    #[test]
    fn one_range_with_the_gain_and_the_resistors() {
        // Vin, G, R1, R3: Vout follows.
        let req = Form::new(4, &[("vin", "-5; -1"), ("gain", "-0.8125"), ("vref", "5"), ("r1", "845"), ("r3", "5.541k")]);
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(computed(&resp, "vout").unwrap().starts_with("50.0"));
        // Vout, R1, R2, R3: Vin and G follow.
        let req = Form::new(4, &[("vout", "0.05; 3.3"), ("vref", "5"), ("r1", "845"), ("r2", "1.04k"), ("r3", "5.541k")]);
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(computed(&resp, "vin").unwrap().starts_with("-5.0"));
        assert_eq!(computed(&resp, "gain").as_deref(), Some("-0.8125"));
        // Without R3 the offset is unknown.
        let req = Form::new(4, &[("vin", "-5; -1"), ("gain", "-0.8125"), ("vref", "5"), ("r1", "845")]);
        assert!(matches!(solve(&req).outcome, Outcome::Pending(_)));
    }

    #[test]
    fn tolerances_widen_the_output_into_a_band() {
        let mut req = request();
        req.set("r1", "845 1%");
        let resp = solve(&req);
        let steps = resp.steps.unwrap();
        // The calculated R2 and R3 take the tolerance of R1.
        let r2 = row(&steps, "R2");
        assert_eq!((r2[2].as_str(), r2[3].as_str()), ("1.0296 kΩ", "1.0504 kΩ"));
        let vout = row(&steps, "opamp.row.vout_with");
        assert!(vout[2].as_str() < "50" && vout[3].as_str() > "3.3", "{vout:?}");
        let plot = resp.plot.unwrap();
        assert_eq!(plot.lines.len(), 4);
        assert_eq!(plot.levels[0].y, 5.0);
    }

    #[test]
    fn no_offset_needs_no_r3() {
        let req = Form::new(4, &[("vin", "-2; -1"), ("vout", "2; 4"), ("vref", "5"), ("r1", "2k")]);
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert_eq!(row(resp.steps.as_ref().unwrap(), "R3")[1], "∞");
        assert_eq!(computed(&resp, "r3").as_deref(), Some("∞"));
    }

    #[test]
    fn an_empty_vref_is_recommended_or_required() {
        let mut req = request();
        req.set("vref", "");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        // C = −0.7625 V: a positive reference, the standard value nearest to 0.76 V.
        assert_eq!(row(resp.steps.as_ref().unwrap(), "level_shifter.row.vref_recommended")[1], "1.0240 V");
        assert_eq!(resp.fields.iter().find(|f| f.key == "vref").unwrap().hint.to_string(), "1.024");
        // 0…1 V to 0…3 V needs a negative one.
        let resp = solve(&Form::new(4, &[("vin", "1"), ("vout", "3"), ("r1", "10k")]));
        assert_eq!(row(resp.steps.as_ref().unwrap(), "level_shifter.row.vref_recommended")[1], "-3.0000 V");
        // R1 and R3 fix it: Vref = −C·R3/R1.
        req.set("r3", "5.541k");
        let resp = solve(&req);
        assert!(row(resp.steps.as_ref().unwrap(), "level_shifter.row.vref_required")[1].starts_with("5.000"));
        // No offset: no reference, no level on the plot.
        let resp = solve(&Form::new(4, &[("vin", "-2; -1"), ("vout", "2; 4"), ("r1", "2k")]));
        assert_eq!(row(resp.steps.as_ref().unwrap(), "level_shifter.row.vref")[1], "level_shifter.vref_not_needed");
        assert!(resp.plot.unwrap().levels.is_empty());
        req.set("vin", "");
        match solve(&req).outcome {
            Outcome::Pending(t) => assert_eq!(t.id(), Some("level_shifter.fill_in_vref")),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn errors() {
        let outcome = |key: &str, text: &str| {
            let mut req = request();
            req.set(key, text);
            solve(&req).outcome
        };
        let id = |o: Outcome| match o {
            Outcome::Error(t) | Outcome::Pending(t) => t.id().map(str::to_owned),
            _ => None,
        };
        assert_eq!(id(outcome("vref", "-5")).as_deref(), Some("level_shifter.err.wrong_reference_sign"));
        assert_eq!(id(outcome("vref", "0")).as_deref(), Some("level_shifter.err.no_reference"));
        assert_eq!(id(outcome("r1", "-1")).as_deref(), Some("level_shifter.err.positive"));
        assert_eq!(id(outcome("vin", "-1; -5")).as_deref(), Some("level_shifter.err.empty_span"));
        assert!(matches!(outcome("gain", "2"), Outcome::FixErrors(_)));
    }
}

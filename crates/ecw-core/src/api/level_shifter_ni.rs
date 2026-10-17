//! Level shifter on a non-inverting amplifier with a reference voltage (TI SBOA262A), laid out
//! like the difference amplifier: the fields sit on the schematic, any two of Vin, Vout and G
//! (with the offset from R1/R2 and Vref) give the third, R1 or R2 and R3 or R4 give the other
//! resistors, and the tolerances of the resistors and Vref widen the output into a worst-case
//! band.

use super::Form;
use crate::msg;
use serde::Serialize;
use uom::num_traits::Zero;
use uom::si::electric_potential::volt;
use uom::si::f64::{ElectricPotential, ElectricalResistance, Ratio};
use uom::si::ratio::ratio;

use super::report::{
    DesignForm, Input, Spec, VrefSource, add_vref_and_band, corner, empty_plot, extremes, param_row, range_row,
    range_text, results_header, sig, spec, spec_in, transfer_plot, value_text, vout_rows, vref_row,
};
use super::{Check, GroupedField, Outcome, PlotData, Table};
use crate::domain::infer::{Values, infer};
use crate::domain::unit::Measured;
use crate::domain::{FieldState, Quantity, Text};
use crate::level_shifter::non_inverting::{
    COUNT, G, R1, R2, R3, R4, VI_MAX, VI_MIN, VO_MAX, VO_MIN, VREF, rules,
};
use crate::level_shifter::{
    NonInvertingError, ideal_non_inverting, output_non_inverting, recommend_non_inverting_reference,
};

type Volts = ElectricPotential;
type Ohms = ElectricalResistance;

const VIN: Spec = spec("vin", "Vin min; max [V]", "2; 5", "signal", Quantity::Voltage, "V");
const VOUT: Spec = spec("vout", "Vout min; max [V]", "0.05; 4.95", "signal", Quantity::Voltage, "V");
const GAIN: Spec = spec("gain", "G [V/V]", "1.63", "signal", Quantity::Ratio, "");

const SPECS: [Spec; 5] = [
    // Vref and the resistors may carry a tolerance (`10k 1%`).
    spec_in("vref", "Vref [V]", "2.5", "circuit", Input::Tolerant, Quantity::Voltage, "V"),
    spec_in("r1", "R1 [Ω]", "1k", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
    spec_in("r2", "R2 [Ω]", "777", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
    spec_in("r3", "R3 [Ω]", "402", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
    spec_in("r4", "R4 [Ω]", "1k", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
];

/// The reply of the non-inverting level shifter page.
#[derive(Debug, Serialize)]
pub struct LevelShifterNiResponse {
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

fn message(error: NonInvertingError, vref: Volts) -> Text {
    let r = |x: Ratio| sig(x.get::<ratio>());
    match error {
        NonInvertingError::EmptySpan => msg!("level_shifter.err.empty_span"),
        NonInvertingError::NoReference => msg!("level_shifter.err.no_reference"),
        NonInvertingError::NoOffset => msg!("level_shifter_ni.err.no_offset"),
        NonInvertingError::ReferenceSign { .. } => {
            msg!("level_shifter.err.wrong_reference_sign", vref = sig(vref.to_base()), opposite = sig(-vref.to_base()))
        }
        NonInvertingError::R3NotPositive { r1_over_r2, g_in, vref_limit } => msg!(
            "level_shifter_ni.err.r3_not_positive",
            sum = sig(1.0 + r1_over_r2.get::<ratio>()),
            g_in = r(g_in),
            hint = vref_limit.map_or(Text::default(), |l| msg!("level_shifter_ni.err.vref_limit", limit = sig(l.to_base()))),
        ),
    }
}

/// Solves the non-inverting level shifter from the given fields and lays out the page: fields
/// on the schematic, results, and the plot of Vo against Vin with the worst-case band.
pub fn solve(req: &Form) -> LevelShifterNiResponse {
    let gain_state = FieldState::parse_exact(req.text("gain"), Quantity::Ratio);
    let mut form = DesignForm::parse(req, &SPECS, &[VIN, VOUT], (&GAIN, gain_state));
    let (outcome, steps, plot) = match design(&mut form, req.decimals) {
        Ok((steps, plot)) => (Outcome::Solved, Some(steps), plot),
        // Empty axes until the form is solved, as for the other amplifiers.
        Err(outcome) => (outcome, None, empty_plot("Vin", "Vo")),
    };
    LevelShifterNiResponse { fields: form.fields, outcome, steps, checks: Vec::new(), notes: Vec::new(), plot: Some(plot) }
}

/// What the user typed, in the types of the circuit.
struct Given {
    vin: Option<(Volts, Volts)>,
    vout: Option<(Volts, Volts)>,
    vref: Option<Volts>,
    gain: Option<Ratio>,
    r: [Option<Ohms>; 4],
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
        values.set_exact(G, self.gain.filter(|_| !both));
        for (var, r) in [R1, R2, R3, R4].into_iter().zip(self.r) {
            values.set_exact(var, r);
        }
        infer(&mut values, &rules()).map_err(|e| Outcome::Error(e.text()))?;
        Ok(values)
    }
}

fn design(form: &mut DesignForm, d: u32) -> Result<(Table, PlotData), Outcome> {
    if form.has_error() {
        return Err(Outcome::FixErrors(msg!("common.fix_errors")));
    }
    let keys = ["r1", "r2", "r3", "r4"];
    let given = Given {
        vin: form.range("vin"),
        vout: form.range("vout"),
        vref: form.value("vref"),
        gain: form.value("gain"),
        r: keys.map(|key| form.value(key)),
    };
    for (r, name) in given.r.iter().zip(["R1", "R2", "R3", "R4"]) {
        if r.is_some_and(|r| r <= Ohms::zero()) {
            return Err(Outcome::Error(msg!("level_shifter.err.positive", name = name)));
        }
    }
    if given.gain.is_some_and(|g| g <= Ratio::zero()) {
        return Err(Outcome::Error(msg!("level_shifter_ni.err.gain_positive")));
    }

    // An empty Vref is found from the two ranges: the value the given R1 and R2 require, or
    // else a standard one of the sign the offset needs and small enough to keep R3 positive.
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
            let (vref, source) = match recommend_non_inverting_reference(vi, vo) {
                Some(vref) => (vref, VrefSource::Recommended),
                // No offset to make (or ranges out of order): the design reports it.
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
        return Err(Outcome::Pending(msg!("level_shifter_ni.fill_in")));
    };
    // Whatever the other fields settled and the user left empty is locked and shown computed.
    if given.vin.is_none() {
        form.set_computed("vin", range_text(vi, d));
    }
    if given.vout.is_none() {
        form.set_computed("vout", range_text(vo, d));
    }
    let design = ideal_non_inverting(vi, vo, vref).map_err(|e| Outcome::Error(message(e, vref)))?;
    if given.gain.is_none() {
        form.set_computed("gain", value_text(design.g_in, d));
    }

    // Each pair of resistors needs one of its two: R1 or R2 for the feedback, R3 or R4 for the
    // input divider. The given one of each pair is the anchor of the other's tolerance.
    let r = [R1, R2, R3, R4].map(|var| values.nominal(var));
    let [Some(r1), Some(r2), Some(r3), Some(r4)] = r else {
        return Err(Outcome::Pending(msg!("level_shifter_ni.fill_in_resistors")));
    };
    for (key, (value, typed)) in keys.iter().zip([r1, r2, r3, r4].into_iter().zip(given.r)) {
        if typed.is_none() {
            form.set_computed(key, value_text(value, d));
        }
    }
    let anchors = (
        if given.r[0].is_some() { "r1" } else { "r2" },
        if given.r[3].is_some() { "r4" } else { "r3" },
    );
    Ok(build(form, Circuit { vi, vo, vref, source, g: design.g_in, r: [r1, r2, r3, r4], anchors }, d))
}

struct Circuit {
    vi: (Volts, Volts),
    vo: (Volts, Volts),
    vref: Volts,
    source: VrefSource,
    /// The gain of the design (from the ranges).
    g: Ratio,
    r: [Ohms; 4],
    /// The given resistor of each pair, whose tolerance the other one takes.
    anchors: (&'static str, &'static str),
}

/// Worst case: R1…R4 and Vref each move within their own tolerance; a calculated resistor
/// takes the relative tolerance of the other one of its pair. Vo is monotonic in every one of
/// them, so the extremes are at the corners.
fn build(form: &DesignForm, c: Circuit, d: u32) -> (Table, PlotData) {
    let Circuit { vi, vo, vref, source, g, r: [r1, r2, r3, r4], anchors } = c;
    let ends = |key: &str, value: Ohms, anchor: &str| {
        if form.value::<Ohms>(key).is_some() {
            form.ends(key, value)
        } else {
            let rel = form.relative(anchor);
            (value * rel.0, value * rel.1)
        }
    };
    let (r1_b, r2_b) = (ends("r1", r1, anchors.0), ends("r2", r2, anchors.0));
    let (r3_b, r4_b) = (ends("r3", r3, anchors.1), ends("r4", r4, anchors.1));
    let vref_b = form.ends("vref", vref);
    let has_tolerance = [r1_b, r2_b, r3_b, r4_b].iter().any(|(lo, hi)| lo != hi) || vref_b.0 != vref_b.1;
    let corner_vo = |mask: u32, vi: Volts| {
        let pick = |bit, b| corner(mask, bit, b);
        output_non_inverting(vi, corner(mask, 4, vref_b), pick(0, r1_b), pick(1, r2_b), pick(2, r3_b), pick(3, r4_b))
    };
    let vo_band = |vi: Volts| extremes(5, |mask| corner_vo(mask, vi));
    let g_band = extremes(5, |mask| (corner_vo(mask, vi.1) - corner_vo(mask, vi.0)) / (vi.1 - vi.0));
    let vo_at = |x: Volts| output_non_inverting(x, vref, r1, r2, r3, r4);
    let nominal = (vo_at(vi.0), vo_at(vi.1));
    let band = has_tolerance.then(|| {
        let (lo, hi) = (vo_band(vi.0), vo_band(vi.1));
        (if hi.0 < lo.0 { hi.0 } else { lo.0 }, if hi.1 > lo.1 { hi.1 } else { lo.1 })
    });

    let mut rows = vec![
        param_row("R1 [Ω]", (r1, r1_b.0, r1_b.1), d),
        param_row("R2 [Ω]", (r2, r2_b.0, r2_b.1), d),
        param_row("R3 [Ω]", (r3, r3_b.0, r3_b.1), d),
        param_row("R4 [Ω]", (r4, r4_b.0, r4_b.1), d),
        vref_row(source, vref, vref_b, d),
        range_row("Vin [V]", vi, vi, d),
    ];
    rows.extend(vout_rows((msg!("opamp.row.vout_ideal"), msg!("opamp.row.vout_with_tolerances")), nominal, band, d));
    let g_real = r4 / (r3 + r4) * (r1 + r2) / r2;
    rows.push(param_row("G [V/V]", (g_real, g_band.0, g_band.1), d));

    // The graph works in plain volts: the edge where the types end.
    let volts = Volts::new::<volt>;
    let base = |(a, b): (Volts, Volts)| (a.to_base(), b.to_base());
    let mut plot = transfer_plot("Vin", base(vi), base(vo), None, &|x| (vo.0 + g * (volts(x) - vi.0)).to_base(), &|x| {
        vo_at(volts(x)).to_base()
    });
    let band_at = |x: f64| base(vo_band(volts(x)));
    add_vref_and_band(&mut plot, Some(vref.to_base()), has_tolerance.then_some(&band_at as &dyn Fn(f64) -> (f64, f64)));
    (Table { header: results_header(), rows }, plot)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> Form {
        Form::new(4, &[("vin", "2; 5"), ("vout", "0.05; 4.95"), ("vref", "2.5"), ("r1", "1k"), ("r4", "1k")])
    }

    /// The row of the results table that starts with `name`.
    fn row(table: &Table, name: &str) -> Vec<String> {
        let row = table.rows.iter().find(|r| r[0].to_string().starts_with(name)).unwrap_or_else(|| panic!("no row {name}"));
        row.iter().map(Text::to_string).collect()
    }

    fn computed(resp: &LevelShifterNiResponse, key: &str) -> Option<String> {
        resp.fields.iter().find(|f| f.key == key).unwrap().info.computed.clone()
    }

    #[test]
    fn control_example() {
        let resp = solve(&request());
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        let steps = resp.steps.as_ref().unwrap();
        let names: Vec<String> = steps.rows.iter().map(|r| r[0].to_string()).collect();
        assert_eq!(names, ["R1 [Ω]", "R2 [Ω]", "R3 [Ω]", "R4 [Ω]", "Vref [V]", "Vin [V]", "opamp.row.vout_ideal", "G [V/V]"]);
        assert!(row(steps, "R2")[1].starts_with("777.2"));
        assert!(row(steps, "R3")[1].starts_with("400."));
        assert_eq!(row(steps, "opamp.row.vout_ideal")[2..4], ["50.0000 mV", "4.9500 V"]);
        assert_eq!(computed(&resp, "gain").as_deref(), Some("1.6333"));
        assert!(computed(&resp, "r2").is_some() && computed(&resp, "r3").is_some());
    }

    #[test]
    fn standard_resistors_give_the_real_output() {
        let mut req = request();
        req.set("r2", "777");
        req.set("r3", "402");
        let steps = solve(&req).steps.unwrap();
        let vout = row(&steps, "opamp.row.vout_ideal");
        assert!(vout[2].starts_with("44.") && vout[3].starts_with("4.93"), "{vout:?}");
    }

    #[test]
    fn the_other_resistor_of_a_pair_is_the_anchor() {
        let mut req = request();
        req.set("r1", "");
        req.set("r2", "777.2");
        req.set("r4", "");
        req.set("r3", "400");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(computed(&resp, "r1").unwrap().starts_with("999.") || computed(&resp, "r1").unwrap().starts_with("1.0000 k"));
        assert!(computed(&resp, "r4").is_some());
        req.set("r3", "");
        match solve(&req).outcome {
            Outcome::Pending(t) => assert_eq!(t.id(), Some("level_shifter_ni.fill_in_resistors")),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn one_range_with_the_gain_and_the_resistors() {
        // Vin, R1…R4: G and Vout follow.
        let req = Form::new(4, &[("vin", "2; 5"), ("vref", "2.5"), ("r1", "1k"), ("r2", "777"), ("r3", "402"), ("r4", "1k")]);
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(computed(&resp, "vout").is_some() && computed(&resp, "gain").is_some());
        // Vout, G, R1, R2: Vin follows.
        let req = Form::new(4, &[("vout", "0.05; 4.95"), ("gain", "1.6333"), ("vref", "2.5"), ("r1", "1k"), ("r2", "777.2"), ("r4", "1k")]);
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(computed(&resp, "vin").unwrap().starts_with("2.0"));
        // Vin and G alone leave the offset unknown.
        let req = Form::new(4, &[("vin", "2; 5"), ("gain", "1.6333"), ("vref", "2.5"), ("r1", "1k")]);
        assert!(matches!(solve(&req).outcome, Outcome::Pending(_)));
    }

    #[test]
    fn tolerances_widen_the_output_into_a_band() {
        let mut req = request();
        req.set("r1", "1k 1%");
        req.set("vref", "2.5 1%");
        let resp = solve(&req);
        let steps = resp.steps.as_ref().unwrap();
        // R2 takes the tolerance of R1; R3 and R4 have none.
        let r2 = row(steps, "R2");
        assert_ne!(r2[2], "—");
        assert_eq!(row(steps, "R4")[2], "—");
        assert_eq!(steps.rows.iter().filter(|r| r[0].to_string().starts_with("opamp.row.vout")).count(), 2);
        assert_eq!(resp.plot.unwrap().lines.len(), 4);
    }

    #[test]
    fn an_empty_vref_is_recommended_or_required() {
        let mut req = request();
        req.set("vref", "");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        // Half the 5.08 V limit: the 2.5 V of the method; it is the hint of the field.
        assert_eq!(row(resp.steps.as_ref().unwrap(), "level_shifter.row.vref_recommended")[1], "2.5000 V");
        let vref = resp.fields.iter().find(|f| f.key == "vref").unwrap();
        assert_eq!((vref.hint.to_string().as_str(), vref.info.computed.is_none()), ("2.5", true));
        // R1 and R2 fix it: Vref = D·R2/R1.
        req.set("r2", "777.2");
        let resp = solve(&req);
        let vref = row(resp.steps.as_ref().unwrap(), "level_shifter.row.vref_required");
        assert!(vref[1].starts_with("2.50"), "{vref:?}");
        // Without both ranges there is nothing to choose from.
        req.set("vout", "");
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
        assert_eq!(id(outcome("vref", "-2.5")).as_deref(), Some("level_shifter.err.wrong_reference_sign"));
        assert_eq!(id(outcome("vref", "10")).as_deref(), Some("level_shifter_ni.err.r3_not_positive"));
        assert_eq!(id(outcome("vref", "0")).as_deref(), Some("level_shifter.err.no_reference"));
        assert_eq!(id(outcome("vout", "1.98; 4.95")).as_deref(), Some("level_shifter_ni.err.no_offset"));
        assert_eq!(id(outcome("r4", "-1")).as_deref(), Some("level_shifter.err.positive"));
        assert_eq!(id(outcome("gain", "-2")), None.or(Some("level_shifter_ni.err.gain_positive".to_owned())));
        assert_eq!(id(outcome("vin", "5; 2")).as_deref(), Some("level_shifter.err.empty_span"));
    }
}

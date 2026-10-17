//! Difference amplifier: gain, resistors, reference and output range.

use super::Form;
use crate::msg;
use serde::Serialize;
use uom::num_traits::Zero;
use uom::si::electric_potential::volt;
use uom::si::f64::{ElectricPotential, ElectricalResistance, Ratio};

use super::report::{
    DesignForm, Input, Spec, add_vref_and_band, corner, empty_plot, extremes, param_row, range_row, range_text,
    results_header, spec, spec_in, transfer_plot, value_text, vout_rows,
};
use super::{Check, GroupedField, Outcome, PlotData, Table};
use crate::domain::infer::{Values, infer};
use crate::domain::solve::ascending;
use crate::domain::unit::Measured;
use crate::domain::{FieldState, Quantity};
use crate::opamp_differential::{COUNT, G, R1, R3, VD_MAX, VD_MIN, VO_MAX, VO_MIN, VREF, gain, output, rules};

type Volts = ElectricPotential;
type Ohms = ElectricalResistance;

/// `Vindiff` and `Vout` are fields of two ends each (`min; max`).
const RANGES: [Spec; 2] = [
    spec("vdiff", "Vindiff min; max [V]", "-1.25; 1.25", "signal", Quantity::Voltage, "V"),
    spec("vo", "Vout min; max [V]", "-2.5; 2.5", "signal", Quantity::Voltage, "V"),
];

const GAIN: Spec = spec("gain", "G [V/V]", "2", "signal", Quantity::Ratio, "");

const SPECS: [Spec; 3] = [
    // Vref and the resistors may carry a tolerance (`10k 1%`): the nominal value is the design
    // point and the tolerance widens the output into a worst-case band.
    spec_in("vref", "Vref [V]", "0", "circuit", Input::Tolerant, Quantity::Voltage, "V"),
    spec_in("r1", "R1 [Ω]", "10k", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
    spec_in("r3", "R3 [Ω]", "20k", "circuit", Input::Tolerant, Quantity::Resistance, "Ω"),
];

/// The reply of the difference amplifier page.
#[derive(Debug, Serialize)]
pub struct DifferentialResponse {
    /// The fields on the schematic: the signal ranges, the reference and the resistors.
    pub fields: Vec<GroupedField>,
    /// What to show in the results area.
    pub outcome: Outcome,
    /// The results table (Nom, Min, Max, Δ), once the page is solved.
    pub steps: Option<Table>,
    /// The checks under the results. Empty for now: they were taken out and may come back.
    pub checks: Vec<Check>,
    /// The notes under the results. Empty for now, as the checks.
    pub notes: Vec<String>,
    /// Vo against Vdiff.
    pub plot: Option<PlotData>,
}

/// Solves the difference amplifier from the given fields and lays out the page: fields on the
/// schematic, results, and the plot of Vo against the input difference with the worst-case band.
pub fn solve(req: &Form) -> DifferentialResponse {
    let gain_state = FieldState::parse_exact(req.text("gain"), Quantity::Ratio);
    let mut form = DesignForm::parse(req, &SPECS, &RANGES, (&GAIN, gain_state));
    let (outcome, steps, plot) = match design(&mut form, req.decimals) {
        Ok((steps, plot)) => (Outcome::Solved, Some(steps), plot),
        // Empty axes until the form is solved, as for the single-ended amplifiers.
        Err(outcome) => (outcome, None, empty_plot("Vdiff", "Vo")),
    };
    DifferentialResponse { fields: form.fields, outcome, steps, checks: Vec::new(), notes: Vec::new(), plot: Some(plot) }
}

/// Any two of Vindiff, Vout and G give the third (`Vo = Vdiff·G + Vref`), and R3 with R1 stands
/// in for the gain (`G = R3/R1`). A field that is computed is locked on the schematic and its
/// value goes to the results; a value the user typed stays as typed.
fn design(form: &mut DesignForm, d: u32) -> Result<(Table, PlotData), Outcome> {
    if form.has_error() {
        return Err(Outcome::FixErrors(msg!("common.fix_errors")));
    }
    let Some(vref) = form.value::<Volts>("vref") else {
        return Err(Outcome::Pending(msg!("report.fill_in", fields = "Vref")));
    };
    let (vd_given, vo_given) = (form.range::<Volts>("vdiff"), form.range::<Volts>("vo"));
    let gain_typed = form.value::<Ratio>("gain");
    let (r1_typed, r3_typed) = (form.value::<Ohms>("r1"), form.value::<Ohms>("r3"));
    let both = vd_given.is_some() && vo_given.is_some();

    let mut values = Values::new(COUNT);
    values.set_exact(VREF, Some(vref));
    for ((lo, hi), range) in [((VD_MIN, VD_MAX), vd_given), ((VO_MIN, VO_MAX), vo_given)] {
        values.set_exact(lo, range.map(|r| r.0));
        values.set_exact(hi, range.map(|r| r.1));
    }
    // With both ranges given, they fix the gain and a typed G is not used.
    values.set_exact(G, gain_typed.filter(|_| !both));
    values.set_exact(R1, r1_typed);
    values.set_exact(R3, r3_typed);
    infer(&mut values, &rules()).map_err(|e| Outcome::Error(e.text()))?;

    if !both && values.nominal(G).is_some_and(|g| g <= Ratio::zero()) {
        return Err(Outcome::Error(msg!("opamp_differential.err.gain_positive")));
    }
    let range = |lo, hi| values.nominal(lo).zip(values.nominal(hi));
    let (Some(vd), Some(vo)) = (range(VD_MIN, VD_MAX), range(VO_MIN, VO_MAX)) else {
        return Err(Outcome::Pending(msg!("report.fill_in_two")));
    };
    if vd_given.is_none() {
        form.set_computed("vdiff", range_text(vd, d));
    }
    if vo_given.is_none() {
        form.set_computed("vo", range_text(vo, d));
    }
    if gain_typed.is_none() {
        form.set_computed("gain", value_text(gain(vd, vo), d));
    }
    if !ascending(vd) || !ascending(vo) {
        return Err(Outcome::Error(msg!("opamp_differential.err.ranges_ordered")));
    }
    // R1 may be left out when R3 is given: the gain then sets it (R1 = R3/G).
    let Some(r1) = values.nominal(R1) else {
        return Err(Outcome::Pending(msg!("report.fill_in", fields = "R1, R3")));
    };
    if r1_typed.is_none() {
        form.set_computed("r1", value_text(r1, d));
    }
    if r1 <= Ohms::zero() {
        return Err(Outcome::Error(msg!("opamp_differential.err.r1_positive")));
    }
    if r3_typed.is_some_and(|r3| r3 <= Ohms::zero()) {
        return Err(Outcome::Error(msg!("opamp_differential.err.r3_positive")));
    }
    let r3 = values.nominal(R3).expect("R1 and the gain give R3");
    if r3_typed.is_none() {
        form.set_computed("r3", value_text(r3, d));
    }
    Ok(build(form, Inputs { vd, vo, vref, r1, r3 }, d))
}

struct Inputs {
    vd: (Volts, Volts),
    vo: (Volts, Volts),
    vref: Volts,
    r1: Ohms,
    r3: Ohms,
}

/// R1 = R2, R3 = R4, Vin, Vout and G, as in the other calculators: Nom, Min, Max, Δ. For a
/// voltage range Nom is the mean of its two ends and Min and Max are the ends; for Vout with
/// tolerances they are the lowest and the highest value it can reach.
fn build(form: &DesignForm, inp: Inputs, d: u32) -> (Table, PlotData) {
    let Inputs { vd, vo, vref, r1, r3 } = inp;
    let g = gain(vd, vo);
    let g_real = r3 / r1;
    let vo_at = |vdiff: Volts| output(Volts::zero(), vdiff, vref, r1, r1, r3, r3);

    // Worst case: Vref, R1, R2, R3 and R4 each move within their own tolerance. Vo is monotonic
    // in every one of them, so the extremes are at the corners. A resistor that is calculated
    // takes the relative tolerance of the other one.
    let scaled = |from: &str, value: Ohms| {
        let (lo, hi) = form.relative(from);
        (value * lo, value * hi)
    };
    let given = |key: &str| form.value::<Ohms>(key).is_some();
    let r1_b = if given("r1") { form.ends("r1", r1) } else { scaled("r3", r1) };
    let r3_b = if given("r3") { form.ends("r3", r3) } else { scaled("r1", r3) };
    let vref_b = form.ends("vref", vref);
    let has_tolerance = r1_b.0 != r1_b.1 || r3_b.0 != r3_b.1 || vref_b.0 != vref_b.1;
    let corner_vo = |mask: u32, vdiff: Volts| {
        let (r1, r2) = (corner(mask, 0, r1_b), corner(mask, 1, r1_b));
        let (r3, r4) = (corner(mask, 2, r3_b), corner(mask, 3, r3_b));
        output(Volts::zero(), vdiff, corner(mask, 4, vref_b), r1, r2, r3, r4)
    };
    let vo_band = |vdiff: Volts| extremes(5, |mask| corner_vo(mask, vdiff));
    // Vref cancels in the gain, so G is taken corner by corner, not from the two bands.
    let g_band = extremes(5, |mask| (corner_vo(mask, vd.1) - corner_vo(mask, vd.0)) / (vd.1 - vd.0));
    let nominal = (vo_at(vd.0), vo_at(vd.1));
    let band = has_tolerance.then(|| {
        let (lo, hi) = (vo_band(vd.0), vo_band(vd.1));
        (if hi.0 < lo.0 { hi.0 } else { lo.0 }, if hi.1 > lo.1 { hi.1 } else { lo.1 })
    });

    let mut rows = vec![
        param_row("R1 = R2 [Ω]", (r1, r1_b.0, r1_b.1), d),
        param_row("R3 = R4 [Ω]", (r3, r3_b.0, r3_b.1), d),
        range_row("Vin [V]", vd, vd, d),
    ];
    rows.extend(vout_rows(
        (msg!("opamp_differential.row.vout_ideal"), msg!("opamp_differential.row.vout_with_tolerances")),
        nominal,
        band,
        d,
    ));
    rows.push(param_row("G [V/V]", (g_real, g_band.0, g_band.1), d));

    // The graph works in plain volts: the edge where the types end.
    let volts = Volts::new::<volt>;
    let base = |(a, b): (Volts, Volts)| (a.to_base(), b.to_base());
    let mut plot = transfer_plot(
        "Vdiff",
        base(vd),
        base(vo),
        None,
        &|x| (vo.0 + g * (volts(x) - vd.0)).to_base(),
        &|x| vo_at(volts(x)).to_base(),
    );
    let band_at = |x: f64| base(vo_band(volts(x)));
    add_vref_and_band(&mut plot, Some(vref.to_base()), has_tolerance.then_some(&band_at as &dyn Fn(f64) -> (f64, f64)));
    (Table { header: results_header(), rows }, plot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Text;

    fn request() -> Form {
        Form::new(4, &[("vdiff", "-1.25; 1.25"), ("vo", "-2.5; 2.5"), ("vref", "0"), ("r1", "10k")])
    }

    /// The row of the results table that starts with `name`.
    fn row(table: &Table, name: &str) -> Vec<String> {
        let row = table.rows.iter().find(|r| r[0].to_string().starts_with(name)).unwrap_or_else(|| panic!("no row {name}"));
        row.iter().map(Text::to_string).collect()
    }

    #[test]
    fn control_example() {
        let resp = solve(&request());
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        let steps = resp.steps.unwrap();
        let names: Vec<String> = steps.rows.iter().map(|r| r[0].to_string()).collect();
        assert_eq!(names, ["R1 = R2 [Ω]", "R3 = R4 [Ω]", "Vin [V]", "opamp_differential.row.vout_ideal", "G [V/V]"]);
        assert_eq!(row(&steps, "R1")[1], "10.0000 kΩ");
        assert_eq!(row(&steps, "R3")[1], "20.0000 kΩ");
        // A range: Nom is the mean, Min and Max the ends.
        assert_eq!(row(&steps, "Vin")[1..4], ["0.0000 V", "-1.2500 V", "1.2500 V"]);
        assert_eq!(row(&steps, "opamp_differential.row.vout")[1..4], ["0.0000 V", "-2.5000 V", "2.5000 V"]);
        assert_eq!(row(&steps, "G")[1], "2.0000");
        assert!(resp.plot.is_some());
    }

    #[test]
    fn resistors_accept_a_tolerance() {
        let req = Form::new(4, &[("vdiff", "-1; 1"), ("vo", "-2; 2"), ("vref", "0"), ("r1", "10k 1%"), ("r3", "20k 5%")]);
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved));
        assert!(resp.fields.iter().all(|f| f.info.status != "error"));
    }

    fn req_vref_only() -> Form {
        Form::new(4, &[("vdiff", "-1; 1"), ("vo", "-2; 2"), ("vref", "0"), ("r1", "10k"), ("r3", "20k")])
    }

    #[test]
    fn tolerances_widen_the_output_into_a_worst_case_band() {
        let req = Form::new(4, &[("vdiff", "-1; 1"), ("vo", "-2; 2"), ("vref", "0"), ("r1", "10k 1%"), ("r3", "20k 1%")]);
        let resp = solve(&req);
        let steps = resp.steps.unwrap();
        assert_eq!(steps.header.len(), 5);
        // Ideal: the ends of the range as computed; with tolerances: the lowest and the highest
        // the output can reach.
        assert_eq!(row(&steps, "opamp_differential.row.vout_ideal")[2], "-2.0000 V");
        let vout = row(&steps, "opamp_differential.row.vout_with");
        assert_eq!(vout[1], "0.0000 V");
        assert_eq!((vout[2].as_str(), vout[3].as_str()), ("-2.0404 V", "2.0404 V"));
        let r1 = row(&steps, "R1");
        assert_eq!((r1[1].as_str(), r1[2].as_str(), r1[3].as_str()), ("10.0000 kΩ", "9.9000 kΩ", "10.1000 kΩ"));
        assert_eq!(resp.plot.unwrap().lines.len(), 4);

        // A tolerance on Vref alone widens the band too, and Vref has its own level.
        let mut vref_only = req_vref_only();
        vref_only.set("vref", "1 5%");
        let resp = solve(&vref_only);
        let plot = resp.plot.unwrap();
        assert_eq!(plot.lines.len(), 4);
        assert_eq!(plot.levels.len(), 1);
        assert_eq!(plot.levels[0].y, 1.0);

        // Without tolerances there is no band.
        let mut exact = req;
        exact.set("r1", "10k");
        exact.set("r3", "20k");
        let resp = solve(&exact);
        assert_eq!(resp.steps.unwrap().rows.iter().filter(|r| r[0].to_string().starts_with("opamp_differential.row.vout")).count(), 1);
        assert_eq!(resp.plot.unwrap().lines.len(), 2);
    }

    #[test]
    fn r1_and_r3_are_enough_with_one_signal_range() {
        let mut req = request();
        req.set("vo", "");
        req.set("vdiff", "1; 2");
        req.set("r3", "5k");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        let vout = resp.fields.iter().find(|f| f.key == "vo").unwrap();
        assert_eq!(vout.info.computed.as_deref(), Some("500.0000 mV; 1.0000 V"));

        let mut req = request();
        req.set("vdiff", "");
        req.set("vo", "-1; 1");
        req.set("r3", "5k");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
    }

    #[test]
    fn r1_follows_from_r3_and_the_signal_ranges() {
        let mut req = request();
        req.set("r1", "");
        req.set("r3", "20k");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        let r1 = resp.fields.iter().find(|f| f.key == "r1").unwrap();
        assert_eq!(r1.info.computed.as_deref(), Some("10.0000 kΩ"));

        req.set("r3", "");
        assert!(matches!(solve(&req).outcome, Outcome::Pending(_)));
    }

    /// Which of Vindiff, Vout, G, R1 are locked for the given inputs.
    fn locked(vdiff: &str, vo: &str, gain: &str, r1: &str, r3: &str) -> Vec<&'static str> {
        let req = Form::new(4, &[("vdiff", vdiff), ("vo", vo), ("gain", gain), ("vref", "0"), ("r1", r1), ("r3", r3)]);
        let resp = solve(&req);
        let mut keys: Vec<_> = resp.fields.iter().filter(|f| f.info.computed.is_some()).map(|f| f.key).collect();
        keys.sort_by_key(|k| ["vdiff", "vo", "gain", "r1", "r3"].iter().position(|x| x == k));
        keys
    }

    #[test]
    fn empty_fields_that_follow_from_the_others_are_locked() {
        // Vindiff, R1, R3: G = R3/R1 and Vout follow (R3 and R1 are both typed).
        assert_eq!(locked("1; 2", "", "", "10k", "5k"), ["vo", "gain"]);
        // Vout, R1, R3.
        assert_eq!(locked("", "-1; 1", "", "10k", "5k"), ["vdiff", "gain"]);
        // Vindiff, Vout, R1: G and R3 follow.
        assert_eq!(locked("-1; 1", "-2; 2", "", "10k", ""), ["gain", "r3"]);
        // Vindiff, Vout, R3: G and R1 follow.
        assert_eq!(locked("-1; 1", "-2; 2", "", "", "20k"), ["gain", "r1"]);
        // Vindiff, G, R1.
        assert_eq!(locked("-1; 1", "", "2", "10k", ""), ["vo", "r3"]);
        // Vout, G, R3: Vindiff and R1 follow.
        assert_eq!(locked("", "-2; 2", "2", "", "20k"), ["vdiff", "r1"]);
        // Everything typed: nothing is overwritten.
        assert!(locked("-1; 1", "-2; 2", "2", "10k", "20k").is_empty());
    }

    #[test]
    fn one_number_is_the_range_from_zero() {
        let mut req = request();
        req.set("vdiff", "1.25");
        req.set("vo", "5");
        req.set("r1", "10k");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        let steps = resp.steps.unwrap();
        assert_eq!(row(&steps, "Vin")[1..4], ["625.0000 mV", "0.0000 V", "1.2500 V"]);
        assert_eq!(row(&steps, "opamp_differential.row.vout_ideal")[2..4], ["0.0000 V", "5.0000 V"]);
    }

    #[test]
    fn a_calculated_resistor_takes_the_tolerance_of_the_other() {
        let mut req = request();
        req.set("r1", "10k 1%");
        let steps = solve(&req).steps.unwrap();
        let r3 = row(&steps, "R3");
        assert_eq!((r3[1].as_str(), r3[2].as_str(), r3[3].as_str()), ("20.0000 kΩ", "19.8000 kΩ", "20.2000 kΩ"));

        req.set("r1", "");
        req.set("r3", "20k 1%");
        let steps = solve(&req).steps.unwrap();
        let r1 = row(&steps, "R1");
        assert_eq!((r1[2].as_str(), r1[3].as_str()), ("9.9000 kΩ", "10.1000 kΩ"));
    }

    #[test]
    fn a_given_r3_replaces_the_computed_one() {
        let mut req = request();
        req.set("r3", "22k");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        let steps = resp.steps.unwrap();
        assert_eq!(row(&steps, "R3")[1], "22.0000 kΩ");
        assert_eq!(row(&steps, "G")[1], "2.2000");
        req.set("r3", "-1");
        assert!(matches!(solve(&req).outcome, Outcome::Error(_)));
    }

    #[test]
    fn the_plot_has_no_supply_rails() {
        let plot = solve(&request()).plot.unwrap();
        assert!(plot.guides.is_empty());
        assert!(plot.y[0] <= -2.5 && plot.y[1] >= 2.5, "{:?}", plot.y);
    }

    #[test]
    fn ranges_are_single_fields_and_need_both_ends() {
        let mut req = request();
        req.set("vdiff", "-1250m; 1.25");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(resp.fields.iter().any(|f| f.key == "vdiff") && resp.fields.iter().all(|f| f.key != "vdiff_min"));
        req.set("vo", "0");
        assert!(matches!(solve(&req).outcome, Outcome::Error(_)));
        req.set("vo", "-2.5; x");
        assert!(matches!(solve(&req).outcome, Outcome::FixErrors(_)));
    }

    #[test]
    fn any_two_of_vindiff_vout_gain_give_the_third() {
        let computed = |resp: &DifferentialResponse, key: &str| {
            resp.fields.iter().find(|f| f.key == key).unwrap().info.computed.clone()
        };
        let mut req = request();
        req.set("vo", "");
        req.set("gain", "2");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(computed(&resp, "vo").is_some() && computed(&resp, "gain").is_none());
        let mut req = request();
        req.set("vdiff", "");
        req.set("gain", "2");
        let resp = solve(&req);
        assert!(matches!(resp.outcome, Outcome::Solved), "{:?}", resp.outcome);
        assert!(computed(&resp, "vdiff").is_some());
        // Both ranges given: G is computed.
        assert!(computed(&solve(&request()), "gain").is_some());
        // One of the three is not enough.
        req.set("gain", "");
        assert!(matches!(solve(&req).outcome, Outcome::Pending(_)));
    }
}

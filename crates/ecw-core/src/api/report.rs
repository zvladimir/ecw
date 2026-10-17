//! Pieces shared by the design pages (level shifters, difference amplifier): the fields, the
//! results table (Nom, Min, Max, Δ), the worst case over the corners of the tolerances and the
//! input/output graph.

use std::collections::HashMap;

use uom::si::f64::ElectricPotential;

use super::opamp::{format_delta, split_range};
use super::{FieldInfo, Form, GroupedField, PlotData, PlotLevel, PlotLine};
use crate::domain::format::format_engineering;
use crate::domain::interval::Interval;
use crate::domain::measure::Measure;
use crate::domain::unit::Measured;
use crate::domain::{FieldState, Quantity, Text};
use crate::msg;

const PLOT_POINTS: usize = 41;
/// Half-width of the axes of a graph that has nothing to draw yet.
pub(crate) const PLOT_DEFAULT_RANGE: f64 = 10.0;

/// How the text of a field is understood.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Input {
    /// A number with an SI prefix and no tolerance: `3.3`, `100m`.
    Exact,
    /// A number with an optional tolerance: `10k 1%`.
    Tolerant,
}

pub(crate) struct Spec {
    pub key: &'static str,
    pub label: &'static str,
    pub hint: &'static str,
    pub group: &'static str,
    pub input: Input,
    pub quantity: Quantity,
    pub unit: &'static str,
}

impl Spec {
    pub(crate) fn parse(&self, text: &str) -> FieldState {
        match self.input {
            Input::Exact => FieldState::parse_exact(text, self.quantity),
            Input::Tolerant => FieldState::parse(text, self.quantity),
        }
    }
}

pub(crate) const fn spec(
    key: &'static str,
    label: &'static str,
    hint: &'static str,
    group: &'static str,
    quantity: Quantity,
    unit: &'static str,
) -> Spec {
    Spec {
        key,
        label,
        hint,
        group,
        input: Input::Exact,
        quantity,
        unit,
    }
}

/// A field with its own way of reading the text.
pub(crate) const fn spec_in(
    key: &'static str,
    label: &'static str,
    hint: &'static str,
    group: &'static str,
    input: Input,
    quantity: Quantity,
    unit: &'static str,
) -> Spec {
    Spec {
        key,
        label,
        hint,
        group,
        input,
        quantity,
        unit,
    }
}

/// A field label: plain text, or `t:id` for a message.
pub(crate) fn label_text(label: &str) -> Text {
    match label.strip_prefix("t:") {
        Some(id) => msg!(id.to_owned()),
        None => label.into(),
    }
}

/// Parses the fields of a form of exact numbers and describes them for the UI.
pub(crate) fn parse_fields<'a>(
    specs: &[Spec],
    text: impl Fn(&str) -> &'a str,
    decimals: u32,
) -> (Vec<GroupedField>, HashMap<&'static str, FieldState>) {
    let states: HashMap<&'static str, FieldState> = specs
        .iter()
        .map(|s| (s.key, s.parse(text(s.key))))
        .collect();
    let fields = specs
        .iter()
        .map(|s| GroupedField {
            key: s.key,
            label: label_text(s.label),
            hint: s.hint.into(),
            group: s.group,
            info: FieldInfo::from_state(&states[s.key], s.unit, decimals, true),
        })
        .collect();
    (fields, states)
}

/// Empty axes for the graph tab before there is anything to draw: `x_name` and `y_name` are the
/// quantities (`Vin`, `Vout`), both in volts.
pub(crate) fn empty_plot(x_name: &str, y_name: &str) -> PlotData {
    PlotData {
        x: [-PLOT_DEFAULT_RANGE, PLOT_DEFAULT_RANGE],
        y: [-PLOT_DEFAULT_RANGE, PLOT_DEFAULT_RANGE],
        x_label: format!("{x_name}, V"),
        y_label: format!("{y_name}, V"),
        x_sym: x_name.to_owned(),
        x_unit: "V".to_owned(),
        y_unit: "V".to_owned(),
        ..Default::default()
    }
}

/// Output against input: the required line and the one with the given resistors, with the
/// supply rails as dashed lines when they are known.
pub(crate) fn transfer_plot(
    x_name: &str,
    (vi_min, vi_max): (f64, f64),
    (vo_min, vo_max): (f64, f64),
    rails: Option<(f64, f64)>,
    ideal: &dyn Fn(f64) -> f64,
    real: &dyn Fn(f64) -> f64,
) -> PlotData {
    let pad = 0.1 * (vi_max - vi_min);
    let (x0, x1) = (vi_min - pad, vi_max + pad);
    let xs: Vec<f64> = (0..PLOT_POINTS)
        .map(|k| x0 + (x1 - x0) * k as f64 / (PLOT_POINTS - 1) as f64)
        .collect();
    let line = |kind, name: Text, f: &dyn Fn(f64) -> f64, width| PlotLine {
        kind,
        name,
        points: xs.iter().map(|&x| [x, f(x)]).collect(),
        width,
    };
    let lines = vec![
        line("nominal", msg!("report.legend.required"), ideal, 2.4),
        line("max", msg!("report.legend.with_resistors"), real, 1.4),
    ];
    let (y_lo, y_hi) = match rails {
        Some((vcc, vee)) => (vee.min(vo_min), vcc.max(vo_max)),
        None => lines
            .iter()
            .flat_map(|l| l.points.iter().map(|p| p[1]))
            .fold((vo_min, vo_max), |(lo, hi), y| (lo.min(y), hi.max(y))),
    };
    let ypad = 0.08 * (y_hi - y_lo);
    PlotData {
        x: [x0, x1],
        y: [y_lo - ypad, y_hi + ypad],
        x_label: format!("{x_name}, V"),
        y_label: "Vo, V".to_owned(),
        x_sym: x_name.to_owned(),
        x_unit: "V".to_owned(),
        y_unit: "V".to_owned(),
        lines,
        guides: rails.map_or_else(Vec::new, |(vcc, vee)| {
            vec![[x0, vcc, x1, vcc], [x0, vee, x1, vee]]
        }),
        ..Default::default()
    }
}

/// A signal range in one field, `min; max`; one number is the range from 0 to it. Returns the
/// field and the states of its two ends.
pub(crate) fn range_field(
    spec: &Spec,
    text: &str,
    decimals: u32,
) -> (GroupedField, [FieldState; 2]) {
    let ends = if text.contains(';') {
        let (min, max) = split_range(text);
        [
            FieldState::parse_exact(min, spec.quantity),
            FieldState::parse_exact(max, spec.quantity),
        ]
    } else {
        match FieldState::parse_exact(text, spec.quantity) {
            FieldState::Ok(m) => {
                let (a, b) = (0.0_f64.min(m.nominal), 0.0_f64.max(m.nominal));
                [
                    FieldState::Ok(Measure::exact(a)),
                    FieldState::Ok(Measure::exact(b)),
                ]
            }
            other => [other.clone(), other],
        }
    };
    let shown = ends
        .iter()
        .find(|s| s.has_error())
        .or(ends.iter().find(|s| s.is_ok()))
        .unwrap_or(&ends[0]);
    let field = GroupedField {
        key: spec.key,
        label: label_text(spec.label),
        hint: spec.hint.into(),
        group: spec.group,
        info: FieldInfo::from_state(shown, spec.unit, decimals, false),
    };
    (field, ends)
}

/// A field already parsed by the caller, without the value echoed under it (the fields on a
/// schematic have no room for it).
pub(crate) fn quiet_field(spec: &Spec, state: &FieldState, decimals: u32) -> GroupedField {
    GroupedField {
        key: spec.key,
        label: label_text(spec.label),
        hint: spec.hint.into(),
        group: spec.group,
        info: FieldInfo::from_state(state, spec.unit, decimals, false),
    }
}

/// Both ends of a range as `min; max`, the text of a computed range field.
pub(crate) fn range_text<T: Measured>((lo, hi): (T, T), decimals: u32) -> String {
    format!(
        "{}; {}",
        format_engineering(lo.to_base(), T::UNIT, decimals),
        format_engineering(hi.to_base(), T::UNIT, decimals)
    )
}

/// The text of a computed single value: engineering notation, a ratio as a plain number.
pub(crate) fn value_text<T: Measured>(value: T, decimals: u32) -> String {
    if T::UNIT.is_empty() {
        trimmed(value.to_base(), "", decimals)
    } else {
        format_engineering(value.to_base(), T::UNIT, decimals)
    }
}

/// The header of a results table: the quantity, Nom, Min, Max, Δ.
pub(crate) fn results_header() -> Vec<Text> {
    vec![
        Text::default(),
        msg!("common.nom"),
        msg!("common.min"),
        msg!("common.max"),
        "Δ, %".into(),
    ]
}

/// A value in engineering notation; a ratio (no unit) as a plain number: `0.8125`, not `812.5 m`.
pub(crate) fn trimmed(value: f64, unit: &str, decimals: u32) -> String {
    let value = value + 0.0; // no "-0.0000"
    if unit.is_empty() {
        return format!("{value:.0$}", decimals as usize);
    }
    format_engineering(value, unit, decimals)
        .trim_end()
        .to_owned()
}

/// A parameter (a resistor, the gain): nominal, lowest and highest value; Min and Max stay empty
/// when there is no tolerance.
pub(crate) fn param_row<T: Measured>(
    label: impl Into<Text>,
    (n, lo, hi): (T, T, T),
    decimals: u32,
) -> Vec<Text> {
    let interval = Interval::new(n.to_base(), lo.to_base(), hi.to_base());
    let cell = |v: f64| Text::from(trimmed(v, T::UNIT, decimals));
    let (min, max) = if interval.is_exact() {
        (Text::from("—"), Text::from("—"))
    } else {
        (cell(interval.lo), cell(interval.hi))
    };
    vec![
        label.into(),
        cell(interval.nominal),
        min,
        max,
        format_delta(interval).into(),
    ]
}

/// A range: Nom is the mean of `nominal`, Min and Max the ends `lo` and `hi`.
pub(crate) fn range_row<T: Measured>(
    label: impl Into<Text>,
    nominal: (T, T),
    (lo, hi): (T, T),
    decimals: u32,
) -> Vec<Text> {
    let mean = (nominal.0.to_base() + nominal.1.to_base()) / 2.0;
    let cell = |v: f64| Text::from(trimmed(v, T::UNIT, decimals));
    vec![
        label.into(),
        cell(mean),
        cell(lo.to_base()),
        cell(hi.to_base()),
        Text::from("—"),
    ]
}

/// The lower and the higher of two values.
pub(crate) fn ordered<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if b < a { (b, a) } else { (a, b) }
}

/// The output range: ideal (`nominal`, the ends without tolerances) and, when there are
/// tolerances, the lowest and the highest value it can reach (`band`).
pub(crate) fn vout_rows<T: Measured>(
    labels: (Text, Text),
    nominal: (T, T),
    band: Option<(T, T)>,
    decimals: u32,
) -> Vec<Vec<Text>> {
    let mut rows = vec![range_row(
        labels.0,
        nominal,
        ordered(nominal.0, nominal.1),
        decimals,
    )];
    if let Some(band) = band {
        rows.push(range_row(labels.1, nominal, band, decimals));
    }
    rows
}

/// The end of a toleranced value that the corner `mask` takes: bit `bit` set is the high end.
pub(crate) fn corner<T: Copy>(mask: u32, bit: u32, (lo, hi): (T, T)) -> T {
    if mask >> bit & 1 == 1 { hi } else { lo }
}

/// The lowest and the highest of `f` over the `2^n` corners of `n` toleranced values.
pub(crate) fn extremes<T: Copy + PartialOrd>(n: u32, f: impl Fn(u32) -> T) -> (T, T) {
    let first = f(0);
    (1..1u32 << n).fold((first, first), |(lo, hi), mask| {
        let v = f(mask);
        (if v < lo { v } else { lo }, if v > hi { v } else { hi })
    })
}

/// Adds the level of `Vref` to a transfer graph and, when there are tolerances, the worst-case
/// edges of the output (`band(x)` is the lowest and the highest output at the input `x`).
/// No level is drawn for a reference the circuit does not need (`vref` is `None`).
pub(crate) fn add_vref_and_band(
    plot: &mut PlotData,
    vref: Option<f64>,
    band: Option<&dyn Fn(f64) -> (f64, f64)>,
) {
    if let Some(vref) = vref {
        plot.levels.push(PlotLevel {
            kind: "iavg",
            name: "Vref".into(),
            y: vref,
        });
        plot.y = [plot.y[0].min(vref), plot.y[1].max(vref)];
    }
    let Some(band) = band else { return };
    let xs: Vec<f64> = plot.lines[0].points.iter().map(|p| p[0]).collect();
    let edge = |name: Text, kind, pick: fn((f64, f64)) -> f64| PlotLine {
        kind,
        name,
        points: xs.iter().map(|&x| [x, pick(band(x))]).collect(),
        width: 1.4,
    };
    // The line with the given resistors keeps its own color; the band edges take the other two.
    plot.lines[1].kind = "accent";
    plot.lines
        .push(edge(msg!("opamp.legend.max"), "max", |b| b.1));
    plot.lines
        .push(edge(msg!("opamp.legend.min"), "min", |b| b.0));
    let ys = plot
        .lines
        .iter()
        .flat_map(|l| l.points.iter().map(|p| p[1]));
    let (lo, hi) = ys.fold((plot.y[0], plot.y[1]), |(lo, hi), y| (lo.min(y), hi.max(y)));
    let pad = 0.08 * (hi - lo);
    plot.y = [lo - pad, hi + pad];
}

/// Four significant digits, no trailing zeros (numbers inside messages).
pub(crate) fn sig(v: f64) -> String {
    if v == 0.0 || !v.is_finite() {
        return format!("{v}");
    }
    let decimals = (3 - v.abs().log10().floor() as i32).clamp(0, 12) as usize;
    let text = format!("{v:.decimals$}");
    if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        text
    }
}

/// Where the reference of a level shifter comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VrefSource {
    /// Typed by the user.
    Given,
    /// The field is empty and the given resistors fix it.
    Required,
    /// The field is empty: a standard value of the right sign and size.
    Recommended,
    /// The field is empty and the circuit needs no reference.
    NotNeeded,
}

/// The row of the reference in a level shifter's results table.
pub(crate) fn vref_row(
    source: VrefSource,
    vref: ElectricPotential,
    ends: (ElectricPotential, ElectricPotential),
    decimals: u32,
) -> Vec<Text> {
    let exact = (vref, vref, vref);
    match source {
        VrefSource::Given => param_row("Vref [V]", (vref, ends.0, ends.1), decimals),
        VrefSource::Required => param_row(msg!("level_shifter.row.vref_required"), exact, decimals),
        VrefSource::Recommended => {
            param_row(msg!("level_shifter.row.vref_recommended"), exact, decimals)
        }
        VrefSource::NotNeeded => {
            vec![
                msg!("level_shifter.row.vref_recommended"),
                msg!("level_shifter.vref_not_needed"),
                "—".into(),
                "—".into(),
                "—".into(),
            ]
        }
    }
}

/// The parsed fields of a design page (the amplifiers): plain fields, range fields (`min; max`)
/// and the gain, with typed access to their values. The fields it describes for the UI can be
/// marked computed or given a new hint.
pub(crate) struct DesignForm {
    pub fields: Vec<GroupedField>,
    states: HashMap<&'static str, FieldState>,
    ranges: HashMap<&'static str, [FieldState; 2]>,
}

impl DesignForm {
    /// `specs` are plain fields, `ranges` two-ended ones; `gain` is parsed by the caller (its
    /// sign rule depends on the circuit).
    pub(crate) fn parse(
        req: &Form,
        specs: &[Spec],
        ranges: &[Spec],
        gain: (&Spec, FieldState),
    ) -> Self {
        let d = req.decimals;
        let (mut fields, mut states) = parse_fields(specs, |key| req.text(key), d);
        let mut range_states = HashMap::new();
        for spec in ranges {
            let (field, ends) = range_field(spec, req.text(spec.key), d);
            fields.push(field);
            range_states.insert(spec.key, ends);
        }
        fields.push(quiet_field(gain.0, &gain.1, d));
        states.insert(gain.0.key, gain.1);
        Self {
            fields,
            states,
            ranges: range_states,
        }
    }

    pub(crate) fn has_error(&self) -> bool {
        self.states
            .values()
            .chain(self.ranges.values().flatten())
            .any(FieldState::has_error)
    }

    fn measure(&self, key: &str) -> Option<Measure> {
        self.states.get(key).and_then(FieldState::measure)
    }

    /// The nominal value of a filled field.
    pub(crate) fn value<T: Measured>(&self, key: &str) -> Option<T> {
        self.measure(key).map(|m| T::from_base(m.nominal))
    }

    /// Both ends of a filled range field.
    pub(crate) fn range<T: Measured>(&self, key: &str) -> Option<(T, T)> {
        let [lo, hi] = self.ranges.get(key)?;
        Some((
            T::from_base(lo.measure()?.nominal),
            T::from_base(hi.measure()?.nominal),
        ))
    }

    /// The ends of a field's tolerance, or `value` at both ends when the field is empty.
    pub(crate) fn ends<T: Measured>(&self, key: &str, value: T) -> (T, T) {
        self.measure(key).map_or((value, value), |m| {
            (T::from_base(m.lower_bound()), T::from_base(m.upper_bound()))
        })
    }

    /// The relative ends of a field's tolerance (`(0.99, 1.01)` for 1 %), `(1, 1)` without one.
    pub(crate) fn relative(&self, key: &str) -> (f64, f64) {
        self.measure(key)
            .filter(|m| m.nominal != 0.0)
            .map_or((1.0, 1.0), |m| {
                (m.lower_bound() / m.nominal, m.upper_bound() / m.nominal)
            })
    }

    /// Shows `text` in the field `key` as computed: the field is locked on the schematic.
    pub(crate) fn set_computed(&mut self, key: &str, text: String) {
        if let Some(f) = self.fields.iter_mut().find(|f| f.key == key) {
            f.info.computed = Some(text);
        }
    }

    pub(crate) fn set_hint(&mut self, key: &str, text: String) {
        if let Some(f) = self.fields.iter_mut().find(|f| f.key == key) {
            f.hint = text.into();
        }
    }
}

//! Current Shape page: the form for the chosen waveform, the table of RMS and average
//! current, and the plot of two periods of the current.

use std::f64::consts::PI;

use super::Form;
use crate::msg;
use serde::Serialize;
use uom::si::electric_current::ampere;
use uom::si::f64::{ElectricCurrent, Time};
use uom::si::time::second;

use super::{
    FieldInfo, FormField, Outcome, PlotData, PlotDim, PlotLevel, PlotLine, PlotMark, Table,
    form_field,
};
use crate::current_shape::calc::{self, Input, Report};
use crate::current_shape::times::{self, TIME_VARS, Times};
use crate::current_shape::{Var, WaveformKind, vars_for};
use crate::domain::format::format_engineering;
use crate::domain::interval::Interval;
use crate::domain::unit::Measured;
use crate::domain::{FieldSet, FieldState, Text};

const PLOT_SAMPLES: usize = 600;
const PLOT_CYCLES: f64 = 2.0;

/// A kind of waveform for the selector: its id and the name to show.
#[derive(Debug, Serialize)]
pub struct KindOption {
    /// The id of the kind in the request.
    pub id: &'static str,
    /// The name of the kind.
    pub name: Text,
}

/// The reply of the Current Shape page: the kinds for the selector, the fields of the chosen
/// kind, the outcome, the results and the plot.
#[derive(Debug, Serialize)]
pub struct ShapeResponse {
    /// The kinds for the selector.
    pub kinds: Vec<KindOption>,
    /// The input fields with their state: parse result, hint, and the computed value of a locked field.
    pub fields: Vec<FormField>,
    /// What to show in the results area.
    pub outcome: Outcome,
    /// RMS and average current and the duty cycle.
    pub table: Option<Table>,
    /// Two periods of the current.
    pub plot: PlotData,
}

/// Parses the form, solves it and lays out the page. A triangle's missing time is computed
/// from the other three and shown in its field.
pub fn solve(req: &Form) -> ShapeResponse {
    let d = req.decimals;
    let kind = WaveformKind::parse(req.text("kind"));
    let vars = vars_for(kind);

    let fields = FieldSet::collect(vars, |var| {
        FieldState::parse_exact(req.text(var.key()), var.quantity())
    });

    let mismatch = kind.is_triangular() && times::is_inconsistent(&fields);
    let solvable = !mismatch && can_solve(kind, vars, &fields);
    let result = solvable.then(|| calc::solve(build_input(kind, &fields)));
    let computed_time = (solvable && kind.is_triangular())
        .then(|| times::missing_var(&fields))
        .flatten();
    let resolved = times::resolve(&fields);

    let form = vars
        .iter()
        .map(|&var| {
            let info = if computed_time == Some(var) {
                let value = resolved.and_then(|t| t.get(var)).map_or(0.0, Measured::to_base);
                FieldInfo::from_state(fields.state(var), var.unit(), d, true)
                    .with_computed(Some(format_engineering(value, var.unit(), d)))
            } else {
                FieldInfo::from_state(fields.state(var), var.unit(), d, true)
            };
            form_field(var.key(), var.label(kind), var.hint(), info)
        })
        .collect();

    let report = result.as_ref().and_then(|r| r.as_ref().ok());
    let table = report.map(|r| key_value_table(r, d));

    let n = fields.valid_count();
    let pending = if n == 0 {
        msg!("current_shape.fill_required")
    } else {
        msg!(
            "current_shape.fill_required_partial",
            given = n,
            total = vars.len()
        )
    };

    ShapeResponse {
        kinds: WaveformKind::ALL
            .iter()
            .map(|&(k, id)| KindOption { id, name: k.name() })
            .collect(),
        fields: form,
        outcome: if mismatch {
            Outcome::Error(msg!("current_shape.times_mismatch"))
        } else {
            Outcome::new(result.as_ref(), fields.has_error(), pending)
        },
        table,
        plot: preview(kind, report, &fields),
    }
}

fn key_value_table(report: &Report, d: u32) -> Table {
    Table {
        header: Vec::new(),
        rows: vec![
            vec![
                "I_rms [A]".into(),
                format_engineering(report.i_rms.nominal.to_base(), "A", d).into(),
            ],
            vec![
                "I_avg [A]".into(),
                format_engineering(report.i_avg.nominal.to_base(), "A", d).into(),
            ],
            vec![
                msg!("current_shape.duty"),
                format!("{:.4}", report.duty.nominal.to_base()).into(),
            ],
        ],
    }
}

fn can_solve(kind: WaveformKind, vars: &[Var], fields: &FieldSet<Var>) -> bool {
    if fields.has_error() {
        return false;
    }

    if kind.is_triangular() {
        vars.iter()
            .filter(|var| !TIME_VARS.contains(var))
            .all(|&var| fields.is_ok(var))
            && times::filled_count(fields) >= TIME_VARS.len() - 1
    } else {
        vars.iter().all(|&var| fields.is_ok(var))
    }
}

fn zero<T: Measured>() -> Interval<T> {
    Interval::exact(T::from_base(0.0))
}

fn build_input(kind: WaveformKind, fields: &FieldSet<Var>) -> Input {
    let resolved = if kind.is_triangular() {
        times::resolve(fields)
    } else {
        None
    };
    let (t1, t2, period) = match resolved {
        Some(times) => (
            Some(Interval::exact(times.t1)),
            Some(Interval::exact(times.t2)),
            Interval::exact(times.period),
        ),
        None => (
            fields.interval(Var::T1),
            fields.interval(Var::T2),
            fields.interval(Var::Period).unwrap_or_else(zero),
        ),
    };

    Input {
        kind,
        ip: fields
            .interval(Var::Ip)
            .unwrap_or_else(zero::<ElectricCurrent>),
        ip2: fields.interval(Var::Ip2),
        imin: fields.interval(Var::Imin),
        ton: fields.interval(Var::Ton),
        t1,
        t2,
        period,
    }
}

// ---------------------------------------------------------------- preview

#[derive(Debug, Clone, Copy)]
struct WaveParams {
    ip: f64,
    ip2: f64,
    imin: f64,
    ton: f64,
    t1: f64,
    t2: f64,
    t3: f64,
    period: f64,
}

impl WaveParams {
    fn resolve(kind: WaveformKind, fields: &FieldSet<Var>) -> Self {
        // The drawing works in seconds: the edge where the types end.
        let seconds = |var: Var, times: Option<Times>| {
            times.and_then(|t| t.get(var)).map(Measured::to_base)
        };
        let resolved = kind.is_triangular().then(|| times::resolve(fields)).flatten();
        let time = |var: Var| {
            if kind.is_triangular() {
                seconds(var, resolved).unwrap_or_else(|| default_value(kind, var))
            } else {
                value_or_default(fields, kind, var)
            }
        };
        let times = (time(Var::T1), time(Var::T2), time(Var::T3), time(Var::Period));

        Self {
            ip: value_or_default(fields, kind, Var::Ip),
            ip2: value_or_default(fields, kind, Var::Ip2),
            imin: value_or_default(fields, kind, Var::Imin),
            ton: value_or_default(fields, kind, Var::Ton),
            t1: times.0,
            t2: times.1,
            t3: times.2,
            period: times.3,
        }
    }

    fn solver_input(&self, kind: WaveformKind) -> Input {
        let current = |v: f64| Interval::exact(ElectricCurrent::new::<ampere>(v));
        let time = |v: f64| Interval::exact(Time::new::<second>(v));

        Input {
            kind,
            ip: current(self.ip),
            ip2: Some(current(self.ip2)),
            imin: Some(current(self.imin)),
            ton: Some(time(self.ton)),
            t1: Some(time(self.t1)),
            t2: Some(time(self.t2)),
            period: time(self.period),
        }
    }
}

struct TimeArrow {
    start: f64,
    end: f64,
    label: &'static str,
    tier: usize,
}

struct AmpMarker {
    x: f64,
    level: f64,
    label: &'static str,
}

/// i(t) over two periods with Irms / Iavg levels and dimension annotations. Fields that are
/// still empty fall back to sample values so the shape is always visible.
fn preview(kind: WaveformKind, solved: Option<&Report>, fields: &FieldSet<Var>) -> PlotData {
    let params = WaveParams::resolve(kind, fields);
    let period = params.period;
    let span_x = PLOT_CYCLES * period;

    let fallback = solved
        .is_none()
        .then(|| calc::solve(params.solver_input(kind)).ok())
        .flatten();
    let report = solved.or(fallback.as_ref());

    let irms = report.map_or(params.ip, |r| r.i_rms.nominal.to_base());
    let iavg = report.map_or(0.0, |r| r.i_avg.nominal.to_base());
    // Only the levels the waveform has: the other fields keep sample values that would stretch the axis.
    let vars = vars_for(kind);
    let imin = if vars.contains(&Var::Imin) { params.imin } else { 0.0 };
    let ip2 = if vars.contains(&Var::Ip2) { params.ip2 } else { 0.0 };
    let y_min_data = imin.min(iavg).min(0.0);
    let y_max_data = params.ip.max(ip2).max(irms).max(iavg).max(0.0);
    let span = (y_max_data - y_min_data).max(1e-9);
    let y_lo = y_min_data - 0.55 * span;
    let y_hi = y_max_data + 0.15 * span;

    let ann_top = y_min_data - 0.05 * span;
    let ann_bottom = y_lo + 0.05 * span;
    let ann_span = ann_top - ann_bottom;
    let tier_y = |tier: usize| {
        let fraction = if tier == 0 { 0.72 } else { 0.24 };
        ann_bottom + fraction * ann_span
    };

    let mut plot = PlotData {
        x: [0.0, span_x],
        y: [y_lo, y_hi],
        x_label: "t, s".into(),
        y_label: "i, A".into(),
        x_sym: "t".into(),
        x_unit: "s".into(),
        y_unit: "A".into(),
        ..Default::default()
    };

    plot.lines.push(PlotLine {
        kind: "accent",
        name: "i(t)".into(),
        points: waveform_points(kind, params),
        width: 1.8,
    });

    if let Some(report) = report {
        plot.levels.push(PlotLevel {
            kind: "irms",
            name: "Irms".into(),
            y: report.i_rms.nominal.to_base(),
        });
        plot.levels.push(PlotLevel {
            kind: "iavg",
            name: "Iavg".into(),
            y: report.i_avg.nominal.to_base(),
        });
    }

    for x in boundary_marks(kind, &params) {
        for x_abs in [x, x + period] {
            if (0.0..=span_x).contains(&x_abs) {
                plot.guides.push([x_abs, y_min_data, x_abs, ann_bottom]);
            }
        }
    }

    let peaks = match kind {
        WaveformKind::Trapezoidal => vec![params.ip, params.ip2],
        WaveformKind::BipolarTriangular => vec![params.ip, params.imin],
        _ => vec![params.ip],
    };
    for level in peaks.into_iter().filter(|&l| l != 0.0) {
        plot.guides.push([0.0, level, span_x, level]);
    }

    for arrow in time_arrows(kind, &params) {
        if arrow.end > arrow.start {
            plot.dims.push(PlotDim {
                x0: arrow.start,
                x1: arrow.end,
                y: tier_y(arrow.tier),
                label: arrow.label,
            });
        }
    }
    for marker in amp_markers(kind, &params) {
        if marker.level != 0.0 {
            plot.marks.push(PlotMark {
                x: marker.x,
                y0: 0.0,
                y1: marker.level,
                label: marker.label,
            });
        }
    }
    plot
}

fn waveform_points(kind: WaveformKind, params: WaveParams) -> Vec<[f64; 2]> {
    let total = params.period * PLOT_CYCLES;
    let n = (PLOT_SAMPLES as f64 * PLOT_CYCLES) as usize;

    (0..=n)
        .map(|i| {
            let t_global = total * (i as f64) / (n as f64);
            let t = t_global.rem_euclid(params.period);
            [t_global, sample(kind, &params, t)]
        })
        .collect()
}

fn sample(kind: WaveformKind, p: &WaveParams, t: f64) -> f64 {
    use WaveformKind::*;

    let in_pulse = (0.0..=p.ton).contains(&t) && p.ton > 0.0;
    match kind {
        HalfSine if in_pulse => p.ip * (PI * t / p.ton).sin(),
        Rectangular if in_pulse => p.ip,
        Trapezoidal if in_pulse => p.ip + (p.ip2 - p.ip) * t / p.ton,
        HalfSine | Rectangular | Trapezoidal => 0.0,
        Triangular | TriangularOffset => {
            if t < 0.0 || p.t1 <= 0.0 || p.t2 <= 0.0 {
                return 0.0;
            }
            let floor = if kind == Triangular { 0.0 } else { p.imin };
            let swing = p.ip - floor;
            if t < p.t1 {
                floor + swing * t / p.t1
            } else if t < p.t1 + p.t2 {
                floor + swing * (p.t1 + p.t2 - t) / p.t2
            } else {
                0.0
            }
        }
        BipolarTriangular => {
            if t < 0.0 {
                0.0
            } else if t < p.t1 {
                p.imin * tent(t, p.t1)
            } else if t < p.t1 + p.t2 {
                p.ip * tent(t - p.t1, p.t2)
            } else {
                0.0
            }
        }
    }
}

fn tent(t: f64, width: f64) -> f64 {
    if t <= width / 2.0 {
        2.0 * t / width
    } else {
        2.0 * (width - t) / width
    }
}

fn time_arrows(kind: WaveformKind, p: &WaveParams) -> Vec<TimeArrow> {
    let period = TimeArrow {
        start: 0.0,
        end: p.period,
        label: "T",
        tier: 1,
    };

    if kind.is_triangular() {
        let segments = [
            ("t1", 0.0, p.t1),
            ("t2", p.t1, p.t2),
            ("t3", p.t1 + p.t2, p.t3),
        ];
        segments
            .into_iter()
            .map(|(label, start, length)| TimeArrow {
                start,
                end: start + length,
                label,
                tier: 0,
            })
            .chain([period])
            .collect()
    } else {
        vec![
            TimeArrow {
                start: 0.0,
                end: p.ton,
                label: "ton",
                tier: 0,
            },
            period,
        ]
    }
}

fn amp_markers(kind: WaveformKind, p: &WaveParams) -> Vec<AmpMarker> {
    use WaveformKind::*;

    let marker = |x: f64, level: f64, label: &'static str| AmpMarker { x, level, label };

    match kind {
        HalfSine | Rectangular => vec![marker(p.ton * 0.5, p.ip, "Ip")],
        Trapezoidal => vec![
            marker(p.ton * 0.15, p.ip, "Ip1"),
            marker(p.ton * 0.85, p.ip2, "Ip2"),
        ],
        Triangular => vec![marker(p.t1 + p.t2 * 0.5, p.ip, "Imax")],
        TriangularOffset => vec![
            marker(p.t1 * 0.15, p.imin, "Imin"),
            marker(p.t1, p.ip, "Imax"),
        ],
        BipolarTriangular => vec![
            marker(p.t1 * 0.5, p.imin, "Imin"),
            marker(p.t1 + p.t2 * 0.5, p.ip, "Imax"),
        ],
    }
}

fn boundary_marks(kind: WaveformKind, p: &WaveParams) -> Vec<f64> {
    if kind.is_triangular() {
        vec![0.0, p.t1, p.t1 + p.t2, p.t1 + p.t2 + p.t3, p.period]
    } else {
        vec![0.0, p.ton, p.period]
    }
}

fn default_value(kind: WaveformKind, var: Var) -> f64 {
    use WaveformKind::*;

    match (kind, var) {
        (TriangularOffset, Var::Imin) => 1.0,
        (TriangularOffset, Var::Ip) => 3.0,
        (BipolarTriangular, Var::Imin) => -1.0,
        (_, Var::Ip) => 1.0,
        (_, Var::Ip2) => 0.5,
        (_, Var::Imin) => 1.0,
        (_, Var::Ton) => 0.5,
        (_, Var::T1 | Var::T2) => 0.25,
        (_, Var::T3) => 0.5,
        (_, Var::Period) => 1.0,
    }
}

fn is_reasonable(kind: WaveformKind, var: Var, value: f64) -> bool {
    use WaveformKind::*;

    match (kind, var) {
        (TriangularOffset, Var::Imin) => value > 0.0,
        (BipolarTriangular, Var::Imin) => value < 0.0,
        (_, Var::Imin) => value != 0.0,
        (_, Var::T3) => value >= 0.0,
        (_, Var::Period | Var::Ton | Var::T1 | Var::T2 | Var::Ip | Var::Ip2) => value > 0.0,
    }
}

fn value_or_default(fields: &FieldSet<Var>, kind: WaveformKind, var: Var) -> f64 {
    fields
        .measure(var)
        .map(|m| m.nominal)
        .filter(|&value| is_reasonable(kind, var, value))
        .unwrap_or_else(|| default_value(kind, var))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(kind: &str) -> Form {
        Form::new(4, &[("kind", kind)])
    }

    #[test]
    fn rectangular_pulse_solves() {
        let mut r = req("rectangular");
        r.set("ip", "2");
        r.set("ton", "1m");
        r.set("period", "4m");
        let resp = solve(&r);
        assert!(matches!(resp.outcome, Outcome::Solved));
        assert_eq!(resp.table.unwrap().rows.len(), 3);
        assert_eq!(resp.plot.levels.len(), 2);
    }

    #[test]
    fn empty_form_still_previews_a_shape() {
        let resp = solve(&req("half_sine"));
        assert!(matches!(resp.outcome, Outcome::Pending(_)));
        assert!(!resp.plot.lines[0].points.is_empty());
    }

    #[test]
    fn triangular_derives_the_missing_time() {
        let mut r = req("triangular");
        r.set("ip", "1");
        r.set("t1", "1m");
        r.set("t2", "1m");
        r.set("t3", "2m");
        let resp = solve(&r);
        assert!(matches!(resp.outcome, Outcome::Solved));
        let period = resp.fields.iter().find(|f| f.key == "period").unwrap();
        assert!(period.info.computed.is_some());
    }

    #[test]
    fn a_rectangular_pulse_fills_the_plot_whatever_the_unused_levels() {
        let mut r = req("rectangular");
        r.set("ip", "10m");
        let resp = solve(&r);
        assert!(resp.plot.y[1] < 0.05, "y axis stretched to {:?}", resp.plot.y);
    }

    #[test]
    fn four_times_that_do_not_add_up_are_an_error() {
        let mut r = req("triangular");
        r.set("ip", "1");
        r.set("t1", "1m");
        r.set("t2", "1m");
        r.set("t3", "5m");
        r.set("period", "4m");
        assert!(matches!(solve(&r).outcome, Outcome::Error(_)));
        r.set("t3", "2m");
        assert!(matches!(solve(&r).outcome, Outcome::Solved));
    }
}

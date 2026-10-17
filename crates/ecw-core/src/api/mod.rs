//! Calculator entry points for the UI layer: raw field text in, display-ready values out.

use serde::Serialize;

use crate::domain::error::SolveError;
use crate::domain::format::{Breakdown, format_engineering};
use crate::domain::{FieldState, Text};
use crate::msg;

mod report;

/// A request of a calculator: the text of every input field by key, plus the number of
/// decimals. Selectors (`mode`, `series`, ...) are texts too. Missing keys read as empty.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Form {
    /// The number of digits after the point in the results (a setting of the UI).
    pub decimals: u32,
    #[serde(flatten)]
    texts: std::collections::HashMap<String, String>,
}

impl Form {
    /// A request from `(key, text)` pairs.
    pub fn new(decimals: u32, pairs: &[(&str, &str)]) -> Self {
        let mut form = Self {
            decimals,
            texts: Default::default(),
        };
        for (key, text) in pairs {
            form.set(key, text);
        }
        form
    }

    /// The text of a field; a field the request does not have reads as empty.
    pub fn text(&self, key: &str) -> &str {
        self.texts.get(key).map_or("", String::as_str)
    }

    /// Sets the text of a field.
    pub fn set(&mut self, key: &str, text: &str) {
        self.texts.insert(key.to_owned(), text.to_owned());
    }
}

type Handler = fn(serde_json::Value) -> Result<serde_json::Value, String>;

/// Wraps a calculator's `solve` function: JSON request in, JSON response out.
fn handler<Req, Resp>(
    request: serde_json::Value,
    solve: fn(&Req) -> Resp,
) -> Result<serde_json::Value, String>
where
    Req: serde::de::DeserializeOwned,
    Resp: Serialize,
{
    let request: Req = serde_json::from_value(request).map_err(|e| e.to_string())?;
    serde_json::to_value(solve(&request)).map_err(|e| e.to_string())
}

/// Every calculator: the name of its module in `api/` is the id the UI asks for, and the
/// module has `solve(&Form) -> Response`. A new calculator is one name here.
macro_rules! calculators {
    ($($name:ident),+ $(,)?) => {
        $(pub mod $name;)+

        const CALCULATORS: &[(&str, Handler)] = &[$((stringify!($name), |r| handler(r, $name::solve)),)+];
    };
}

calculators!(
    wire_awg,
    opamp,
    ohms_law,
    voltage_divider,
    resistor_solver,
    current_shape,
    level_shifter,
    level_shifter_ni,
    opamp_differential,
);

/// The ids of all calculators.
pub fn ids() -> impl Iterator<Item = &'static str> {
    CALCULATORS.iter().map(|(id, _)| *id)
}

/// Runs the calculator `id` on a JSON request and returns its JSON response.
pub fn solve(id: &str, request: serde_json::Value) -> Result<serde_json::Value, String> {
    let (_, run) = CALCULATORS
        .iter()
        .find(|(name, _)| *name == id)
        .ok_or_else(|| format!("unknown calculator: {id}"))?;
    run(request)
}

/// Parse feedback for one input field.
#[derive(Debug, Clone, Serialize)]
pub struct FieldInfo {
    /// `"ok"`, `"empty"` or `"error"`.
    pub status: &'static str,
    /// Parsed value (`"→ 1.0000 A"`) or the error message; empty when there is nothing to say.
    pub text: Text,
    /// Display text when the value is computed from the other fields.
    pub computed: Option<String>,
}

impl FieldInfo {
    pub(crate) fn from_state(
        state: &FieldState,
        unit: &str,
        decimals: u32,
        with_value: bool,
    ) -> Self {
        let (status, text) = match state {
            FieldState::Ok(m) if with_value => (
                "ok",
                format!("→ {}", format_engineering(m.nominal, unit, decimals)).into(),
            ),
            FieldState::Ok(_) => ("ok", Text::default()),
            FieldState::Err(e) => ("error", e.text()),
            FieldState::Empty => ("empty", Text::default()),
        };
        Self {
            status,
            text,
            computed: None,
        }
    }

    /// An empty field that the solver filled in; shows `computed` instead of an editor.
    pub(crate) fn with_computed(mut self, computed: Option<String>) -> Self {
        self.computed = computed;
        self
    }

    /// An input field that is locked when `blocked` (the other inputs already determine it).
    pub(crate) fn input(state: &FieldState, unit: &str, decimals: u32, blocked: bool) -> Self {
        if blocked {
            Self::blocked()
        } else {
            Self::from_state(state, unit, decimals, true)
        }
    }

    /// A field locked because the other inputs already determine the result.
    fn blocked() -> Self {
        Self {
            status: "empty",
            text: Text::default(),
            computed: Some(String::new()),
        }
    }
}

/// An input field of a dynamic form (the set of fields depends on a mode selector).
#[derive(Debug, Clone, Serialize)]
pub struct FormField {
    /// The key of the field in the request.
    pub key: &'static str,
    /// The caption of the field.
    pub label: Text,
    /// An example for the empty field.
    pub hint: Text,
    /// How the text of the field was read.
    #[serde(flatten)]
    pub info: FieldInfo,
}

/// An input field of a form that is split into labelled groups.
#[derive(Debug, Clone, Serialize)]
pub struct GroupedField {
    /// The key of the field in the request.
    pub key: &'static str,
    /// The caption of the field.
    pub label: Text,
    /// An example for the empty field.
    pub hint: Text,
    /// The group of the page the field belongs to (`signal`, `circuit`).
    pub group: &'static str,
    /// How the text of the field was read.
    #[serde(flatten)]
    pub info: FieldInfo,
}

/// One line of a report's list of checks.
#[derive(Debug, Serialize)]
pub struct Check {
    /// `"ok"`, `"fail"` or `"info"`.
    pub status: &'static str,
    /// What the line says.
    pub text: Text,
}

/// What to show in the results area.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", content = "text", rename_all = "snake_case")]
pub enum Outcome {
    /// The results are ready.
    Solved,
    /// The solver rejected the inputs.
    Error(Text),
    /// Some input field does not parse.
    FixErrors(Text),
    /// The form is not complete yet.
    Pending(Text),
    /// More inputs than the calculator takes.
    TooMany(Text),
}

impl Outcome {
    pub(crate) fn new<T>(
        result: Option<&Result<T, SolveError>>,
        has_input_error: bool,
        pending: Text,
    ) -> Self {
        match result {
            Some(Ok(_)) => Self::Solved,
            Some(Err(e)) => Self::Error(e.text()),
            None if has_input_error => Self::FixErrors(msg!("common.fix_errors")),
            None => Self::Pending(pending),
        }
    }
}

impl Outcome {
    /// For calculators that take exactly `needed` of their quantities: too few inputs get
    /// `pending(given)`, too many `TooMany`.
    pub(crate) fn by_count<T>(
        result: Option<&Result<T, SolveError>>,
        has_input_error: bool,
        given: usize,
        needed: usize,
        pending: impl FnOnce(usize) -> Text,
    ) -> Self {
        match Self::new(result, has_input_error, Text::default()) {
            Self::Pending(_) if given < needed => Self::Pending(pending(given)),
            Self::Pending(_) => Self::TooMany(msg!(
                "common.too_many_inputs",
                given = given,
                required = needed
            )),
            other => other,
        }
    }
}

/// A results table: a header row and the rows of cells.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Table {
    /// The captions of the columns.
    pub header: Vec<Text>,
    /// The cells of each row.
    pub rows: Vec<Vec<Text>>,
}

/// A row of the breakdown table: its message id and the cell it takes from a breakdown.
type Picker = (&'static str, fn(&Breakdown) -> &str);

/// Nominal / min / max / deviation rows for several quantities side by side.
pub(crate) fn breakdown_table(items: &[(&str, Breakdown)]) -> Table {
    let mut header = vec![Text::default()];
    header.extend(items.iter().map(|(name, _)| Text::from(*name)));

    let pickers: [Picker; 7] = [
        ("breakdown.nominal", |b| &b.nominal),
        ("breakdown.min", |b| &b.min),
        ("breakdown.max", |b| &b.max),
        ("breakdown.upper_percent", |b| &b.upper_percent),
        ("breakdown.upper_value", |b| &b.upper_absolute),
        ("breakdown.lower_percent", |b| &b.lower_percent),
        ("breakdown.lower_value", |b| &b.lower_absolute),
    ];
    let rows = pickers
        .iter()
        .map(|(key, pick)| {
            let mut row = vec![msg!(*key)];
            row.extend(items.iter().map(|(_, b)| Text::from(pick(b))));
            row
        })
        .collect();

    Table { header, rows }
}

// ---------------------------------------------------------------- plots

/// A curve of a plot: points `[x, y]` in data units, its colour key and line width.
#[derive(Debug, Serialize)]
pub struct PlotLine {
    /// Color key: `"nominal"`, `"max"`, `"min"`, `"accent"`.
    pub kind: &'static str,
    /// The name in the legend.
    pub name: Text,
    /// The points `[x, y]` in data units.
    pub points: Vec<[f64; 2]>,
    /// The line width, in pixels.
    pub width: f64,
}

/// Full-width horizontal line that appears in the legend.
#[derive(Debug, Serialize)]
pub struct PlotLevel {
    /// The colour key: `nominal`, `max`, `min`, `accent`.
    pub kind: &'static str,
    /// The name in the legend.
    pub name: Text,
    /// The height of the line, in data units.
    pub y: f64,
}

/// Time dimension arrow with a label.
#[derive(Debug, Serialize)]
pub struct PlotDim {
    /// Where the arrow starts on the x axis.
    pub x0: f64,
    /// Where the arrow ends on the x axis.
    pub x1: f64,
    /// The height of the arrow.
    pub y: f64,
    /// The label written next to the arrow.
    pub label: &'static str,
}

/// Amplitude marker: vertical arrow from `y0` to `y1` at `x`.
#[derive(Debug, Serialize)]
pub struct PlotMark {
    /// Where the arrow stands on the x axis.
    pub x: f64,
    /// The bottom of the arrow.
    pub y0: f64,
    /// The tip of the arrow.
    pub y1: f64,
    /// The label written next to the arrow.
    pub label: &'static str,
}

/// Everything the UI needs to draw a plot: the axes, the curves, the levels, the guides and
/// the dimension marks. The UI draws it as it is and knows no physics.
#[derive(Debug, Serialize, Default)]
pub struct PlotData {
    /// The range of the x axis, `[min, max]`.
    pub x: [f64; 2],
    /// The range of the y axis, `[min, max]`.
    pub y: [f64; 2],
    /// The caption of the x axis.
    pub x_label: String,
    /// The caption of the y axis.
    pub y_label: String,
    /// Name and unit of the cursor readout: `Vin = 1.0000 V`.
    pub x_sym: String,
    /// The unit of x in the cursor readout.
    pub x_unit: String,
    /// The unit of y in the cursor readout.
    pub y_unit: String,
    /// The curves.
    pub lines: Vec<PlotLine>,
    /// The horizontal levels.
    pub levels: Vec<PlotLevel>,
    /// Thin dashed segments `[x1, y1, x2, y2]`.
    pub guides: Vec<[f64; 4]>,
    /// The time dimension arrows.
    pub dims: Vec<PlotDim>,
    /// The amplitude markers.
    pub marks: Vec<PlotMark>,
}

pub(crate) fn form_field(
    key: &'static str,
    label: impl Into<Text>,
    hint: impl Into<Text>,
    info: FieldInfo,
) -> FormField {
    FormField {
        key,
        label: label.into(),
        hint: hint.into(),
        info,
    }
}

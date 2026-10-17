/// Categories of calculators (the groups of the sidebar).
pub mod category;
/// Parse and solve errors, rendered as messages.
pub mod error;
/// The E-series tables of preferred values (E6 … E96).
pub mod eseries;
/// Engineering notation (`4.70 kΩ`) and the nominal/min/max breakdown of a result.
pub mod format;
pub mod infer;
/// Intervals with a nominal value and arithmetic that carries the worst case.
pub mod interval;
/// A parsed input: nominal value with its tolerances.
pub mod measure;
/// Parsing of what the user types into a field (`4.7k 1%`).
pub mod parse;
/// Quantities of the input fields: symbol, unit, hint.
pub mod quantity;
pub mod series;
/// Small checks and helpers shared by the calculators.
pub mod solve;
pub mod text;
/// Units, SI prefixes and the link to the `uom` quantity types.
pub mod unit;
pub mod waveform;

pub use category::Category;
pub use eseries::ESeries;
pub use field_set::FieldSet;
pub use field_state::FieldState;
pub use interval::Interval;
pub use measure::Measure;
pub use quantity::Quantity;
pub use series::Series;
pub use text::Text;
pub use unit::Measured;
/// The states of all fields of one form, parsed together.
pub mod field_set;
/// The state of one input field: empty, a value, or an error.
pub mod field_state;

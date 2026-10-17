//! The calculation core of ECW (Engineers' Circuit Workbench): the electronics calculators
//! without any user interface.
//!
//! Every calculator is a module with its physics (`ohms_law`, `opamp`, `level_shifter`, …) and a
//! page in [`api`] that turns the text of the input fields into a ready-to-show reply. `domain`
//! holds what they share: values with tolerances, parsing and formatting, the solver that
//! derives unknown quantities from known ones, and the messages.
//!
//! The core knows no languages and no windows: the desktop app and the browser version both call
//! [`api::solve`] with JSON and show what comes back.

/// The reply of a calculator for the UI: the entry point and the shared pieces of the pages.
pub mod api;
/// The calculator of the current of a waveform (RMS, average, duty).
pub mod current_shape;
/// What every calculator uses: quantities, tolerances, parsing, formatting, errors.
pub mod domain;
pub mod level_shifter;
/// Ohm's law.
pub mod ohms_law;
/// Inverting and non-inverting amplifiers.
pub mod opamp;
pub mod opamp_differential;
/// Search for resistors close to a value.
pub mod resistor_solver;
/// Voltage divider of two or three resistors.
pub mod voltage_divider;
/// Wire AWG table: resistance, voltage drop and heating of copper wire.
pub mod wire_awg;

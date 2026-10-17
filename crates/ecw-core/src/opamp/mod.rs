//! Inverting and non-inverting amplifiers on an op-amp: from any sufficient set of the signal
//! ranges, the resistors and the gain the rest follows (TI Analog Engineer's Circuit Cookbook).

pub mod calc;

/// Amplifier configuration: the signal comes in on the inverting or non-inverting input.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Topology {
    /// G = -R2/R1, so G < 0.
    #[default]
    Inverting,
    /// G = 1 + R1/R2, so G > 1.
    NonInverting,
}

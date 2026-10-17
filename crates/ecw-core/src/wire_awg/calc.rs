//! The calculation of a wire table: electrical and thermal figures of every AWG size.

use std::f64::consts::PI;

use uom::si::electrical_resistivity::ohm_meter;
use uom::si::f64::{
    Area, ElectricCurrent, ElectricPotential, ElectricalResistance, ElectricalResistivity, Length,
    Power, TemperatureInterval, ThermalResistance,
};
use uom::si::length::millimeter;

use crate::domain::error::SolveError;
use crate::domain::interval::Interval;
use crate::domain::quantity::Quantity;
use crate::domain::solve::require_positive;

/// Resistivity of copper, Ω·m.
fn rho() -> ElectricalResistivity {
    ElectricalResistivity::new::<ohm_meter>(1.72e-8)
}

const AWG_TABLE: &[(&str, f64)] = &[
    ("40", 0.0799),
    ("39", 0.0897),
    ("38", 0.1007),
    ("37", 0.1131),
    ("36", 0.1270),
    ("35", 0.1426),
    ("34", 0.1601),
    ("33", 0.1798),
    ("32", 0.2019),
    ("31", 0.2268),
    ("30", 0.2547),
    ("29", 0.2859),
    ("28", 0.3211),
    ("27", 0.3606),
    ("26", 0.4049),
    ("25", 0.4547),
    ("24", 0.5106),
    ("23", 0.5733),
    ("22", 0.6438),
    ("21", 0.7229),
    ("20", 0.8118),
    ("19", 0.9116),
    ("18", 1.0237),
    ("17", 1.1495),
    ("16", 1.2908),
    ("15", 1.4495),
    ("14", 1.6277),
    ("13", 1.8278),
    ("12", 2.0525),
    ("11", 2.3048),
    ("10", 2.5882),
    ("9", 2.9064),
    ("8", 3.2639),
    ("7", 3.6649),
    ("6", 4.1154),
    ("5", 4.6213),
    ("4", 5.1894),
    ("3", 5.8273),
    ("2", 6.5437),
    ("1", 7.3481),
    ("0", 8.2525),
    ("00", 9.2658),
    ("000", 10.4049),
    ("0000", 11.6840),
];

/// The number of AWG sizes in the table.
pub const AWG_ROWS: usize = AWG_TABLE.len();

/// The request: the current, the length, the thermal resistance of the wire to the air, and the
/// number of parallel wires for each row of the table (a missing entry counts as one).
pub struct Input<'a> {
    /// The current.
    pub i: Interval<ElectricCurrent>,
    /// The length of the wire.
    pub l: Interval<Length>,
    /// The thermal resistance of the wire to the air.
    pub rth: Interval<ThermalResistance>,
    /// The number of parallel wires for each row, in the order of the table.
    pub n: &'a [f64],
}

/// One AWG size: the diameter `d`, the cross section `a`, the resistance of the length `r`,
/// the voltage drop at the current, the power loss `p` and the temperature rise `dt` (`p` times
/// the thermal resistance).
#[derive(Debug)]
pub struct Row {
    /// The AWG size.
    pub awg: &'static str,
    /// The diameter of one wire.
    pub d: Length,
    /// The cross section of one wire.
    pub a: Area,
    /// The resistance of the length (all parallel wires).
    pub r: Interval<ElectricalResistance>,
    /// The voltage drop at the current.
    pub vdrop: Interval<ElectricPotential>,
    /// The power loss.
    pub p: Interval<Power>,
    /// The temperature rise: `P · Rth`.
    pub dt: Interval<TemperatureInterval>,
}

/// Calculates every row of the table. The resistance uses the resistivity of copper at room
/// temperature (1.72·10⁻⁸ Ω·m); the table is the standard solid-wire diameters.
pub fn solve(input: Input<'_>) -> Result<Vec<Row>, SolveError> {
    require_positive(input.i, Quantity::Current)?;
    require_positive(input.l, Quantity::Length)?;
    require_positive(input.rth, Quantity::ThermalResistance)?;

    let rows = AWG_TABLE
        .iter()
        .enumerate()
        .map(|(idx, &(awg, diameter_mm))| {
            let d = Length::new::<millimeter>(diameter_mm);
            let a = cross_section(d);
            let strands = input.n.get(idx).copied().unwrap_or(1.0).max(1.0);

            let r = wire_resistance(input.l, a, strands);
            let vdrop: Interval<ElectricPotential> = input.i * r;
            let p: Interval<Power> = input.i * vdrop;
            let dt: Interval<TemperatureInterval> = p * input.rth;

            Row {
                awg,
                d,
                a,
                r,
                vdrop,
                p,
                dt,
            }
        })
        .collect();

    Ok(rows)
}

fn cross_section(d: Length) -> Area {
    let radius = d / 2.0;
    radius * PI * radius
}

/// `strands` is a count of parallel wires, not a quantity.
fn wire_resistance(l: Interval<Length>, a: Area, strands: f64) -> Interval<ElectricalResistance> {
    let area = a * strands;
    l.map(|l| rho() * l / area)
}

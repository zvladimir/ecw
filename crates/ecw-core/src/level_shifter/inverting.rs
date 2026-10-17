//! The quantities and rules of the inverting level shifter for `infer`:
//! `Vo = −A·Vi − B·Vref` with `A = R1/R2` and `B = R1/R3`.

use uom::si::f64::{ElectricPotential, ElectricalResistance, Ratio};
use uom::si::ratio::ratio;

use super::NEGLIGIBLE;
use crate::domain::infer::{Rule, Var, rule2, rule4, when2, when4};
use crate::domain::interval::Interval;

/// The lower end of the input range.
pub const VI_MIN: Var<ElectricPotential> = Var::new(0);
/// The upper end of the input range.
pub const VI_MAX: Var<ElectricPotential> = Var::new(1);
/// The lower end of the output range.
pub const VO_MIN: Var<ElectricPotential> = Var::new(2);
/// The upper end of the output range.
pub const VO_MAX: Var<ElectricPotential> = Var::new(3);
/// The reference voltage (any sign).
pub const VREF: Var<ElectricPotential> = Var::new(4);
/// The gain magnitude `R1/R2`.
pub const A: Var<Ratio> = Var::new(5);
/// The share of the reference `R1/R3`.
pub const B: Var<Ratio> = Var::new(6);
/// The feedback resistor.
pub const R1: Var<ElectricalResistance> = Var::new(7);
/// The input resistor.
pub const R2: Var<ElectricalResistance> = Var::new(8);
/// The reference resistor.
pub const R3: Var<ElectricalResistance> = Var::new(9);
/// The number of quantities.
pub const COUNT: usize = 10;

/// `B` large enough to need an `R3`.
fn needs_r3(b: Interval<Ratio>) -> bool {
    b.nominal.get::<ratio>() > NEGLIGIBLE
}

/// The ranges come first: they fix the design (`A` from the spans, `B` from the pair
/// `Vi_max ↔ Vo_min`), and the given resistors only fill what the ranges leave open. Then one
/// resistor gives the others, one range gives the other, and the resistors with the ranges give
/// the reference that they require.
pub fn rules() -> Vec<Rule> {
    vec![
        when4(A, VI_MIN, VI_MAX, VO_MIN, VO_MAX, |i0, i1, o0, o1| {
            (o1 - o0) / (i1 - i0)
        }),
        when4(B, VO_MIN, VI_MAX, A, VREF, |vo, vi, a, vref| {
            (vo + vi * a) / (-vref)
        }),
        when2(A, R1, R2, |r1, r2| r1 / r2),
        when2(B, R1, R3, |r1, r3| r1 / r3),
        rule2(R1, R2, A, |r2, a| Ok(a * r2)),
        when2(R1, R3, B, |r3, b| needs_r3(b).then(|| b * r3)),
        when2(R2, R1, A, |r1, a| r1 / a),
        when2(R3, R1, B, |r1, b| if needs_r3(b) { r1 / b } else { None }),
        rule4(VO_MAX, VI_MIN, A, B, VREF, |vi, a, b, vref| {
            Ok(-a * vi - b * vref)
        }),
        rule4(VO_MIN, VI_MAX, A, B, VREF, |vi, a, b, vref| {
            Ok(-a * vi - b * vref)
        }),
        when4(VI_MIN, VO_MAX, A, B, VREF, |vo, a, b, vref| {
            (-vo - b * vref) / a
        }),
        when4(VI_MAX, VO_MIN, A, B, VREF, |vo, a, b, vref| {
            (-vo - b * vref) / a
        }),
        when4(VREF, VO_MIN, VI_MAX, A, B, |vo, vi, a, b| {
            (-(vo + vi * a)) / b
        }),
    ]
}

//! The quantities and rules of the non-inverting level shifter for `infer`:
//! `Vo = G·Vi − M·Vref` with `M = R1/R2`, `K = R4/(R3 + R4)` and `G = K·(1 + M)`.

use uom::num_traits::Zero;
use uom::si::f64::{ElectricPotential, ElectricalResistance, Ratio};

use super::{exact_one, one};
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
/// The gain of the stage.
pub const G: Var<Ratio> = Var::new(5);
/// The feedback ratio `R1/R2`.
pub const M: Var<Ratio> = Var::new(6);
/// The input divider `R4/(R3 + R4)`.
pub const K: Var<Ratio> = Var::new(7);
/// The feedback resistor.
pub const R1: Var<ElectricalResistance> = Var::new(8);
/// The resistor to the reference.
pub const R2: Var<ElectricalResistance> = Var::new(9);
/// The upper resistor of the input divider.
pub const R3: Var<ElectricalResistance> = Var::new(10);
/// The lower resistor of the input divider.
pub const R4: Var<ElectricalResistance> = Var::new(11);
/// The number of quantities.
pub const COUNT: usize = 12;

/// A divider that both resistors can make: `0 < K < 1`.
fn proper(k: Interval<Ratio>) -> bool {
    k.nominal > Ratio::zero() && k.nominal < one()
}

/// The ranges come first: they fix the design (`G` from the spans, `M` from the pair
/// `Vi_min ↔ Vo_min`, `K` from both), and the given resistors only fill what the ranges leave
/// open. Then one resistor of each pair gives the other, one range gives the other, and the
/// resistors with the ranges give the reference that they require.
pub fn rules() -> Vec<Rule> {
    vec![
        when4(G, VI_MIN, VI_MAX, VO_MIN, VO_MAX, |i0, i1, o0, o1| {
            (o1 - o0) / (i1 - i0)
        }),
        when4(M, VI_MIN, VO_MIN, G, VREF, |vi, vo, g, vref| {
            (vi * g - vo) / vref
        }),
        when2(M, R1, R2, |r1, r2| r1 / r2),
        when2(K, G, M, |g, m| g / (exact_one() + m)),
        when2(K, R3, R4, |r3, r4| r4 / (r3 + r4)),
        rule2(G, K, M, |k, m| Ok(k * (exact_one() + m))),
        when2(R2, R1, M, |r1, m| r1 / m),
        rule2(R1, R2, M, |r2, m| Ok(m * r2)),
        when2(R3, R4, K, |r4, k| {
            if proper(k) {
                Some(r4 * ((exact_one() / k)? - exact_one()))
            } else {
                None
            }
        }),
        when2(R4, R3, K, |r3, k| {
            if proper(k) {
                r3 * k / (exact_one() - k)
            } else {
                None
            }
        }),
        rule4(VO_MIN, VI_MIN, G, M, VREF, |vi, g, m, vref| {
            Ok(g * vi - vref * m)
        }),
        rule4(VO_MAX, VI_MAX, G, M, VREF, |vi, g, m, vref| {
            Ok(g * vi - vref * m)
        }),
        when4(VI_MIN, VO_MIN, G, M, VREF, |vo, g, m, vref| {
            (vo + vref * m) / g
        }),
        when4(VI_MAX, VO_MAX, G, M, VREF, |vo, g, m, vref| {
            (vo + vref * m) / g
        }),
        when4(VREF, VI_MIN, VO_MIN, G, M, |vi, vo, g, m| (vi * g - vo) / m),
    ]
}

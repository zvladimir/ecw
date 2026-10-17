//! Difference amplifier (subtractor) on an op-amp, TI SBOA274A.
//!
//! `R1` from `Vi1` to the inverting input, `R3` the feedback, `R2` from `Vi2` to the
//! non-inverting input, `R4` from that input to `Vref`. With `R1 = R2` and `R3 = R4`:
//! `Vo = (Vi2 − Vi1)·G + Vref`, `G = R3/R1`.

use uom::si::f64::{ElectricPotential, ElectricalResistance, Ratio};
use uom::si::ratio::ratio;

use crate::domain::infer::{Rule, Var, rule2, rule3, when2, when3, when4};

/// The quantities of the design: the ranges of the difference and of the output, the
/// reference, the gain and the resistor pairs `R1 = R2`, `R3 = R4`.
pub const VD_MIN: Var<ElectricPotential> = Var::new(0);
/// The upper end of the range of the difference `Vi2 − Vi1`.
pub const VD_MAX: Var<ElectricPotential> = Var::new(1);
/// The lower end of the output range.
pub const VO_MIN: Var<ElectricPotential> = Var::new(2);
/// The upper end of the output range.
pub const VO_MAX: Var<ElectricPotential> = Var::new(3);
/// The reference voltage.
pub const VREF: Var<ElectricPotential> = Var::new(4);
/// The gain `R3/R1`.
pub const G: Var<Ratio> = Var::new(5);
/// The input resistors `R1 = R2`.
pub const R1: Var<ElectricalResistance> = Var::new(6);
/// The feedback resistors `R3 = R4`.
pub const R3: Var<ElectricalResistance> = Var::new(7);
/// The number of quantities.
pub const COUNT: usize = 8;

/// `Vo = Vd·G + Vref` and `G = R3/R1`: any two of Vd, Vo and G give the third, and the gain
/// with one resistor gives the other. The gain of the two ranges comes first, so a typed gain
/// or resistor pair does not override the ranges.
pub fn rules() -> Vec<Rule> {
    vec![
        when4(G, VD_MIN, VD_MAX, VO_MIN, VO_MAX, |d0, d1, o0, o1| {
            (o1 - o0) / (d1 - d0)
        }),
        when2(G, R1, R3, |r1, r3| r3 / r1),
        rule3(VO_MIN, VD_MIN, G, VREF, |vd, g, vref| Ok(vd * g + vref)),
        rule3(VO_MAX, VD_MAX, G, VREF, |vd, g, vref| Ok(vd * g + vref)),
        when3(VD_MIN, VO_MIN, G, VREF, |vo, g, vref| (vo - vref) / g),
        when3(VD_MAX, VO_MAX, G, VREF, |vo, g, vref| (vo - vref) / g),
        rule2(R3, R1, G, |r1, g| Ok(g * r1)),
        when2(R1, R3, G, |r3, g| r3 / g),
    ]
}

/// Gain from the spans of the difference and of the output.
pub fn gain(
    (vd_min, vd_max): (ElectricPotential, ElectricPotential),
    (vo_min, vo_max): (ElectricPotential, ElectricPotential),
) -> Ratio {
    (vo_max - vo_min) / (vd_max - vd_min)
}

/// The general transfer function.
pub fn output(
    vi1: ElectricPotential,
    vi2: ElectricPotential,
    vref: ElectricPotential,
    r1: ElectricalResistance,
    r2: ElectricalResistance,
    r3: ElectricalResistance,
    r4: ElectricalResistance,
) -> ElectricPotential {
    let noise_gain = Ratio::new::<ratio>(1.0) + r3 / r1;
    -vi1 * r3 / r1 + vi2 * r4 / (r2 + r4) * noise_gain + vref * r2 / (r2 + r4) * noise_gain
}

/// Voltage at the non-inverting input.
pub fn positive_input(
    vi2: ElectricPotential,
    vref: ElectricPotential,
    r2: ElectricalResistance,
    r4: ElectricalResistance,
) -> ElectricPotential {
    (vi2 * r4 + vref * r2) / (r2 + r4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::electric_potential::volt;
    use uom::si::electrical_resistance::ohm;

    fn v(x: f64) -> ElectricPotential {
        ElectricPotential::new::<volt>(x)
    }

    fn r(x: f64) -> ElectricalResistance {
        ElectricalResistance::new::<ohm>(x)
    }

    fn close(a: ElectricPotential, b: f64, tol: f64) {
        let a = a.get::<volt>();
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    #[test]
    fn gain_of_the_task() {
        assert!((gain((v(-1.25), v(1.25)), (v(-2.5), v(2.5))).get::<ratio>() - 2.0).abs() < 1e-12);
    }

    #[test]
    fn matched_resistors_give_a_plain_subtractor() {
        // R1 = R2, R3 = R4: Vo = (Vi2 - Vi1)·G + Vref.
        let vo = output(v(0.7), v(1.2), v(0.3), r(10e3), r(10e3), r(20e3), r(20e3));
        close(vo, (1.2 - 0.7) * 2.0 + 0.3, 1e-12);
        // The common-mode part cancels.
        close(
            output(v(1.0), v(1.0), v(0.0), r(10e3), r(10e3), r(20e3), r(20e3)),
            0.0,
            1e-12,
        );
    }

    #[test]
    fn mismatch_creates_a_common_mode_error() {
        let vo = output(
            v(1.0),
            v(1.0),
            v(0.0),
            r(10e3),
            r(10e3),
            r(20e3),
            r(20_040.0),
        );
        assert!(vo.get::<volt>().abs() > 1e-4);
    }

    #[test]
    fn positive_input_example() {
        close(
            positive_input(v(1.0), v(0.0), r(10e3), r(20e3)),
            2.0 / 3.0,
            1e-12,
        );
    }
}

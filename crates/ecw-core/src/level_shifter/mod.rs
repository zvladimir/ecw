//! Level shifters on an op-amp with a reference voltage: they map the input range `Vin` onto
//! the output range `Vout`.
//!
//! Inverting (TI SBOA261A): the input `Vi` goes through `R2` and the reference `Vref` through
//! `R3` to the inverting input, `R1` is the feedback, the non-inverting input is grounded:
//! `Vo = −Vi·R1/R2 − Vref·R1/R3`.
//!
//! Non-inverting (TI SBOA262A): the input goes through the divider `R3`/`R4` to the
//! non-inverting input, `R1` is the feedback, `R2` leads to `Vref`:
//! `Vo = Vi·R4/(R3+R4)·(R1+R2)/R2 − Vref·R1/R2`.

use uom::num_traits::Zero;
use uom::si::electric_potential::volt;
use uom::si::f64::{ElectricPotential, ElectricalResistance, Ratio};
use uom::si::ratio::ratio;

use crate::domain::interval::Interval;
use crate::domain::solve::ascending;

pub mod inverting;
pub mod non_inverting;

type Volts = ElectricPotential;
type Ohms = ElectricalResistance;
/// A signal range, its lower end first.
pub type Range = (Volts, Volts);

/// A ratio below this counts as zero (no offset, no `R3`).
const NEGLIGIBLE: f64 = 1e-12;

fn one() -> Ratio {
    Ratio::new::<ratio>(1.0)
}

fn exact_one() -> Interval<Ratio> {
    Interval::exact(one())
}

/// Common reference voltages (reference ICs and supply rails), in volts.
pub const STANDARD_REFERENCES: [f64; 9] = [1.024, 1.25, 2.048, 2.5, 3.0, 3.3, 4.096, 5.0, 10.0];

/// The standard reference nearest to `target` (on a log scale), below `limit` when there is
/// one; `target` itself when no standard value is below the limit.
pub fn standard_reference(target: Volts, limit: Option<Volts>) -> Volts {
    let distance = |v: Volts| (v / target).get::<ratio>().ln().abs();
    STANDARD_REFERENCES
        .iter()
        .map(|&v| Volts::new::<volt>(v))
        .filter(|v| limit.is_none_or(|l| *v < l))
        .min_by(|a, b| distance(*a).total_cmp(&distance(*b)))
        .unwrap_or(target)
}

/// The gain from the spans of the two ranges.
fn span_gain((vi_min, vi_max): Range, (vo_min, vo_max): Range) -> Ratio {
    (vo_max - vo_min) / (vi_max - vi_min)
}

/// The offset an inverting stage needs from the reference: `Vref·R1/R3 = −C` with
/// `C = Vo_min + G_in·Vi_max`. Positive `C` needs a negative `Vref`, and the other way round.
pub fn inverting_offset(vi: Range, vo: Range) -> Volts {
    vo.0 + span_gain(vi, vo) * vi.1
}

/// A reference for the inverting stage: the sign the offset needs and the standard value
/// nearest to `|C|`, so that `R3` comes out near `R1`. `None` when no offset is needed.
pub fn recommend_inverting_reference(vi: Range, vo: Range) -> Option<Volts> {
    let c = inverting_offset(vi, vo);
    (c.abs() > Volts::new::<volt>(NEGLIGIBLE))
        .then(|| -standard_reference(c.abs(), None) * c.value.signum())
}

/// A reference for the non-inverting stage: the sign of the offset `D = Vi_min·G_in − Vo_min`
/// and, when `G_in > 1`, below the limit `|D|/(G_in − 1)` that keeps `R3` positive; the
/// standard value nearest to half the limit (or to `|D|` without one, `R1 = R2`). `None` when
/// no offset is needed.
pub fn recommend_non_inverting_reference(vi: Range, vo: Range) -> Option<Volts> {
    let g_in = span_gain(vi, vo);
    let offset = vi.0 * g_in - vo.0;
    if offset.abs() < Volts::new::<volt>(NEGLIGIBLE) {
        return None;
    }
    let limit = (g_in > one()).then(|| offset.abs() / (g_in - one()));
    let target = limit.map_or(offset.abs(), |l| l / 2.0);
    Some(standard_reference(target, limit) * offset.value.signum())
}

/// The ratios of the ideal inverting design; the resistors follow from any one of them.
#[derive(Debug, Clone, Copy)]
pub struct Ideal {
    /// Gain magnitude from the spans, `|ΔVo/ΔVi| = R1/R2`.
    pub g_in: Ratio,
    /// `R1/R3`, always positive; zero when no reference is needed (no `R3`).
    pub r1_over_r3: Ratio,
}

/// What is wrong with the inverting design.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    /// `Vi_max ≤ Vi_min` or `Vo_max ≤ Vo_min`.
    EmptySpan,
    /// The reference is zero.
    NoReference,
    /// `R1/R3` came out negative: this sign of `Vref` cannot produce the offset.
    WrongReferenceSign { r1_over_r3: Ratio },
}

/// The gain from the spans, then `R1/R3` from the offset. An inverting stage maps `Vi_max` to
/// `Vo_min`, so the pair `Vi_max ↔ Vo_min` fixes the offset.
pub fn ideal(vi: Range, vo: Range, vref: Volts) -> Result<Ideal, Error> {
    if !ascending(vi) || !ascending(vo) {
        return Err(Error::EmptySpan);
    }
    if vref == Volts::zero() {
        return Err(Error::NoReference);
    }
    let g_in = span_gain(vi, vo);
    let r1_over_r3 = (vo.0 + vi.1 * g_in) / (-vref);
    if r1_over_r3.get::<ratio>() < -NEGLIGIBLE {
        return Err(Error::WrongReferenceSign { r1_over_r3 });
    }
    let r1_over_r3 = if r1_over_r3.get::<ratio>() > NEGLIGIBLE {
        r1_over_r3
    } else {
        Ratio::zero()
    };
    Ok(Ideal { g_in, r1_over_r3 })
}

/// `Vo = −Vi·R1/R2 − Vref·R1/R3` (no `R3` means no reference path).
pub fn output(vi: Volts, vref: Volts, r1: Ohms, r2: Ohms, r3: Option<Ohms>) -> Volts {
    -vi * r1 / r2 - r3.map_or(Volts::zero(), |r3| vref * r1 / r3)
}

/// The ratios of the ideal non-inverting design.
#[derive(Debug, Clone, Copy)]
pub struct NonInvertingDesign {
    /// Gain from the spans, `ΔVo/ΔVi`.
    pub g_in: Ratio,
    /// `Vi_min·G_in − Vo_min`, the offset that `Vref` has to supply (through `R1/R2`).
    pub offset: Volts,
    pub r1_over_r2: Ratio,
    /// The input divider `R4/(R3 + R4)`, below 1.
    pub divider: Ratio,
}

/// What is wrong with the non-inverting design.
#[derive(Debug, Clone, Copy)]
pub enum NonInvertingError {
    /// `Vi_max ≤ Vi_min` or `Vo_max ≤ Vo_min`.
    EmptySpan,
    /// The reference is zero.
    NoReference,
    /// The offset is zero: `R2` would be infinite.
    NoOffset,
    /// `R2` came out negative: the reference has the wrong sign for this offset.
    ReferenceSign { offset: Volts },
    /// `R3` would not be positive because `1 + R1/R2 ≤ G_in`. The ratio `R1/R2` depends only on
    /// the signal ranges and `Vref`; `vref_limit` is the largest `|Vref|` that works.
    R3NotPositive {
        r1_over_r2: Ratio,
        g_in: Ratio,
        vref_limit: Option<Volts>,
    },
}

/// The gain from the spans, `R1/R2` from the reference point `Vi_min ↔ Vo_min`, then the
/// divider from the gain.
pub fn ideal_non_inverting(
    vi: Range,
    vo: Range,
    vref: Volts,
) -> Result<NonInvertingDesign, NonInvertingError> {
    use NonInvertingError::*;
    if !ascending(vi) || !ascending(vo) {
        return Err(EmptySpan);
    }
    if vref == Volts::zero() {
        return Err(NoReference);
    }
    let g_in = span_gain(vi, vo);
    let offset = vi.0 * g_in - vo.0;
    if offset.abs() < Volts::new::<volt>(NEGLIGIBLE) {
        return Err(NoOffset);
    }
    let r1_over_r2 = offset / vref;
    if r1_over_r2 <= Ratio::zero() {
        return Err(ReferenceSign { offset });
    }
    let divider = g_in / (one() + r1_over_r2);
    if divider >= one() {
        let vref_limit = (g_in > one()).then(|| offset.abs() / (g_in - one()));
        return Err(R3NotPositive {
            r1_over_r2,
            g_in,
            vref_limit,
        });
    }
    Ok(NonInvertingDesign {
        g_in,
        offset,
        r1_over_r2,
        divider,
    })
}

/// `Vo = Vi·R4/(R3+R4)·(R1+R2)/R2 − Vref·R1/R2`.
pub fn output_non_inverting(
    vi: Volts,
    vref: Volts,
    r1: Ohms,
    r2: Ohms,
    r3: Ohms,
    r4: Ohms,
) -> Volts {
    vi * r4 / (r3 + r4) * (r1 + r2) / r2 - vref * r1 / r2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Series;
    use uom::si::electrical_resistance::ohm;

    fn v(x: f64) -> Volts {
        Volts::new::<volt>(x)
    }

    fn r(x: f64) -> Ohms {
        Ohms::new::<ohm>(x)
    }

    fn close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    fn task() -> (Range, Range) {
        ((v(-5.0), v(-1.0)), (v(0.05), v(3.3)))
    }

    fn ni_task() -> (Range, Range) {
        ((v(2.0), v(5.0)), (v(0.05), v(4.95)))
    }

    #[test]
    fn ideal_design_matches_the_task() {
        let (vi, vo) = task();
        let d = ideal(vi, vo, v(5.0)).unwrap();
        close(d.g_in.get::<ratio>(), 0.8125, 1e-12);
        close(d.r1_over_r3.get::<ratio>(), 0.1525, 1e-12);
        // With R1 = 845 Ω: R2 = R1/G_in, R3 = R1/(R1/R3).
        close((r(845.0) / d.g_in).get::<ohm>(), 1040.0, 1e-9);
        close((r(845.0) / d.r1_over_r3).get::<ohm>(), 5541.0, 1.0);
    }

    #[test]
    fn standard_values_give_the_task_outputs() {
        close(
            output(v(-1.0), v(5.0), r(845.0), r(1040.0), Some(r(5560.0))).get::<volt>(),
            0.0526,
            0.0001,
        );
        close(
            output(v(-5.0), v(5.0), r(845.0), r(1040.0), Some(r(5560.0))).get::<volt>(),
            3.3026,
            0.0001,
        );
    }

    #[test]
    fn wrong_reference_sign_is_reported() {
        // Vi −1…1, Vo 0…3.3: the offset needs a negative reference.
        let (vi, vo) = ((v(-1.0), v(1.0)), (v(0.0), v(3.3)));
        assert!(matches!(
            ideal(vi, vo, v(5.0)),
            Err(Error::WrongReferenceSign { .. })
        ));
        assert!(ideal(vi, vo, v(-5.0)).is_ok());
    }

    #[test]
    fn no_offset_needs_no_r3() {
        assert_eq!(
            ideal((v(-2.0), v(-1.0)), (v(2.0), v(4.0)), v(5.0))
                .unwrap()
                .r1_over_r3,
            Ratio::zero()
        );
    }

    #[test]
    fn non_inverting_design_matches_the_task() {
        let (vi, vo) = ni_task();
        let d = ideal_non_inverting(vi, vo, v(2.5)).unwrap();
        close(d.g_in.get::<ratio>(), 1.6333, 1e-4);
        // With R1 = R4 = 1 kΩ: R2 = R1/(R1/R2), R3 = R4·(1/divider − 1).
        let r2 = (r(1000.0) / d.r1_over_r2).get::<ohm>();
        let r3 = (r(1000.0) * (one() / d.divider - one())).get::<ohm>();
        close(r2, 777.2, 0.1);
        close(r3, 400.0, 0.5);
        close(Series::E192.nearest(r2), 777.0, 1e-9);
        close(Series::E192.nearest(r3), 402.0, 1e-9);
        let out = |x| {
            output_non_inverting(v(x), v(2.5), r(1000.0), r(777.0), r(402.0), r(1000.0))
                .get::<volt>()
        };
        close(out(2.0), 0.0449, 0.0002);
        close(out(5.0), 4.9387, 0.0002);
    }

    #[test]
    fn non_inverting_errors() {
        let (vi, vo) = ni_task();
        // A negative reference cannot give this positive offset.
        assert!(matches!(
            ideal_non_inverting(vi, vo, v(-2.5)),
            Err(NonInvertingError::ReferenceSign { .. })
        ));
        // A large Vref makes R1/R2 too small for the gain: R3 would not be positive.
        match ideal_non_inverting(vi, vo, v(10.0)) {
            Err(NonInvertingError::R3NotPositive {
                vref_limit: Some(limit),
                ..
            }) => {
                close(limit.get::<volt>(), 3.2167 / 0.6333, 0.01);
                assert!(ideal_non_inverting(vi, vo, limit * 0.99).is_ok());
            }
            other => panic!("{other:?}"),
        }
        assert!(matches!(
            ideal_non_inverting(vi, (v(1.98), v(4.95)), v(2.5)),
            Err(NonInvertingError::NoOffset)
        ));
    }

    #[test]
    fn recommended_references_work() {
        let at = |x: Option<Volts>| x.map(|v| v.get::<volt>());
        // Inverting: C = 0.05 − 0.8125 = −0.7625 needs a positive reference near 0.76 V.
        let (vi, vo) = task();
        assert_eq!(at(recommend_inverting_reference(vi, vo)), Some(1.024));
        // 0…1 V to 0…3 V: C = 3 needs a negative one.
        let up = ((v(0.0), v(1.0)), (v(0.0), v(3.0)));
        assert_eq!(at(recommend_inverting_reference(up.0, up.1)), Some(-3.0));
        assert_eq!(
            recommend_inverting_reference((v(-2.0), v(-1.0)), (v(2.0), v(4.0))),
            None
        );
        for (vi, vo) in [task(), up, ((v(-10.0), v(10.0)), (v(0.0), v(3.3)))] {
            let vref = recommend_inverting_reference(vi, vo).unwrap();
            assert!(ideal(vi, vo, vref).is_ok(), "{vi:?} {vo:?} {vref:?}");
        }
        // Non-inverting: the limit is 5.08 V, half of it gives the 2.5 V of the method.
        let (vi, vo) = ni_task();
        assert_eq!(at(recommend_non_inverting_reference(vi, vo)), Some(2.5));
        assert_eq!(recommend_non_inverting_reference(up.0, up.1), None);
        // The limit is 1.5 V: the value nearest to 0.75 V below it.
        assert_eq!(
            at(recommend_non_inverting_reference(
                (v(1.0), v(2.0)),
                (v(0.0), v(3.0))
            )),
            Some(1.024)
        );
        let cases = [
            ni_task(),
            ((v(1.0), v(2.0)), (v(0.0), v(3.0))),
            ((v(-1.0), v(1.0)), (v(0.0), v(3.3))),
            ((v(0.5), v(1.0)), (v(0.0), v(5.0))),
            ((v(0.1), v(1.0)), (v(0.0), v(5.0))),
        ];
        for (vi, vo) in cases {
            let vref = recommend_non_inverting_reference(vi, vo).unwrap();
            assert!(
                ideal_non_inverting(vi, vo, vref).is_ok(),
                "{vi:?} {vo:?} {vref:?}"
            );
        }
    }

    #[test]
    fn spans_must_be_positive() {
        let (_, vo) = task();
        assert!(matches!(
            ideal((v(-1.0), v(-5.0)), vo, v(5.0)),
            Err(Error::EmptySpan)
        ));
    }
}

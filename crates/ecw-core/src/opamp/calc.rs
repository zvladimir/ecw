//! The calculation of the amplifier: the signal ranges, the gain and the resistors, solved
//! together.

use std::ops::{Div, Mul};

use crate::msg;
use uom::num_traits::Zero;
use uom::si::electric_potential::volt;
use uom::si::f64::{ElectricPotential, ElectricalResistance, Ratio};
use uom::si::ratio::ratio;

use crate::domain::error::{Requirement, SolveError};
use crate::domain::interval::Interval;
use crate::domain::quantity::Quantity;

use super::Topology;

/// The given quantities of the amplifier, each `None` if left empty: the ends of the input and
/// the output ranges, the feedback resistors and the gain (negative for the inverting one).
#[derive(Debug, Clone, Copy, Default)]
pub struct Input {
    pub vin_min: Option<Interval<ElectricPotential>>,
    pub vin_max: Option<Interval<ElectricPotential>>,
    pub vout_min: Option<Interval<ElectricPotential>>,
    pub vout_max: Option<Interval<ElectricPotential>>,
    pub r1: Option<Interval<ElectricalResistance>>,
    pub r2: Option<Interval<ElectricalResistance>>,

    pub gain: Option<Interval<Ratio>>,
}

/// Which quantities of [`Solved`] were computed and not given, so the UI can mark them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Computed {
    pub vin_min: bool,
    pub vin_max: bool,
    pub vout_min: bool,
    pub vout_max: bool,
    pub r1: bool,
    pub r2: bool,
    pub gain: bool,
}

/// All quantities that could be found, with a flag for each that was computed.
#[derive(Debug, Clone, Copy, Default)]
pub struct Solved {
    pub vin_min: Option<Interval<ElectricPotential>>,
    pub vin_max: Option<Interval<ElectricPotential>>,
    pub vout_min: Option<Interval<ElectricPotential>>,
    pub vout_max: Option<Interval<ElectricPotential>>,
    pub r1: Option<Interval<ElectricalResistance>>,
    pub r2: Option<Interval<ElectricalResistance>>,
    pub gain: Option<Interval<Ratio>>,
    pub computed: Computed,
}

type Volts = Interval<ElectricPotential>;
type Ohms = Interval<ElectricalResistance>;
type Gain = Interval<Ratio>;

fn one() -> Ratio {
    Ratio::new::<ratio>(1.0)
}

/// One end of the signal range, in signed volts. When inverting (G < 0), `vin_min` pairs
/// with `vout_max` and `vin_max` with `vout_min`; when non-inverting, `vin_min` pairs
/// with `vout_min` and `vin_max` with `vout_max`.
#[derive(Debug, Clone, Copy)]
struct Side {
    vin: Option<Volts>,
    vout: Option<Volts>,
    vin_given: bool,
    vout_given: bool,
}

impl Side {
    fn new(
        vin: Option<Interval<ElectricPotential>>,
        vout: Option<Interval<ElectricPotential>>,
    ) -> Self {
        let exact = |iv: Interval<ElectricPotential>| Interval::exact(iv.nominal);
        Self {
            vin: vin.map(exact),
            vout: vout.map(exact),
            vin_given: vin.is_some(),
            vout_given: vout.is_some(),
        }
    }

    fn known(&self) -> (bool, bool) {
        (self.vin.is_some(), self.vout.is_some())
    }
}

impl Topology {
    /// Sign of the gain: Vout = sign · |G| · Vin for the internal positive gain.
    fn signed(self, gain: Gain) -> Gain {
        match self {
            Topology::Inverting => -gain,
            Topology::NonInverting => gain,
        }
    }

    fn sign(self) -> Ratio {
        match self {
            Topology::Inverting => -one(),
            Topology::NonInverting => one(),
        }
    }
}

/// Finds the missing quantities of the amplifier from the given ones. The ranges, the gain and
/// the resistors are solved together until nothing new follows.
pub fn solve(input: Input, topology: Topology) -> Result<Solved, SolveError> {
    validate(&input, topology)?;

    let r_pair_given = input.r1.is_some() && input.r2.is_some();
    let r_has_tolerance =
        input.r1.is_some_and(|r| !r.is_exact()) || input.r2.is_some_and(|r| !r.is_exact());
    let gain_overridden = input.gain.is_some() && (r_pair_given || r_has_tolerance);

    let (lo_vout, hi_vout) = match topology {
        Topology::Inverting => (input.vout_max, input.vout_min),
        Topology::NonInverting => (input.vout_min, input.vout_max),
    };
    // `lo` is the end with the smaller Vin, `hi` the one with the larger Vin.
    let mut lo = Side::new(input.vin_min, lo_vout);
    let mut hi = Side::new(input.vin_max, hi_vout);
    let mut r1 = input.r1;
    let mut r2 = input.r2;
    let mut gain = input.gain.map(|g| Interval::exact(g.nominal.abs()));

    loop {
        let before = (
            lo.known(),
            hi.known(),
            r1.is_some(),
            r2.is_some(),
            gain.is_some(),
        );

        (r1, r2, gain) = solve_resistor_triad(r1, r2, gain, topology)?;
        for side in [&mut lo, &mut hi] {
            let signed = gain.map(|g| topology.signed(g));
            let (vin, vout, signed) = solve_voltage_triad(side.vin, side.vout, signed)?;
            (side.vin, side.vout, gain) = (vin, vout, signed.map(|g| topology.signed(g)));
        }
        check_gain_range(gain, topology)?;

        let after = (
            lo.known(),
            hi.known(),
            r1.is_some(),
            r2.is_some(),
            gain.is_some(),
        );
        if before == after {
            break;
        }
    }

    if !gain_overridden {
        check_consistent(&lo, gain, topology)?;
        check_consistent(&hi, gain, topology)?;
    }

    for side in [&mut lo, &mut hi] {
        refine_side(side, gain, gain_overridden, topology);
    }

    let vout_overridden = |side: &Side| gain_overridden && side.vin_given && side.vout_given;
    let vout_computed =
        |side: &Side| side.vout.is_some() && (!side.vout_given || vout_overridden(side));

    // The end of the Vout range that belongs to each Vin end.
    let (vout_min_side, vout_max_side) = match topology {
        Topology::Inverting => (&hi, &lo),
        Topology::NonInverting => (&lo, &hi),
    };

    Ok(Solved {
        vin_min: lo.vin,
        vin_max: hi.vin,
        vout_min: vout_min_side.vout,
        vout_max: vout_max_side.vout,
        r1,
        r2,
        gain: gain.map(|i| topology.signed(i)),
        computed: Computed {
            vin_min: !lo.vin_given && lo.vin.is_some(),
            vin_max: !hi.vin_given && hi.vin.is_some(),
            vout_min: vout_computed(vout_min_side),
            vout_max: vout_computed(vout_max_side),
            r1: input.r1.is_none() && r1.is_some(),
            r2: input.r2.is_none() && r2.is_some(),
            gain: gain.is_some() && (input.gain.is_none() || gain_overridden),
        },
    })
}

/// The gain that came out of the equations must be physically possible.
fn check_gain_range(gain: Option<Gain>, topology: Topology) -> Result<(), SolveError> {
    let Some(g) = gain else {
        return Ok(());
    };
    match topology {
        Topology::Inverting if g.nominal <= Ratio::zero() => Err(SolveError::OutOfRange {
            name: "G".into(),
            requirement: Requirement::Negative,
        }),
        Topology::NonInverting if g.nominal <= one() => Err(SolveError::OutOfRange {
            name: "G".into(),
            requirement: Requirement::GreaterThanOne,
        }),
        _ => Ok(()),
    }
}

/// Applies resistor tolerances to the output end and derives a missing input
/// end from a given output end.
fn refine_side(side: &mut Side, gain: Option<Gain>, gain_overridden: bool, topology: Topology) {
    if gain_overridden
        && side.vin_given
        && let (Some(vi), Some(g)) = (side.vin, gain)
    {
        side.vout = Some(vi * topology.signed(g));
    } else if side.vout_given
        && let (Some(g), Some(vo)) = (gain, side.vout)
        && !g.is_exact()
        && g.nominal > Ratio::zero()
    {
        let a = vo.nominal * (g.lo / g.nominal);
        let b = vo.nominal * (g.hi / g.nominal);
        side.vout = Some(Interval::new(vo.nominal, a.min(b), a.max(b)));
    }

    if !side.vin_given
        && side.vout_given
        && let (Some(vo), Some(g)) = (side.vout, gain)
        && g.nominal > Ratio::zero()
    {
        side.vin = Some(Interval::exact(vo.nominal / (topology.sign() * g.nominal)));
    }
}

fn check_consistent(side: &Side, gain: Option<Gain>, topology: Topology) -> Result<(), SolveError> {
    if let (true, true, Some(vi), Some(vo), Some(g)) =
        (side.vin_given, side.vout_given, side.vin, side.vout, gain)
    {
        let expected = topology.sign() * vi.nominal * g.nominal;
        if (vo.nominal - expected).abs()
            > expected.abs().max(ElectricPotential::new::<volt>(1.0)) * 1e-9
        {
            return Err(SolveError::OutOfRange {
                name: msg!("opamp.name.vout_range"),
                requirement: Requirement::ConsistentVinVoutGain,
            });
        }
    }
    Ok(())
}

fn validate(input: &Input, topology: Topology) -> Result<(), SolveError> {
    let not_positive = |r: Option<Ohms>| r.is_some_and(|r| r.lo <= ElectricalResistance::zero());
    if not_positive(input.r1) || not_positive(input.r2) {
        return Err(SolveError::NotPositive(Quantity::Resistance));
    }

    let volts = |v: Option<Volts>| v.map(|v| v.nominal);

    if let (Some(lo), Some(hi)) = (volts(input.vin_min), volts(input.vin_max))
        && lo > hi
    {
        return Err(SolveError::Ordered {
            left: msg!("opamp.name.vin_min"),
            right: msg!("opamp.name.vin_max"),
        });
    }
    if let (Some(lo), Some(hi)) = (volts(input.vout_min), volts(input.vout_max))
        && lo > hi
    {
        return Err(SolveError::Ordered {
            left: msg!("opamp.name.vout_min"),
            right: msg!("opamp.name.vout_max"),
        });
    }

    match topology {
        Topology::Inverting => {
            if input.gain.is_some_and(|g| g.contains_zero()) {
                return Err(SolveError::OutOfRange {
                    name: "G".into(),
                    requirement: Requirement::NonZero,
                });
            }
            if input.gain.is_some_and(|g| g.nominal > Ratio::zero()) {
                return Err(SolveError::OutOfRange {
                    name: "G".into(),
                    requirement: Requirement::Negative,
                });
            }
        }
        Topology::NonInverting => {
            if input.gain.is_some_and(|g| g.lo <= one()) {
                return Err(SolveError::OutOfRange {
                    name: "G".into(),
                    requirement: Requirement::GreaterThanOne,
                });
            }
        }
    }
    Ok(())
}

type Triad<A, B> = (Option<A>, Option<B>, Option<Gain>);

fn solve_voltage_triad(
    vin: Option<Volts>,
    vout: Option<Volts>,
    gain: Option<Gain>,
) -> Result<Triad<Volts, Volts>, SolveError> {
    match (vin, vout, gain) {
        (Some(v), Some(o), None) if v.nominal != ElectricPotential::zero() => {
            let g = divide(o, v, "Vin")?;
            Ok((Some(v), Some(o), Some(g)))
        }

        (Some(v), None, Some(g)) => Ok((Some(v), Some(v * g), Some(g))),

        (None, Some(o), Some(g)) => {
            let v = divide(o, g, "G")?;
            Ok((Some(v), Some(o), Some(g)))
        }

        (v, o, g) => Ok((v, o, g)),
    }
}

/// Inverting: |G| = R2/R1. Non-inverting: G = 1 + R1/R2.
fn solve_resistor_triad(
    r1: Option<Ohms>,
    r2: Option<Ohms>,
    gain: Option<Gain>,
    topology: Topology,
) -> Result<Triad<Ohms, Ohms>, SolveError> {
    match topology {
        Topology::Inverting => match (r1, r2, gain) {
            (Some(a), Some(b), _) => {
                let g = divide(b, a, "R1")?;
                Ok((Some(a), Some(b), Some(g)))
            }

            (Some(a), None, Some(g)) => {
                let b = with_tolerance_of(a, a.nominal * g.nominal);
                Ok((Some(a), Some(b), Some(g)))
            }

            (None, Some(b), Some(g)) => {
                let a = with_tolerance_of(b, b.nominal / g.nominal);
                Ok((Some(a), Some(b), Some(g)))
            }

            (a, b, g) => Ok((a, b, g)),
        },
        Topology::NonInverting => match (r1, r2, gain) {
            (Some(a), Some(b), _) => {
                let g = Interval::exact(one()) + divide(a, b, "R2")?;
                Ok((Some(a), Some(b), Some(g)))
            }

            (Some(a), None, Some(g)) => {
                let b = with_tolerance_of(a, a.nominal / excess(g)?);
                Ok((Some(a), Some(b), Some(g)))
            }

            (None, Some(b), Some(g)) => {
                let a = with_tolerance_of(b, b.nominal * excess(g)?);
                Ok((Some(a), Some(b), Some(g)))
            }

            (a, b, g) => Ok((a, b, g)),
        },
    }
}

/// G - 1 = R1/R2, which must be positive.
fn excess(gain: Gain) -> Result<Ratio, SolveError> {
    if gain.nominal <= one() {
        return Err(SolveError::OutOfRange {
            name: "G".into(),
            requirement: Requirement::GreaterThanOne,
        });
    }
    Ok(gain.nominal - one())
}

/// Interval around `nominal` carrying the same relative tolerance as `like`.
fn with_tolerance_of<T>(like: Interval<T>, nominal: T) -> Interval<T>
where
    T: Copy + Div<T, Output = Ratio> + Mul<Ratio, Output = T>,
{
    Interval::new(
        nominal,
        nominal * (like.lo / like.nominal),
        nominal * (like.hi / like.nominal),
    )
}

fn divide<T, U>(
    num: Interval<T>,
    den: Interval<U>,
    name: &'static str,
) -> Result<Interval<<T as Div<U>>::Output>, SolveError>
where
    T: Copy + Div<U>,
    U: Copy + PartialOrd + Zero,
    <T as Div<U>>::Output: Copy + PartialOrd,
{
    (num / den).ok_or(SolveError::OutOfRange {
        name: name.into(),
        requirement: Requirement::NonZero,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::electrical_resistance::ohm;

    fn volts(v: f64) -> Option<Interval<ElectricPotential>> {
        Some(Interval::exact(ElectricPotential::new::<volt>(v)))
    }

    fn ohms(v: f64) -> Option<Interval<ElectricalResistance>> {
        Some(Interval::exact(ElectricalResistance::new::<ohm>(v)))
    }

    fn vout(solved: &Solved) -> (f64, f64) {
        let get = |v: Option<Interval<ElectricPotential>>| v.unwrap().nominal.get::<volt>();
        (get(solved.vout_min), get(solved.vout_max))
    }

    #[test]
    fn non_inverting_range_from_resistors() {
        let input = Input {
            vin_min: volts(1.0),
            vin_max: volts(2.0),
            r1: ohms(10_000.0),
            r2: ohms(1_000.0),
            ..Default::default()
        };
        let solved = solve(input, Topology::NonInverting).unwrap();
        assert!((solved.gain.unwrap().nominal.get::<ratio>() - 11.0).abs() < 1e-9);
        let (lo, hi) = vout(&solved);
        assert!((lo - 11.0).abs() < 1e-9 && (hi - 22.0).abs() < 1e-9);
    }

    #[test]
    fn non_inverting_resistor_from_ranges() {
        // G = (22 - 11) / (2 - 1) = 11, so R1 = (G - 1) * R2.
        let input = Input {
            vin_min: volts(1.0),
            vin_max: volts(2.0),
            vout_min: volts(11.0),
            vout_max: volts(22.0),
            r2: ohms(1_000.0),
            ..Default::default()
        };
        let solved = solve(input, Topology::NonInverting).unwrap();
        assert!((solved.gain.unwrap().nominal.get::<ratio>() - 11.0).abs() < 1e-9);
        assert!((solved.r1.unwrap().nominal.get::<ohm>() - 10_000.0).abs() < 1e-6);
    }

    #[test]
    fn non_inverting_gain_must_exceed_one() {
        let input = Input {
            vin_min: volts(2.0),
            vout_min: volts(1.0),
            ..Default::default()
        };
        assert!(solve(input, Topology::NonInverting).is_err());
    }

    #[test]
    fn inverting_pairs_min_with_max() {
        let input = Input {
            vin_min: volts(-1.0),
            vin_max: volts(1.0),
            r1: ohms(1_000.0),
            r2: ohms(5_000.0),
            ..Default::default()
        };
        let solved = solve(input, Topology::Inverting).unwrap();
        assert!((solved.gain.unwrap().nominal.get::<ratio>() + 5.0).abs() < 1e-9);
        let (lo, hi) = vout(&solved);
        assert!((lo + 5.0).abs() < 1e-9 && (hi - 5.0).abs() < 1e-9);
    }
}

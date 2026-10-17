//! The calculation of RMS and average current for the chosen waveform.

use uom::si::f64::{ElectricCurrent, Ratio, Time};

use crate::domain::error::SolveError;
use crate::domain::interval::Interval;
use crate::domain::quantity::Quantity;
use crate::domain::waveform;
use uom::num_traits::Zero;

use super::WaveformKind;

/// The request of the calculation: the kind of waveform and its values. `ip` is the peak
/// (`Imax` for the triangles); a value that the kind does not use is `None`.
#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub kind: WaveformKind,
    pub ip: Interval<ElectricCurrent>,
    pub ip2: Option<Interval<ElectricCurrent>>,
    pub imin: Option<Interval<ElectricCurrent>>,
    pub ton: Option<Interval<Time>>,
    pub t1: Option<Interval<Time>>,
    pub t2: Option<Interval<Time>>,
    pub period: Interval<Time>,
}

/// RMS and average current and the duty cycle of the pulse.
#[derive(Debug, Clone, Copy)]
pub struct Report {
    pub i_rms: Interval<ElectricCurrent>,
    pub i_avg: Interval<ElectricCurrent>,

    pub duty: Interval<Ratio>,
}

/// Calculates the waveform after checking the input (positive times, pulse within the period,
/// currents in the right order).
pub fn solve(input: Input) -> Result<Report, SolveError> {
    validate(&input)?;

    let (ip, period) = (input.ip, input.period);

    let stats = match input.kind {
        WaveformKind::HalfSine => {
            let ton = required(input.ton, "ton")?;
            waveform::half_sine(ip, ton, period)?
        }
        WaveformKind::Rectangular => {
            let ton = required(input.ton, "ton")?;
            waveform::rectangular(ip, ton, period)?
        }
        WaveformKind::Trapezoidal => {
            let ip2 = required(input.ip2, "Ip2")?;
            let ton = required(input.ton, "ton")?;
            waveform::trapezoidal(ip, ip2, ton, period)?
        }
        WaveformKind::Triangular => {
            let t1 = required(input.t1, "T1")?;
            let t2 = required(input.t2, "T2")?;
            waveform::triangular(ip, t1, t2, period)?
        }
        WaveformKind::TriangularOffset => {
            let imin = required(input.imin, "Imin")?;
            let t1 = required(input.t1, "T1")?;
            let t2 = required(input.t2, "T2")?;
            waveform::triangular_offset(imin, ip, t1, t2, period)?
        }
        WaveformKind::BipolarTriangular => {
            let imin = required(input.imin, "Imin")?;
            let t1 = required(input.t1, "T1")?;
            let t2 = required(input.t2, "T2")?;
            waveform::bipolar_triangular(imin, ip, t1, t2, period)?
        }
    };

    Ok(Report {
        i_rms: stats.rms,
        i_avg: stats.avg,
        duty: stats.duty,
    })
}

fn required<T: Copy>(
    value: Option<Interval<T>>,
    name: &'static str,
) -> Result<Interval<T>, SolveError> {
    value.ok_or(SolveError::MissingInput { name: name.into() })
}

fn validate(input: &Input) -> Result<(), SolveError> {
    if input.period.lo <= Time::zero() {
        return Err(SolveError::BadPeriod);
    }

    use WaveformKind::*;
    match input.kind {
        HalfSine | Rectangular => validate_single_pulse(input)?,
        Trapezoidal => {
            validate_single_pulse(input)?;
            if let Some(ip2) = input.ip2
                && ip2.lo < ElectricCurrent::zero()
            {
                return Err(SolveError::NotPositive(Quantity::Current));
            }
        }
        Triangular => validate_triangular(input, false)?,
        TriangularOffset => validate_triangular(input, true)?,
        BipolarTriangular => validate_bipolar(input)?,
    }
    Ok(())
}

fn validate_single_pulse(input: &Input) -> Result<(), SolveError> {
    let ton = required(input.ton, "ton")?;
    if ton.nominal <= Time::zero() {
        return Err(SolveError::BadPulseWidth);
    }
    if ton.nominal > input.period.nominal {
        return Err(SolveError::PulseExceedsPeriod);
    }
    if input.ip.lo < ElectricCurrent::zero() {
        return Err(SolveError::NotPositive(Quantity::Current));
    }
    Ok(())
}

fn validate_triangular(input: &Input, needs_min: bool) -> Result<(), SolveError> {
    let t1 = required(input.t1, "T1")?;
    let t2 = required(input.t2, "T2")?;

    if input.ip.lo <= ElectricCurrent::zero() {
        return Err(SolveError::NotPositive(Quantity::Current));
    }
    if t1.nominal <= Time::zero() || t2.nominal <= Time::zero() {
        return Err(SolveError::BadPulseWidth);
    }

    if needs_min {
        let imin = required(input.imin, "Imin")?;
        if imin.lo <= ElectricCurrent::zero() {
            return Err(SolveError::NotPositive(Quantity::Current));
        }
        if imin.nominal >= input.ip.nominal {
            return Err(SolveError::Ordered {
                left: "Imin".into(),
                right: "Imax".into(),
            });
        }
    }

    let total = t1.nominal + t2.nominal;
    if total > input.period.nominal {
        return Err(SolveError::PulseExceedsPeriod);
    }
    Ok(())
}

fn validate_bipolar(input: &Input) -> Result<(), SolveError> {
    let imin = required(input.imin, "Imin")?;
    let t1 = required(input.t1, "T1")?;
    let t2 = required(input.t2, "T2")?;

    if input.ip.lo <= ElectricCurrent::zero() {
        return Err(SolveError::NotPositive(Quantity::Current));
    }

    if imin.hi >= ElectricCurrent::zero() {
        return Err(SolveError::NotPositive(Quantity::Current));
    }
    if t1.nominal <= Time::zero() || t2.nominal <= Time::zero() {
        return Err(SolveError::BadPulseWidth);
    }
    let total = t1.nominal + t2.nominal;
    if total > input.period.nominal {
        return Err(SolveError::PulseExceedsPeriod);
    }
    Ok(())
}

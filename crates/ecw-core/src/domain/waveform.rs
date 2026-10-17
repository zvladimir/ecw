//! RMS and average of periodic current waveforms, with tolerances as intervals.

use std::f64::consts::PI;
use std::ops::Div;

use uom::si::f64::{ElectricCurrent, Ratio, Time};

use crate::domain::error::SolveError;
use crate::domain::interval::Interval;

type Current = Interval<ElectricCurrent>;
type Duration = Interval<Time>;

/// The result for a waveform: RMS and average current and the duty cycle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stats {
    /// The root-mean-square current over the period.
    pub rms: Current,

    /// The average current over the period.
    pub avg: Current,

    /// The share of the period that the pulse takes.
    pub duty: Interval<Ratio>,
}

/// `value / period`; the period must not reach zero.
fn per_period<T>(
    value: Interval<T>,
    period: Duration,
) -> Result<Interval<<T as Div<Time>>::Output>, SolveError>
where
    T: Copy + Div<Time>,
    <T as Div<Time>>::Output: Copy + PartialOrd,
{
    (value / period).ok_or(SolveError::BadPeriod)
}

fn duty(t_on: Duration, period: Duration) -> Result<Interval<Ratio>, SolveError> {
    per_period(t_on, period)
}

/// The square root of a value that must not be negative; `name` says which one it is.
fn checked_sqrt<T, R>(
    value: Interval<T>,
    sqrt: impl Fn(T) -> R,
    name: &'static str,
) -> Result<Interval<R>, SolveError>
where
    T: Copy + PartialOrd + uom::num_traits::Zero,
{
    value
        .try_sqrt(sqrt)
        .ok_or(SolveError::ComputedNotPositive { name: name.into() })
}

/// Half-sine pulse of the peak `ip` and width `t_on` once per `period`.
pub fn half_sine(ip: Current, t_on: Duration, period: Duration) -> Result<Stats, SolveError> {
    let d = duty(t_on, period)?;
    let half_duty = d.map(|v| v / 2.0);
    let sqrt_half = checked_sqrt(half_duty, |v| v.sqrt(), "D/2")?;
    let rms = ip * sqrt_half;
    let two_over_pi = Interval::exact(2.0 / PI);
    let avg = ip * two_over_pi * d;
    Ok(Stats { rms, avg, duty: d })
}

/// Rectangular pulse: `ip` for `t_on`, zero for the rest of the period.
pub fn rectangular(ip: Current, t_on: Duration, period: Duration) -> Result<Stats, SolveError> {
    let d = duty(t_on, period)?;
    let sqrt_duty = checked_sqrt(d, |v| v.sqrt(), "D")?;
    let rms = ip * sqrt_duty;
    let avg = ip * d;
    Ok(Stats { rms, avg, duty: d })
}

/// A pulse whose current goes linearly from `ip1` to `ip2` over `t_on`; zero for the rest of the
/// period.
pub fn trapezoidal(
    ip1: Current,
    ip2: Current,
    t_on: Duration,
    period: Duration,
) -> Result<Stats, SolveError> {
    let d = duty(t_on, period)?;
    let sum = ip1 * ip1 + ip1 * ip2 + ip2 * ip2;
    let scaled = sum * d.map(|v| v / 3.0);
    let rms = checked_sqrt(scaled, |v| v.sqrt(), "D·(Ip1² + Ip1·Ip2 + Ip2²)/3")?;
    let half_sum = (ip1 + ip2).map(|v| v / 2.0);
    let avg = half_sum * d;
    Ok(Stats { rms, avg, duty: d })
}

/// A triangle with the peak `ip`: it rises during `t1` and falls during `t2`; zero for the rest
/// of the period.
pub fn triangular(
    ip: Current,
    t1: Duration,
    t2: Duration,
    period: Duration,
) -> Result<Stats, SolveError> {
    let d = duty(t1 + t2, period)?;

    let third = d.map(|v| v / 3.0);
    let rms = ip * checked_sqrt(third, |v| v.sqrt(), "D/3")?;

    let avg = ip.map(|v| v / 2.0) * d;

    Ok(Stats { rms, avg, duty: d })
}

/// A triangle that swings between `imin` and `imax` (both above zero) during `t1 + t2`; zero for
/// the rest of the period.
pub fn triangular_offset(
    imin: Current,
    imax: Current,
    t1: Duration,
    t2: Duration,
    period: Duration,
) -> Result<Stats, SolveError> {
    let d = duty(t1 + t2, period)?;

    let i_sum = (imin + imax).map(|v| v / 2.0);
    let avg = i_sum * d;

    let sum_sq = imin * imin + imin * imax + imax * imax;
    let scaled = sum_sq * d.map(|v| v / 3.0);
    let rms = checked_sqrt(scaled, |v| v.sqrt(), "D·(min² + min·max + max²)/3")?;

    Ok(Stats { rms, avg, duty: d })
}

/// Two triangles: a negative one with the peak `imin` during `t1` and a positive one with the
/// peak `imax` during `t2`.
pub fn bipolar_triangular(
    imin: Current,
    imax: Current,
    t1: Duration,
    t2: Duration,
    period: Duration,
) -> Result<Stats, SolveError> {
    let d = duty(t1 + t2, period)?;

    let avg_neg = per_period(imin.map(|v| v / 2.0) * t1, period)?;
    let avg_pos = per_period(imax.map(|v| v / 2.0) * t2, period)?;
    let avg = avg_neg + avg_pos;

    let sq_neg = per_period((imin * imin) * t1, period)?.map(|v| v / 3.0);
    let sq_pos = per_period((imax * imax) * t2, period)?.map(|v| v / 3.0);
    let rms = checked_sqrt(sq_neg + sq_pos, |v| v.sqrt(), "(min²·t1 + max²·t2)/(3T)")?;

    Ok(Stats { rms, avg, duty: d })
}

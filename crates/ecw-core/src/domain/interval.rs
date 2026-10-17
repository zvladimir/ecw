use std::ops::{Add, Div, Mul, Neg, Sub};

use uom::num_traits::Zero;

/// A value with its worst case: the nominal and the lowest and highest it can be.
/// Arithmetic keeps the bounds: `a - b` runs from `a.lo - b.hi` to `a.hi - b.lo`, so the result
/// is the true range, not a sum of independent errors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval<T> {
    /// The value without tolerances.
    pub nominal: T,
    /// The lowest value.
    pub lo: T,
    /// The highest value.
    pub hi: T,
}

impl<T> Interval<T> {
    /// An interval with explicit nominal and bounds.
    #[inline]
    pub const fn new(nominal: T, lo: T, hi: T) -> Self {
        Self { nominal, lo, hi }
    }
}

impl<T: Copy> Interval<T> {
    /// A value without a tolerance: `lo == nominal == hi`.
    #[inline]
    pub const fn exact(v: T) -> Self {
        Self {
            nominal: v,
            lo: v,
            hi: v,
        }
    }

    /// True if there is no spread.
    #[inline]
    pub fn is_exact(&self) -> bool
    where
        T: PartialEq,
    {
        self.lo == self.hi
    }

    /// Applies `f` to the nominal and both bounds. `f` must be increasing, or the bounds swap.
    #[inline]
    pub fn map<U, F>(self, f: F) -> Interval<U>
    where
        F: Fn(T) -> U,
    {
        Interval {
            nominal: f(self.nominal),
            lo: f(self.lo),
            hi: f(self.hi),
        }
    }
}

impl<T> Add for Interval<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            nominal: self.nominal + rhs.nominal,
            lo: self.lo + rhs.lo,
            hi: self.hi + rhs.hi,
        }
    }
}

impl<T> Sub for Interval<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            nominal: self.nominal - rhs.nominal,
            lo: self.lo - rhs.hi,
            hi: self.hi - rhs.lo,
        }
    }
}

impl<T, U> Mul<Interval<U>> for Interval<T>
where
    T: Copy + Mul<U>,
    U: Copy,
    <T as Mul<U>>::Output: Copy + PartialOrd,
{
    type Output = Interval<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Interval<U>) -> Self::Output {
        let products = [
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ];
        let (lo, hi) = min_max(&products);
        Interval {
            nominal: self.nominal * rhs.nominal,
            lo,
            hi,
        }
    }
}

impl<T, U> Div<Interval<U>> for Interval<T>
where
    T: Copy + Div<U>,
    U: Copy + PartialOrd + Zero,
    <T as Div<U>>::Output: Copy + PartialOrd,
{
    type Output = Option<Interval<<T as Div<U>>::Output>>;

    fn div(self, rhs: Interval<U>) -> Self::Output {
        let zero = U::zero();
        if rhs.lo <= zero && rhs.hi >= zero {
            return None;
        }
        let quotients = [
            self.lo / rhs.lo,
            self.lo / rhs.hi,
            self.hi / rhs.lo,
            self.hi / rhs.hi,
        ];
        let (lo, hi) = min_max(&quotients);
        Some(Interval {
            nominal: self.nominal / rhs.nominal,
            lo,
            hi,
        })
    }
}

impl<T: Copy + Neg<Output = T>> Neg for Interval<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            nominal: -self.nominal,
            lo: -self.hi,
            hi: -self.lo,
        }
    }
}

impl<T: Copy + PartialOrd + Zero> Interval<T> {
    /// The square root, taken by `sqrt` (`|x| x.sqrt()`: on a quantity it halves the dimension,
    /// so the result has its own type); `None` when the interval reaches below zero.
    pub fn try_sqrt<R>(self, sqrt: impl Fn(T) -> R) -> Option<Interval<R>> {
        (self.lo >= T::zero()).then(|| self.map(sqrt))
    }

    /// True if zero lies within the bounds.
    pub fn contains_zero(&self) -> bool {
        self.lo <= T::zero() && self.hi >= T::zero()
    }
}

#[inline]
fn min_max<T: Copy + PartialOrd, const N: usize>(values: &[T; N]) -> (T, T) {
    debug_assert!(N > 0);
    let mut lo = values[0];
    let mut hi = values[0];
    for v in &values[1..] {
        if *v < lo {
            lo = *v;
        }
        if *v > hi {
            hi = *v;
        }
    }
    (lo, hi)
}

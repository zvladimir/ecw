use std::ops::Div;

use crate::domain::error::SolveError;
use crate::domain::interval::Interval;
use crate::domain::quantity::Quantity;
use crate::domain::unit::Measured;
use uom::num_traits::Zero;

/// `interval` itself, or `NotPositive(quantity)` if its lower bound is not above zero.
#[inline]
pub fn require_positive<T: Measured>(
    interval: Interval<T>,
    quantity: Quantity,
) -> Result<Interval<T>, SolveError> {
    if interval.lo <= T::zero() {
        return Err(SolveError::NotPositive(quantity));
    }
    Ok(interval)
}

/// Checks a value that may be missing: only a given value has to be positive.
#[inline]
pub fn check_positive<T: Measured>(
    value: Option<Interval<T>>,
    quantity: Quantity,
) -> Result<(), SolveError> {
    if let Some(interval) = value {
        require_positive(interval, quantity)?;
    }
    Ok(())
}

/// `numerator / denominator`, or `NotPositive(denom_quantity)` if the denominator's range
/// reaches zero. The error names the quantity the user has to change.
#[inline]
pub fn checked_div<T, U>(
    numerator: Interval<T>,
    denominator: Interval<U>,
    denom_quantity: Quantity,
) -> Result<Interval<<T as Div<U>>::Output>, SolveError>
where
    T: Copy + Div<U>,
    U: Copy + PartialOrd + Zero,
    <T as Div<U>>::Output: Copy + PartialOrd,
{
    (numerator / denominator).ok_or(SolveError::NotPositive(denom_quantity))
}

macro_rules! impl_count_given {
    ($ty:ty { $($field:ident),+ $(,)? }) => {
        impl $ty {
            /// The number of fields of the request that are given (not `None`).
            pub fn count_given(&self) -> usize {
                0 $(+ usize::from(self.$field.is_some()))+
            }
        }
    };
}

pub(crate) use impl_count_given;

/// `(lo, hi)` is a range that goes up: `lo < hi`. False also when either end is NaN, so a
/// broken range is refused like an empty one.
#[inline]
pub fn ascending<T: PartialOrd>((lo, hi): (T, T)) -> bool {
    lo.partial_cmp(&hi) == Some(std::cmp::Ordering::Less)
}

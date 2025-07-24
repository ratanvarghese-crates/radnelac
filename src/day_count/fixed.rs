use crate::common::bound::BoundedDayCount;
use crate::common::bound::EffectiveBound;
use crate::common::math::TermNum;
use crate::common::math::EFFECTIVE_MAX;
use crate::common::math::EFFECTIVE_MIN;

const FIXED_MAX_SCALE: f64 = 2048.0;

/// Maximum supported value for a `Fixed`
///
/// This somewhere after January 1, 47000000 Common Era in the proleptic Gregorian calendar.
///
/// It is possible to create a `Fixed` at with a higher value - this is permitted for
/// intermediate results of calculations. However in general a `Fixed` with a value beyond
/// `FIXED_MAX` is at risk of reduced accuracy calculations.
pub const FIXED_MAX: f64 = (EFFECTIVE_MAX * (FIXED_MAX_SCALE - 1.0)) / FIXED_MAX_SCALE;
/// Minimum supported value for a `Fixed`
///
/// This somewhere before January 1, 47000000 Before Common Era in the proleptic Gregorian calendar.
///
/// It is possible to create a `Fixed` at with a lower value - this is permitted for
/// intermediate results of calculations. However in general a `Fixed` with a value beyond
/// `FIXED_MIN` is at risk of reduced accuracy calculations.
pub const FIXED_MIN: f64 = (EFFECTIVE_MIN * (FIXED_MAX_SCALE - 1.0)) / FIXED_MAX_SCALE;

/// Represents a fixed point in time
///
/// This is internally a floating point number, where the integer portion represents a
/// particular day and the fractional portion represents a particular time of day.
///
/// The epoch used for this data structure is considered an internal implementation detail.
///
/// Note that equality and ordering operations are subject to limitations similar to
/// equality and ordering operations on a floating point number. Two `Fixed` values represent
/// the same day or even the same second, but still appear different on the sub-second level.
/// Use `get_day_i` to compare days, and use `same_second` to compare seconds.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Fixed(f64);

impl Fixed {
    /// Returns a new `Fixed` with day 0 and the same time of day.
    pub fn to_time_of_day(self) -> Fixed {
        Fixed(self.0.modulus(1.0))
    }

    /// Returns a new `Fixed` with the same day and midnight as the time of day.
    pub fn to_day(self) -> Fixed {
        Fixed(self.0.floor())
    }

    /// Returns the day as an integer
    pub fn get_day_i(self) -> i64 {
        self.to_day().get() as i64
    }

    /// Returns true if `self` and `other` represent the same second of time.
    ///
    ///
    pub fn same_second(self, other: Self) -> bool {
        self.0.approx_eq(other.0)
    }
}

impl EffectiveBound for Fixed {
    fn effective_min() -> Fixed {
        Fixed(FIXED_MIN)
    }

    fn effective_max() -> Fixed {
        Fixed(FIXED_MAX)
    }
}

impl BoundedDayCount<f64> for Fixed {
    fn new(t: f64) -> Fixed {
        //It's expected to go beyond the FIXED_MAX/FIXED_MIN for intermediate results
        //of calculations. However going beyond EFFECTIVE_MAX/EFFECTIVE_MIN is
        //probably a bug.
        debug_assert!(
            Fixed::almost_in_effective_bounds(t, FIXED_MAX / FIXED_MAX_SCALE).is_ok(),
            "t = {}",
            t
        );
        Fixed(t)
    }
    fn get(self) -> f64 {
        self.0
    }
}

pub trait FromFixed: Copy + Clone {
    fn from_fixed(t: Fixed) -> Self;
}

pub trait ToFixed: Copy + Clone {
    fn to_fixed(self) -> Fixed;
    fn convert<T: FromFixed>(self) -> T {
        T::from_fixed(self.to_fixed())
    }
}

pub trait Epoch: FromFixed {
    fn epoch() -> Fixed;
}

pub trait CalculatedBounds: FromFixed + ToFixed + PartialEq + PartialOrd {}

impl<T: CalculatedBounds> EffectiveBound for T {
    fn effective_min() -> Self {
        Self::from_fixed(Fixed::effective_min())
    }

    fn effective_max() -> Self {
        Self::from_fixed(Fixed::effective_max())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::math::EFFECTIVE_EPSILON;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use proptest::proptest;

    #[test]
    fn bounds_propeties() {
        assert!(FIXED_MAX < EFFECTIVE_MAX && FIXED_MAX > (EFFECTIVE_MAX / 2.0));
        assert!(FIXED_MIN > EFFECTIVE_MIN && FIXED_MIN < (EFFECTIVE_MIN / 2.0));
    }

    #[test]
    fn reject_weird() {
        let weird_values = [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            FIXED_MAX + 1.0,
            FIXED_MIN - 1.0,
        ];
        for x in weird_values {
            assert!(Fixed::in_effective_bounds(x).is_err());
        }
    }

    #[test]
    fn accept_ok() {
        let ok_values = [FIXED_MAX, FIXED_MIN, 0.0, -0.0, EFFECTIVE_EPSILON];
        for x in ok_values {
            assert!(Fixed::in_effective_bounds(x).is_ok());
        }
    }

    #[test]
    fn comparisons() {
        let f_min = Fixed::effective_min();
        let f_mbig = Fixed::new(-100000.0 * 365.25);
        let f_m1 = Fixed::new(-1.0);
        let f_0 = Fixed::new(0.0);
        let f_p1 = Fixed::new(1.0);
        let f_pbig = Fixed::new(100000.0 * 365.25);
        let f_max = Fixed::effective_max();
        assert!(f_min < f_mbig);
        assert!(f_mbig < f_m1);
        assert!(f_m1 < f_0);
        assert!(f_0 < f_p1);
        assert!(f_p1 < f_pbig);
        assert!(f_pbig < f_max);
    }

    proptest! {
        #[test]
        fn time_of_day(t in FIXED_MIN..FIXED_MAX) {
            let f = Fixed::new(t).to_time_of_day().get();
            assert!(f <= 1.0 && f >= 0.0);
        }

        #[test]
        fn day(t in FIXED_MIN..FIXED_MAX) {
            let f = Fixed::new(t).to_day().get();
            assert!(f <= (t + 1.0) && f >= (t - 1.0));
            let d = Fixed::new(t).get_day_i();
            assert_eq!(d as f64, f);
        }
    }
}

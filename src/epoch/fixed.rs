use super::common::bounded_small_int_guaranteed;
use super::common::simple_bounded;
use crate::error::CalendarError;
use crate::math::TermNum;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;
use num_traits::Bounded;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::Sub;

pub trait DayCount: TermNum {}

impl DayCount for f64 {}
impl DayCount for i64 {}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Fixed<T: DayCount>(T);

pub type FixedMoment = Fixed<f64>;
pub type FixedDate = Fixed<i64>;

// Between the two fixed types

impl From<FixedMoment> for FixedDate {
    fn from(t: FixedMoment) -> FixedDate {
        FixedDate {
            0: t.0.floor() as i64,
        }
    }
}

impl From<FixedDate> for FixedMoment {
    fn from(date: FixedDate) -> FixedMoment {
        FixedMoment { 0: date.0 as f64 }
    }
}

// Bounded

simple_bounded!(f64, FixedMoment, EFFECTIVE_MIN, EFFECTIVE_MAX);
simple_bounded!(i64, FixedDate, EFFECTIVE_MIN, EFFECTIVE_MAX);

// Larger number primitives

macro_rules! fixed_from_big_int {
    ($t:ident, $u: ident) => {
        impl TryFrom<$t> for Fixed<$u> {
            type Error = CalendarError;
            fn try_from(date: $t) -> Result<Fixed<$u>, Self::Error> {
                Ok(Fixed::<$u>::from(Fixed::<$t>::try_from(date)?))
            }
        }

        impl From<Fixed<$u>> for $t {
            fn from(date: Fixed<$u>) -> $t {
                $t::from(Fixed::<$t>::from(date))
            }
        }
    };
}

fixed_from_big_int!(f64, i64);
fixed_from_big_int!(i64, f64);

// Smaller integer primitives (not bothering with f32)

bounded_small_int_guaranteed!(i32, f64, FixedMoment);
bounded_small_int_guaranteed!(i16, f64, FixedMoment);
bounded_small_int_guaranteed!(i8, f64, FixedMoment);
bounded_small_int_guaranteed!(i32, i64, FixedDate);
bounded_small_int_guaranteed!(i16, i64, FixedDate);
bounded_small_int_guaranteed!(i8, i64, FixedDate);

// Sub and Add

impl<T: Sub<Output = T> + DayCount> Sub for Fixed<T> {
    type Output = T;
    fn sub(self, other: Self) -> Self::Output {
        self.0 - other.0
    }
}

impl<T: Add<Output = T> + DayCount> Add<T> for Fixed<T> {
    type Output = T;
    fn add(self, other: T) -> Self::Output {
        self.0 + other
    }
}

pub trait Epoch {
    fn epoch() -> FixedDate;
}

pub trait EpochMoment {
    fn epoch_moment() -> FixedMoment;
}

impl<T: EpochMoment> Epoch for T {
    fn epoch() -> FixedDate {
        FixedDate::from(Self::epoch_moment())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::EFFECTIVE_EPSILON;
    use crate::math::EFFECTIVE_MAX;
    use crate::math::EFFECTIVE_MIN;
    use proptest::proptest;

    #[test]
    fn reject_weird() {
        let weird_values = [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            EFFECTIVE_MAX + 1.0,
            EFFECTIVE_MIN - 1.0,
        ];
        for x in weird_values {
            println!("{:?}", x);
            assert!(FixedMoment::try_from(x).is_err());
            assert!(FixedDate::try_from(x).is_err());
        }
    }

    #[test]
    fn accept_ok() {
        let ok_values = [EFFECTIVE_MAX, EFFECTIVE_MIN, 0.0, -0.0, EFFECTIVE_EPSILON];
        for x in ok_values {
            assert!(FixedMoment::try_from(x).is_ok());
            assert!(FixedDate::try_from(x).is_ok());
        }
    }

    #[test]
    fn bad_roundtrip() {
        let start = FixedMoment::from(i32::MAX);
        let next = FixedMoment::try_from(i64::try_from(start).unwrap() + 1).unwrap();
        let last = i32::try_from(next);
        assert!(last.is_err());
    }

    #[test]
    fn good_roundtrip() {
        let start = FixedMoment::from(i32::MAX);
        let next = FixedMoment::try_from(i64::try_from(start).unwrap()).unwrap();
        let last = i32::try_from(next).unwrap();
        assert_eq!(last, i32::MAX);
    }

    proptest! {
        #[test]
        fn easy_i32(t in i32::MIN..i32::MAX) {
            let j0 = FixedMoment::try_from(t as f64).unwrap();
            let j1 = FixedMoment::from(t);
            assert_eq!(j0, j1);
        }
    }
}

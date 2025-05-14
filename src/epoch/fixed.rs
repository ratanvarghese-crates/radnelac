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

macro_rules! fixed_bounded {
    ($t: ident) => {
        impl Bounded for Fixed<$t> {
            fn min_value() -> Fixed<$t> {
                Fixed::<$t> {
                    0: EFFECTIVE_MIN as $t,
                }
            }

            fn max_value() -> Fixed<$t> {
                Fixed::<$t> {
                    0: EFFECTIVE_MAX as $t,
                }
            }
        }
    };
}

fixed_bounded!(f64);
fixed_bounded!(i64);

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

        impl TryFrom<$t> for Fixed<$t> {
            type Error = CalendarError;
            fn try_from(date: $t) -> Result<Fixed<$t>, Self::Error> {
                if (date.is_weird()) {
                    Err(CalendarError::NaNInfinite)
                } else if (date < Fixed::<$t>::min_value().0) {
                    Err(CalendarError::OutOfBounds)
                } else if (date > Fixed::<$t>::max_value().0) {
                    Err(CalendarError::OutOfBounds)
                } else {
                    Ok(Fixed::<$t> { 0: date })
                }
            }
        }

        impl From<Fixed<$t>> for $t {
            fn from(date: Fixed<$t>) -> $t {
                date.0
            }
        }
    };
}

fixed_from_big_int!(f64, i64);
fixed_from_big_int!(i64, f64);

// Smaller integer primitives (not bothering with f32)

macro_rules! fixed_from_small_int {
    ($t:ident, $u: ident) => {
        impl From<$t> for Fixed<$u> {
            fn from(date: $t) -> Fixed<$u> {
                Fixed::<$u>::try_from(date as i64).expect("Known to be within bounds.")
            }
        }

        impl TryFrom<Fixed<$u>> for $t {
            type Error = CalendarError;
            fn try_from(date: Fixed<$u>) -> Result<$t, Self::Error> {
                if (date.0 < ($t::min_value() as $u) || date.0 > ($t::max_value() as $u)) {
                    Err(CalendarError::OutOfBounds)
                } else {
                    Ok(date.0 as $t)
                }
            }
        }
    };
}

fixed_from_small_int!(i32, i64);
fixed_from_small_int!(i32, f64);
fixed_from_small_int!(i16, i64);
fixed_from_small_int!(i16, f64);
fixed_from_small_int!(i8, i64);
fixed_from_small_int!(i8, f64);

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
}

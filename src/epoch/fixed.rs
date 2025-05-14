use crate::error::CalendarError;
use crate::math::TermNum;
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
                Ok(Fixed::<$t> {
                    0: date.within_eff()?,
                })
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
                Ok(i64::from(date).within_type::<$t>()? as $t)
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

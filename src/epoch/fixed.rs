use crate::error::CalendarError;
use crate::math::TermNum;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Fixed<T: TermNum>(T);

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

// TryFrom maximum size

impl TryFrom<i64> for FixedDate {
    type Error = CalendarError;
    fn try_from(date: i64) -> Result<FixedDate, Self::Error> {
        Ok(FixedDate {
            0: date.within_eff()?,
        })
    }
}

impl TryFrom<f64> for FixedMoment {
    type Error = CalendarError;
    fn try_from(t: f64) -> Result<FixedMoment, Self::Error> {
        Ok(FixedMoment { 0: t.within_eff()? })
    }
}

// TryFrom the other's type

impl TryFrom<f64> for FixedDate {
    type Error = CalendarError;
    fn try_from(t: f64) -> Result<FixedDate, Self::Error> {
        Ok(FixedDate::from(FixedMoment::try_from(t)?))
    }
}

impl TryFrom<i64> for FixedMoment {
    type Error = CalendarError;
    fn try_from(t: i64) -> Result<FixedMoment, Self::Error> {
        Ok(FixedMoment::from(FixedDate::try_from(t)?))
    }
}

// From<Fixed> for maximum size

impl From<FixedDate> for i64 {
    fn from(date: FixedDate) -> i64 {
        date.0
    }
}

impl From<FixedMoment> for f64 {
    fn from(t: FixedMoment) -> f64 {
        t.0
    }
}

// From<Fixed> for the other's type

impl From<FixedDate> for f64 {
    fn from(date: FixedDate) -> f64 {
        f64::from(FixedMoment::from(date))
    }
}

impl From<FixedMoment> for i64 {
    fn from(t: FixedMoment) -> i64 {
        i64::from(FixedDate::from(t))
    }
}

// Smaller int types

macro_rules! fixed_from_small_int {
    ($t:ident, $u: ident) => {
        impl From<$t> for $u {
            fn from(date: $t) -> $u {
                $u::try_from(date as i64).expect("Known to be within bounds.")
            }
        }

        impl TryFrom<$u> for $t {
            type Error = CalendarError;
            fn try_from(date: $u) -> Result<$t, Self::Error> {
                Ok(i64::from(date).within_type::<$t>()? as $t)
            }
        }
    };
}

fixed_from_small_int!(i32, FixedDate);
fixed_from_small_int!(i32, FixedMoment);
fixed_from_small_int!(i16, FixedDate);
fixed_from_small_int!(i16, FixedMoment);
fixed_from_small_int!(i8, FixedDate);
fixed_from_small_int!(i8, FixedMoment);

// Sub and Add

impl<T: Sub<Output = T> + TermNum> Sub for Fixed<T> {
    type Output = T;
    fn sub(self, other: Self) -> Self::Output {
        self.0 - other.0
    }
}

impl<T: Add<Output = T> + TermNum> Add<T> for Fixed<T> {
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

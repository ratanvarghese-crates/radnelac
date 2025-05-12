use crate::error::CalendarError;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;

pub trait Fixed {}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct FixedMoment(pub f64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct FixedDate(i64);

impl Fixed for FixedMoment {}
impl Fixed for FixedDate {}

impl From<FixedDate> for f64 {
    fn from(date: FixedDate) -> f64 {
        date.0 as f64
    }
}

impl From<FixedDate> for i64 {
    fn from(date: FixedDate) -> i64 {
        f64::from(date) as i64
    }
}

impl TryFrom<FixedDate> for i32 {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<i32, Self::Error> {
        let t = f64::from(date);
        if t > (i32::MAX as f64) || t < (i32::MIN as f64) {
            Err(CalendarError::OutOfBounds)
        } else {
            Ok(t as i32)
        }
    }
}

impl From<FixedDate> for FixedMoment {
    fn from(date: FixedDate) -> FixedMoment {
        FixedMoment(f64::from(date))
    }
}

impl From<i32> for FixedDate {
    fn from(date: i32) -> FixedDate {
        FixedDate(date as i64)
    }
}

impl TryFrom<i64> for FixedDate {
    type Error = CalendarError;
    fn try_from(date: i64) -> Result<FixedDate, Self::Error> {
        FixedDate::try_from(date as f64)
    }
}

impl TryFrom<f64> for FixedDate {
    type Error = CalendarError;
    fn try_from(t: f64) -> Result<FixedDate, Self::Error> {
        let date = t.floor();
        if date > EFFECTIVE_MAX || date < (EFFECTIVE_MIN) {
            Err(CalendarError::OutOfBounds)
        } else {
            Ok(FixedDate(date as i64))
        }
    }
}

impl TryFrom<FixedMoment> for FixedDate {
    type Error = CalendarError;
    fn try_from(t: FixedMoment) -> Result<FixedDate, Self::Error> {
        Ok(FixedDate::try_from(t.0)?)
    }
}

pub trait Epoch<T: Fixed> {
    fn epoch() -> T;
}

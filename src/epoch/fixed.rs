use crate::error::CalendarError;

pub trait Fixed {}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct FixedMoment(pub f64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct FixedDate(pub i32);

impl Fixed for FixedMoment {}
impl Fixed for FixedDate {}

impl From<FixedDate> for FixedMoment {
    fn from(date: FixedDate) -> FixedMoment {
        FixedMoment(date.0 as f64)
    }
}

impl TryFrom<FixedMoment> for FixedDate {
    type Error = CalendarError;
    fn try_from(t: FixedMoment) -> Result<FixedDate, Self::Error> {
        let date = t.0.floor();
        if date > (i32::MAX as f64) || date < (i32::MIN as f64) {
            Err(CalendarError::OutOfBounds)
        } else {
            Ok(FixedDate(date as i32))
        }
    }
}

pub trait Epoch<T: Fixed> {
    fn epoch() -> T;
}

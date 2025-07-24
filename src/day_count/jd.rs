use crate::common::bound::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;

const JD_EPOCH: f64 = -1721424.5;

/// Represents a Julian Day Number (not to be confused with the Julian Calendar)
///
/// The Julian Day Number is the count of days since noon on January 1, 4713 BC
/// in the proleptic Julian Calendar (November 24, 4714 BCE in the proleptic Gregorian
/// Calendar).
///
/// This is internally a floating point number, where the integer portion represents a
/// particular day and the fractional portion represents a particular time of day.
///
/// Note that equality and ordering operations are subject to limitations similar to
/// equality and ordering operations on a floating point number. Two `JulianDay` values represent
/// the same day or even the same second, but still appear different on the sub-second level.
///
/// Further reading:
/// + [Wikipedia](https://en.m.wikipedia.org/wiki/Julian_day)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDay(f64);

impl CalculatedBounds for JulianDay {}

impl FromFixed for JulianDay {
    fn from_fixed(t: Fixed) -> JulianDay {
        JulianDay(t.get() - JD_EPOCH)
    }
}

impl ToFixed for JulianDay {
    fn to_fixed(self) -> Fixed {
        Fixed::new(JD_EPOCH + self.0)
    }
}

impl Epoch for JulianDay {
    fn epoch() -> Fixed {
        Fixed::new(JD_EPOCH)
    }
}

impl BoundedDayCount<f64> for JulianDay {
    fn new(t: f64) -> JulianDay {
        debug_assert!(JulianDay::in_effective_bounds(t).is_ok());
        JulianDay(t)
    }
    fn get(self) -> f64 {
        self.0
    }
}

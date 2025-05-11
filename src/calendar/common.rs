use crate::epoch::fixed::FixedDate;
use crate::error::CalendarError;
use crate::math::modulus;

use crate::math::EFFECTIVE_MAX;

pub const MAX_YEARS: i32 = (EFFECTIVE_MAX / 365.25) as i32;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum CommonDayOfWeek {
    Sunday = 0,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl CommonDayOfWeek {
    fn on_or_before(self, date: FixedDate) -> Result<FixedDate, CalendarError> {
        let date = f64::from(date);
        let k = (self as i64) as f64;
        FixedDate::try_from(date - (CommonDayOfWeek::try_from(date - k)? as i64) as f64)
    }

    fn on_or_after(self, date: FixedDate) -> Result<FixedDate, CalendarError> {
        let date = FixedDate::try_from(f64::from(date) + 6.0)?;
        self.on_or_before(date)
    }

    fn nearest(self, date: FixedDate) -> Result<FixedDate, CalendarError> {
        let date = FixedDate::try_from(f64::from(date) + 3.0)?;
        self.on_or_before(date)
    }

    fn before(self, date: FixedDate) -> Result<FixedDate, CalendarError> {
        let date = FixedDate::try_from(f64::from(date) - 1.0)?;
        self.on_or_before(date)
    }

    fn after(self, date: FixedDate) -> Result<FixedDate, CalendarError> {
        let date = FixedDate::try_from(f64::from(date) + 7.0)?;
        self.on_or_before(date)
    }
}

impl TryFrom<f64> for CommonDayOfWeek {
    type Error = CalendarError;
    fn try_from(date: f64) -> Result<CommonDayOfWeek, Self::Error> {
        match modulus(date, 7.0)? {
            0.0 => Ok(CommonDayOfWeek::Sunday),
            1.0 => Ok(CommonDayOfWeek::Monday),
            2.0 => Ok(CommonDayOfWeek::Tuesday),
            3.0 => Ok(CommonDayOfWeek::Wednesday),
            4.0 => Ok(CommonDayOfWeek::Thursday),
            5.0 => Ok(CommonDayOfWeek::Friday),
            6.0 => Ok(CommonDayOfWeek::Saturday),
            _ => Err(CalendarError::ImpossibleResult),
        }
    }
}

impl From<FixedDate> for CommonDayOfWeek {
    fn from(date: FixedDate) -> CommonDayOfWeek {
        CommonDayOfWeek::try_from(f64::from(date)).expect("FixedDate will be in range for modulus")
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct CommonDate {
    year: i32,
    month: u8,
    day: u8,
}

impl CommonDate {
    pub fn try_new(year: i32, month: u8, day: u8) -> Result<CommonDate, CalendarError> {
        if year > MAX_YEARS || year < -MAX_YEARS {
            Err(CalendarError::OutOfBounds)
        } else {
            Ok(CommonDate { year, month, day })
        }
    }

    pub fn new(year: i16, month: u8, day: u8) -> CommonDate {
        CommonDate::try_new(year as i32, month, day).expect("Year is guaranteed to be in bounds")
    }

    pub fn get_year(self) -> i32 {
        self.year
    }

    pub fn get_month(self) -> u8 {
        self.month
    }

    pub fn get_day(self) -> u8 {
        self.day
    }
}

pub trait ValidCommonDate {
    fn is_valid(date: CommonDate) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    proptest! {
        #[test]
        fn day_of_week_repeats(x in i32::MIN..(i32::MAX - 7)) {
            let a1 = CommonDayOfWeek::from(FixedDate::from(x));
            let a2 = CommonDayOfWeek::from(FixedDate::from(x + 1));
            let a3 = CommonDayOfWeek::from(FixedDate::from(x + 7));
            assert_ne!(a1, a2);
            assert_eq!(a1, a3);
        }

        #[test]
        fn day_of_week_on_or_before(x1 in (i32::MIN+14)..i32::MAX) {
            let d1 = CommonDayOfWeek::Wednesday.on_or_before(FixedDate::from(x1)).unwrap();
            let d2 = CommonDayOfWeek::Wednesday.on_or_before(d1).unwrap();
            assert_eq!(d1, d2);
            let x2 = i32::try_from(d2).unwrap();
            for i in 1..6 {
                let d3 = CommonDayOfWeek::Wednesday.on_or_before(FixedDate::from(x2 - i)).unwrap();
                assert_ne!(d1, d3);
            }
        }
    }
}

use crate::error::CalendarError;

use crate::math::EFFECTIVE_MAX;

pub const MAX_YEARS: i32 = (EFFECTIVE_MAX / 365.25) as i32;

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

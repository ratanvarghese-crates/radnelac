use crate::common::error::CalendarError;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct CommonDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl CommonDate {
    pub fn new(year: i32, month: u8, day: u8) -> CommonDate {
        CommonDate { year, month, day }
    }
}

pub trait ToCommonDate {
    fn to_common_date(self) -> CommonDate;
}

pub trait TryFromCommonDate: Sized {
    fn try_from_common_date(d: CommonDate) -> Result<Self, CalendarError>;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct OrdinalDate {
    pub year: i32,
    pub day_of_year: u16,
}

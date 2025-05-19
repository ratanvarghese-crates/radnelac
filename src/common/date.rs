use crate::common::bound::EffectiveBound;
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

pub trait ToFromCommonDate: Sized + EffectiveBound {
    fn to_common_date(self) -> CommonDate;
    fn from_common_date_unchecked(d: CommonDate) -> Self;
    fn valid_month_day(d: CommonDate) -> Result<(), CalendarError>;

    fn in_effective_bounds(d: CommonDate) -> bool {
        let min = Self::effective_min().to_common_date();
        let max = Self::effective_max().to_common_date();
        d >= min && d <= max
    }

    fn try_from_common_date(d: CommonDate) -> Result<Self, CalendarError> {
        match Self::valid_month_day(d) {
            Err(e) => Err(e),
            Ok(_) => {
                if Self::in_effective_bounds(d) {
                    Ok(Self::from_common_date_unchecked(d))
                } else {
                    Err(CalendarError::OutOfBounds)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct OrdinalDate {
    pub year: i32,
    pub day_of_year: u16,
}

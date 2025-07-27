use crate::calendar::gregorian::Gregorian;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::GuaranteedMonth;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::Perennial;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::prelude::ToFromOrdinalDate;
use crate::calendar::HasIntercalaryDays;
use crate::calendar::OrdinalDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;
use std::num::NonZero;

/// Represents a month in the Cotsworth Calendar
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum CotsworthMonth {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    Sol,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// Represents a complementary day of the Cotsworth Calendar
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum CotsworthComplementaryDay {
    /// The day that ends every year of the Cotsworth Calendar.
    /// This is also represented as December 29. It is not part of any week.
    YearDay = 1,
    /// The extra day added in leap years of the Cotsworth Calendar.
    /// This is also represented as June 29. It is not part of any week.
    LeapDay,
}

/// Represents a date in the Cotsworth calendar
/// (ie. International Fixed Calendar, Eastman plan, Yearal)
///
/// This calendar was originally designed by Moses B Cotsworth.
/// George Eastman instituted its use within the Eastman Kodak Company from 1928 to 1989.
///
/// ## Year 0
///
/// Year 0 is supported for this calendar.
///
/// ## Further reading
/// + [Wikipedia](https://en.wikipedia.org/wiki/Cotsworth_calendar)
/// + ["Nation's Business" May 1926](https://www.freexenon.com/wp-content/uploads/2018/07/The-Importance-of-Calendar-Reform-to-the-Business-World-George-Eastman.pdf)
///   + "The Importance of Calendar Reform to the Business World"
///   + by George Eastman, President, Eastman Kodak Company

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Cotsworth(CommonDate);

impl ToFromOrdinalDate for Cotsworth {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        Gregorian::valid_ordinal(ord)
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        Gregorian::ordinal_from_fixed(fixed_date)
    }

    fn to_ordinal(self) -> OrdinalDate {
        let approx_m = ((self.0.month as i64) - 1) * 28;
        let offset_m = if self.0.month > 6 && Cotsworth::is_leap(self.0.year) {
            approx_m + 1
        } else {
            approx_m
        };
        let doy = (offset_m as u16) + (self.0.day as u16);
        OrdinalDate {
            year: self.0.year,
            day_of_year: doy,
        }
    }

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        const LEAP_DAY_ORD: u16 = (6 * 28) + 1;
        let result = match (ord.day_of_year, Cotsworth::is_leap(ord.year)) {
            (366, true) => CommonDate::new(ord.year, 13, 29),
            (365, false) => CommonDate::new(ord.year, 13, 29),
            (LEAP_DAY_ORD, true) => CommonDate::new(ord.year, 6, 29),
            (doy, is_leap) => {
                let correction = if doy < LEAP_DAY_ORD || !is_leap { 0 } else { 1 };
                let month = ((((doy - correction) - 1) as i64).div_euclid(28) + 1) as u8;
                let day = ((doy - correction) as i64).adjusted_remainder(28) as u8;
                CommonDate::new(ord.year, month, day)
            }
        };
        Cotsworth(result)
    }
}

impl HasIntercalaryDays<CotsworthComplementaryDay> for Cotsworth {
    fn complementary(self) -> Option<CotsworthComplementaryDay> {
        if self.0.day == 29 && self.0.month == (CotsworthMonth::December as u8) {
            Some(CotsworthComplementaryDay::YearDay)
        } else if self.0.day == 29 && self.0.month == (CotsworthMonth::June as u8) {
            Some(CotsworthComplementaryDay::LeapDay)
        } else {
            None
        }
    }

    fn complementary_count(p_year: i32) -> u8 {
        if Cotsworth::is_leap(p_year) {
            2
        } else {
            1
        }
    }
}

impl Perennial<CotsworthMonth, Weekday> for Cotsworth {
    fn weekday(self) -> Option<Weekday> {
        if self.complementary().is_some() {
            None
        } else {
            Weekday::from_i64(((self.0.day as i64) - 1).modulus(7))
        }
    }

    fn days_per_week() -> u8 {
        7
    }

    fn weeks_per_month() -> u8 {
        4
    }
}

impl HasLeapYears for Cotsworth {
    fn is_leap(c_year: i32) -> bool {
        Gregorian::is_leap(c_year)
    }
}

impl CalculatedBounds for Cotsworth {}

impl Epoch for Cotsworth {
    fn epoch() -> Fixed {
        Gregorian::epoch()
    }
}

impl FromFixed for Cotsworth {
    fn from_fixed(fixed_date: Fixed) -> Cotsworth {
        let ord = Self::ordinal_from_fixed(fixed_date);
        Self::from_ordinal_unchecked(ord)
    }
}

impl ToFixed for Cotsworth {
    fn to_fixed(self) -> Fixed {
        let offset_y = Gregorian::try_year_start(self.0.year)
            .expect("Year known to be valid")
            .to_fixed()
            .get_day_i()
            - 1;
        let ord = self.to_ordinal();
        Fixed::cast_new(offset_y + (ord.day_of_year as i64))
    }
}

impl ToFromCommonDate<CotsworthMonth> for Cotsworth {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        if date.month < 1 || date.month > 13 {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 || date.day > 29 {
            Err(CalendarError::InvalidDay)
        } else if date.day == 29 {
            if date.month == 13 || (Cotsworth::is_leap(date.year) && date.month == 6) {
                Ok(())
            } else {
                Err(CalendarError::InvalidDay)
            }
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        CommonDate::new(year, CotsworthMonth::December as u8, 29)
    }
}

impl Quarter for Cotsworth {
    fn quarter(self) -> NonZero<u8> {
        let m = self.to_common_date().month;
        if m == 13 {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new(((m - 1) / 3) + 1).expect("(m-1)/3 > -1")
        }
    }
}

impl GuaranteedMonth<CotsworthMonth> for Cotsworth {}

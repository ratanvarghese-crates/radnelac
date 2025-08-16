// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::gregorian::Gregorian;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::GuaranteedMonth;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::Perennial;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::prelude::ToFromOrdinalDate;
use crate::calendar::AllowYearZero;
use crate::calendar::CalendarMoment;
use crate::calendar::HasEpagemonae;
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
///
/// ## Introduction
///
/// The Cotsworth calendar (also called the International Fixed Calendar, the Eastman plan, or
/// the Yearal) was originally designed by Moses Bruine Cotsworth. The supposed benefits compared
/// to the Gregorian calendar are that the Cotsworth months all have the same lengths and the
/// Cotsworth months always start on the same day of the week, every year.
///
/// George Eastman instituted the use of the Cotsworth calendar within the Eastman Kodak\
/// Company from 1928 to 1989.
///
/// There was an International Fixed Calendar League advocating for the adoption of the Cotsworth
/// calendar from 1923 to 1937.
///
/// ## Basic structure
///
/// Years are divided into 13 months. All months have 4 weeks of 7 days each. The first day of
/// every month is a Sunday, and the twenty-eighth day of every month is a Saturday.
///
/// The final month, December, has an extra day which is not part of any week - this is Year Day.
///
/// During leap years the sixth month, June, also has an extra day which is not part of any week -
/// this is Leap Day. The Cotsworth calendar follows the Gregorian leap year rule: if a Gregorian
/// year is a leap year, the corresponding Cotsworth year is also a leap year.
///
/// The start of any given Gregorian year is also the start of the corresponding Cotsworth year.
///
/// ## Epoch
///
/// The first year of the Cotsworth calendar is also the first year of the proleptic Gregorian
/// calendar.
///
/// ## Representation and Examples
///
/// ### Months
///
/// The months are represented in this crate as [`CotsworthMonth`].
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_1_1 = CommonDate::new(1902, 1, 1);
/// let a_1_1 = Cotsworth::try_from_common_date(c_1_1).unwrap();
/// assert_eq!(a_1_1.month(), CotsworthMonth::January);
/// ```
///
/// Note that although many month names are shared with [`GregorianMonth`](crate::calendar::GregorianMonth),
/// the months of these two calendar systems are represented by distinct enums. This is because:
///
/// 1. [`CotsworthMonth::Sol`] has no corresponding [`GregorianMonth`](crate::calendar::GregorianMonth)
/// 2. Any [`CotsworthMonth`] after [`CotsworthMonth::Sol`] has a different numeric value than
///    the corresponding [`GregorianMonth`](crate::calendar::GregorianMonth).
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// assert_eq!(CotsworthMonth::June as u8, GregorianMonth::June as u8);
/// assert!(CotsworthMonth::June < CotsworthMonth::Sol && CotsworthMonth::Sol < CotsworthMonth::July);
/// assert_ne!(CotsworthMonth::July as u8, GregorianMonth::July as u8);
///
/// ```
///
/// ### Weekdays
///
/// The days of the Cotsworth week are not always the same as the days of the common week.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
/// use radnelac::day_cycle::*;
///
/// let c_1_1 = CommonDate::new(2025, 1, 1);
/// let a_1_1 = Cotsworth::try_from_common_date(c_1_1).unwrap();
/// assert_eq!(a_1_1.weekday().unwrap(), Weekday::Sunday); //Cotsworth week
/// assert_eq!(a_1_1.convert::<Weekday>(), Weekday::Wednesday); //Common week
/// ```
///
/// ### Year Day and Leap Day
///
/// Year Day and Leap Day can be represented using [`CotsworthComplementaryDay`], or as
/// December 29 and June 29 respectively.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_year_day = CommonDate::new(2028, 13, 29);
/// let a_year_day = Cotsworth::try_from_common_date(c_year_day).unwrap();
/// assert_eq!(a_year_day.month(), CotsworthMonth::December);
/// assert_eq!(a_year_day.epagomenae().unwrap(), CotsworthComplementaryDay::YearDay);
/// assert!(a_year_day.weekday().is_none());
/// let c_leap_day = CommonDate::new(2028, 6, 29);
/// let a_leap_day = Cotsworth::try_from_common_date(c_leap_day).unwrap();
/// assert_eq!(a_leap_day.month(), CotsworthMonth::June);
/// assert_eq!(a_leap_day.epagomenae().unwrap(), CotsworthComplementaryDay::LeapDay);
/// assert!(a_leap_day.weekday().is_none());
///
/// ```
///
/// ## Inconsistencies with Other Implementations
///
/// In other implementations of the Cotsworth calendar, Leap Day and Year Day may be treated as
/// being outside any month. This crate **does not** support that representation - instead
/// Leap Day is treated as June 29 and Year Day is treated as December 29.
///
/// ## Further reading
///
/// + [Wikipedia](https://en.wikipedia.org/wiki/Cotsworth_calendar)
/// + [*The Rational Almanac* by Moses Bruine Cotsworth](https://archive.org/details/rationalalmanact00cotsuoft/mode/2up)
/// + [*The Importance of Calendar Reform to the Business World* by George Eastman](https://www.freexenon.com/wp-content/uploads/2018/07/The-Importance-of-Calendar-Reform-to-the-Business-World-George-Eastman.pdf)

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Cotsworth(CommonDate);

impl AllowYearZero for Cotsworth {}

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

impl HasEpagemonae<CotsworthComplementaryDay> for Cotsworth {
    fn epagomenae(self) -> Option<CotsworthComplementaryDay> {
        if self.0.day == 29 && self.0.month == (CotsworthMonth::December as u8) {
            Some(CotsworthComplementaryDay::YearDay)
        } else if self.0.day == 29 && self.0.month == (CotsworthMonth::June as u8) {
            Some(CotsworthComplementaryDay::LeapDay)
        } else {
            None
        }
    }

    fn epagomenae_count(p_year: i32) -> u8 {
        if Cotsworth::is_leap(p_year) {
            2
        } else {
            1
        }
    }
}

impl Perennial<CotsworthMonth, Weekday> for Cotsworth {
    fn weekday(self) -> Option<Weekday> {
        if self.epagomenae().is_some() {
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
        debug_assert!(Self::valid_ymd(date).is_ok());
        Self(date)
    }

    fn valid_ymd(date: CommonDate) -> Result<(), CalendarError> {
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

    fn month_length(year: i32, month: CotsworthMonth) -> u8 {
        let approx_len = Cotsworth::days_per_week() * Cotsworth::weeks_per_month();
        match (month, Cotsworth::is_leap(year)) {
            (CotsworthMonth::June, true) => approx_len + 1,
            (CotsworthMonth::December, _) => approx_len + 1,
            (_, _) => approx_len,
        }
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

/// Represents a date *and time* in the Cotsworth Calendar
pub type CotsworthMoment = CalendarMoment<Cotsworth>;

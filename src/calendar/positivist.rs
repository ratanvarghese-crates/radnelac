// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Calendier Positiviste Page 52-53
use crate::calendar::gregorian::Gregorian;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::Perennial;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::prelude::ToFromOrdinalDate;
use crate::calendar::AllowYearZero;
use crate::calendar::CalendarMoment;
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
use std::num::NonZero;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const POSITIVIST_YEAR_OFFSET: i32 = 1789 - 1;
const NON_MONTH: u8 = 14;

/// Represents a month of the Positivist Calendar
///
/// The Positivist months are named after famous historical figures.
///
/// Note that the complementary days at the end of the Positivist calendar year have no
/// month and thus are not represented by PositivistMonth. When representing an
/// arbitrary day in the Positivist calendar, use an `Option<PositivistMonth>` for the
/// the month field.
///
/// See page 19 of "Calendier Positiviste" for more details.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PositivistMonth {
    Moses = 1,
    Homer,
    Aristotle,
    Archimedes,
    Caesar,
    SaintPaul,
    Charlemagne,
    Dante,
    Gutenburg,
    Shakespeare,
    Descartes,
    Frederick,
    Bichat,
}

/// Represents a complementary day of the Positivist Calendar
///
/// These are not part of any week or month.
/// See page 8 of "Calendier Positiviste" for more details.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PositivistComplementaryDay {
    /// In leap years of the Positivist calendar, this is the second-last day of the year.
    /// In common years of the Positivist calendar, this is the last day of the year.
    FestivalOfTheDead = 1,
    /// In leap years of the Positivist calendar, this is the last day of the year.
    FestivalOfHolyWomen,
}

/// Represents a date in the Positivist calendar
///
/// ## Further reading
/// + [Positivists.org](http://positivists.org/calendar.html)
/// + ["Calendrier Positiviste" by August Comte](https://gallica.bnf.fr/ark:/12148/bpt6k21868f/f42.planchecontact)
/// + ["The Positivist Calendar" by Henry Edger](https://books.google.ca/books?id=S_BRAAAAMAAJ)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Positivist(CommonDate);

impl AllowYearZero for Positivist {}

impl ToFromOrdinalDate for Positivist {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        let ord_g = OrdinalDate {
            year: ord.year + POSITIVIST_YEAR_OFFSET,
            day_of_year: ord.day_of_year,
        };
        Gregorian::valid_ordinal(ord_g)
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        let ord_g = Gregorian::ordinal_from_fixed(fixed_date);
        OrdinalDate {
            year: ord_g.year - POSITIVIST_YEAR_OFFSET,
            day_of_year: ord_g.day_of_year,
        }
    }

    fn to_ordinal(self) -> OrdinalDate {
        let offset_m = ((self.0.month as i64) - 1) * 28;
        let doy = (offset_m as u16) + (self.0.day as u16);
        OrdinalDate {
            year: self.0.year,
            day_of_year: doy,
        }
    }

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        let year = ord.year;
        let month = (((ord.day_of_year - 1) as i64).div_euclid(28) + 1) as u8;
        let day = (ord.day_of_year as i64).adjusted_remainder(28) as u8;
        debug_assert!(day > 0 && day < 29);
        Positivist(CommonDate::new(year, month, day))
    }
}

impl HasIntercalaryDays<PositivistComplementaryDay> for Positivist {
    // Calendier Positiviste Page 8
    fn complementary(self) -> Option<PositivistComplementaryDay> {
        if self.0.month == NON_MONTH {
            PositivistComplementaryDay::from_u8(self.0.day)
        } else {
            None
        }
    }

    fn complementary_count(p_year: i32) -> u8 {
        if Positivist::is_leap(p_year) {
            2
        } else {
            1
        }
    }
}

impl Perennial<PositivistMonth, Weekday> for Positivist {
    // Calendier Positiviste Page 23-30
    fn weekday(self) -> Option<Weekday> {
        if self.0.month == NON_MONTH {
            None
        } else {
            Weekday::from_i64((self.0.day as i64).modulus(7))
        }
    }

    fn days_per_week() -> u8 {
        7
    }

    fn weeks_per_month() -> u8 {
        4
    }
}

impl HasLeapYears for Positivist {
    // Not sure about the source for this...
    fn is_leap(p_year: i32) -> bool {
        Gregorian::is_leap(POSITIVIST_YEAR_OFFSET + p_year)
    }
}

impl CalculatedBounds for Positivist {}

impl Epoch for Positivist {
    fn epoch() -> Fixed {
        Gregorian::try_year_start(POSITIVIST_YEAR_OFFSET)
            .expect("Year known to be valid")
            .to_fixed()
    }
}

impl FromFixed for Positivist {
    fn from_fixed(date: Fixed) -> Positivist {
        let ord_g = Gregorian::ordinal_from_fixed(date);
        let ord = OrdinalDate {
            year: ord_g.year - POSITIVIST_YEAR_OFFSET,
            day_of_year: ord_g.day_of_year,
        };
        Self::from_ordinal_unchecked(ord)
    }
}

impl ToFixed for Positivist {
    fn to_fixed(self) -> Fixed {
        let y = self.0.year + POSITIVIST_YEAR_OFFSET;
        let offset_y = Gregorian::try_year_start(y)
            .expect("Year known to be valid")
            .to_fixed()
            .get_day_i()
            - 1;
        let doy = self.to_ordinal().day_of_year as i64;
        Fixed::cast_new(offset_y + doy)
    }
}

impl ToFromCommonDate<PositivistMonth> for Positivist {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        if date.month < 1 || date.month > NON_MONTH {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.month < NON_MONTH && date.day > 28 {
            Err(CalendarError::InvalidDay)
        } else if date.month == NON_MONTH && date.day > Positivist::complementary_count(date.year) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        CommonDate::new(year, NON_MONTH, Positivist::complementary_count(year))
    }
}

impl Quarter for Positivist {
    fn quarter(self) -> NonZero<u8> {
        match self.try_month() {
            Some(PositivistMonth::Bichat) | None => NonZero::new(4 as u8).unwrap(),
            Some(m) => NonZero::new((((m as u8) - 1) / 3) + 1).expect("(m-1)/3 > -1"),
        }
    }
}

/// Represents a date *and time* in the Positivist Calendar
pub type PositivistMoment = CalendarMoment<Positivist>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_text() {
        //The Positivist Calendar, page 37
        let dg = Gregorian::try_from_common_date(CommonDate::new(1855, 1, 1)).unwrap();
        let dp = Positivist::try_from_common_date(CommonDate::new(67, 1, 1)).unwrap();
        let fg = dg.to_fixed();
        let fp = dp.to_fixed();
        assert_eq!(fg, fp);
    }
}

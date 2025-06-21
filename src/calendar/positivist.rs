// http://positivists.org/calendar.html
// https://gallica.bnf.fr/ark:/12148/bpt6k21868f/f42.planchecontact
// https://books.google.ca/books?id=S_BRAAAAMAAJ

// Calendier Positiviste Page 52-53
use crate::calendar::gregorian::Gregorian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::CommonDay;
use crate::common::date::CommonYear;
use crate::common::date::HasLeapYears;
use crate::common::date::PerennialWithComplementaryDay;
use crate::common::date::Quarter;
use crate::common::date::ToFromCommonDate;
use crate::common::date::TryMonth;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use std::num::NonZero;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const POSITIVIST_YEAR_OFFSET: i16 = 1789 - 1;

// Calendier Positiviste Page 19
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
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

// Calendier Positiviste Page 8
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum PositivistComplementaryDay {
    FestivalOfTheDead = 1,
    FestivalOfHolyWomen,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Positivist(CommonDate);

impl PerennialWithComplementaryDay<PositivistComplementaryDay, Weekday> for Positivist {
    // Calendier Positiviste Page 8
    fn complementary(self) -> Option<PositivistComplementaryDay> {
        if self.0.month == 14 {
            PositivistComplementaryDay::from_u8(self.0.day)
        } else {
            None
        }
    }

    // Calendier Positiviste Page 23-30
    fn weekday(self) -> Option<Weekday> {
        if self.0.month == 14 {
            None
        } else {
            Weekday::from_i64((self.0.day as i64).modulus(7))
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

impl HasLeapYears for Positivist {
    // Not sure about the source for this...
    fn is_leap(p_year: i32) -> bool {
        Gregorian::is_leap((POSITIVIST_YEAR_OFFSET as i32) + p_year)
    }
}

impl CalculatedBounds for Positivist {}

impl Epoch for Positivist {
    fn epoch() -> Fixed {
        Gregorian::new_year(POSITIVIST_YEAR_OFFSET)
    }
}

impl FromFixed for Positivist {
    fn from_fixed(date: Fixed) -> Positivist {
        let ord = Gregorian::ordinal_from_fixed(date);
        let year = ord.year - (POSITIVIST_YEAR_OFFSET as i32);
        let month = (((ord.day_of_year - 1) as i64).div_euclid(28) + 1) as u8;
        let day = (ord.day_of_year as i64).adjusted_remainder(28) as u8;
        debug_assert!(day > 0 && day < 29);
        Positivist(CommonDate::new(year, month, day))
    }
}

impl ToFixed for Positivist {
    fn to_fixed(self) -> Fixed {
        let y = self.0.year + (POSITIVIST_YEAR_OFFSET as i32);
        let offset_y = Gregorian::try_from_common_date(CommonDate::new(y, 1, 1))
            .expect("month 1, day 1 is always valid for Gregorian")
            .to_fixed()
            .get_day_i()
            - 1;
        let offset_m = ((self.0.month as i64) - 1) * 28;
        Fixed::cast_new(offset_y + offset_m + (self.0.day as i64))
    }
}

impl ToFromCommonDate for Positivist {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        if date.month < 1 || date.month > 14 {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.month < 14 && date.day > 28 {
            Err(CalendarError::InvalidDay)
        } else if date.month == 14 && date.day > Positivist::complementary_count(date.year) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }
}

impl Quarter for Positivist {
    fn quarter(self) -> NonZero<u8> {
        let m = self.to_common_date().month;
        if m == 13 || m == 14 {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new(((m - 1) / 3) + 1).expect("(m-1)/3 > -1")
        }
    }
}

impl CommonYear for Positivist {}
impl TryMonth<PositivistMonth> for Positivist {}
impl CommonDay for Positivist {}

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

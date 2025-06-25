use crate::calendar::gregorian::GregorianMonth;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::CommonDay;
use crate::common::date::GuaranteedMonth;
use crate::common::date::HasLeapYears;
use crate::common::date::Quarter;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::RataDie;
use crate::day_count::ToFixed;
use std::num::NonZero;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

pub type JulianMonth = GregorianMonth;

const JULIAN_EPOCH_RD: i32 = -1;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Julian(CommonDate);

impl Julian {
    pub fn year(self) -> NonZero<i32> {
        NonZero::new(self.0.year).expect("Will not be assigned zero")
    }

    pub fn prior_elapsed_days(year: i32) -> i64 {
        let y = if year < 0 { year + 1 } else { year } as i64;

        let offset_e = Julian::epoch().get_day_i() - 1;
        let offset_y = 365 * (y - 1);
        let offset_leap = (y - 1).div_euclid(4);
        offset_e + offset_y + offset_leap
    }

    pub fn new_year(g_year: NonZero<i16>) -> Fixed {
        Julian(CommonDate {
            year: g_year.get() as i32,
            month: JulianMonth::January as u8,
            day: 1,
        })
        .to_fixed()
    }

    pub fn year_end(g_year: NonZero<i16>) -> Fixed {
        Julian(CommonDate {
            year: g_year.get() as i32,
            month: JulianMonth::December as u8,
            day: 31,
        })
        .to_fixed()
    }
}

impl HasLeapYears for Julian {
    fn is_leap(j_year: i32) -> bool {
        let m4 = j_year.modulus(4);
        if j_year > 0 {
            m4 == 0
        } else {
            m4 == 3
        }
    }
}

impl CalculatedBounds for Julian {}

impl Epoch for Julian {
    fn epoch() -> Fixed {
        RataDie::new(JULIAN_EPOCH_RD as f64).to_fixed()
    }
}

impl FromFixed for Julian {
    fn from_fixed(fixed_date: Fixed) -> Julian {
        let date = fixed_date.get_day_i();
        let epoch = Julian::epoch().get_day_i();
        let approx = ((4 * (date - epoch)) + 1464).div_euclid(1461);
        let year = if approx <= 0 { approx - 1 } else { approx } as i32;
        let year_start = Julian(CommonDate::new(year, 1, 1)).to_fixed().get_day_i();
        let prior_days = date - year_start;
        let march1 = Julian(CommonDate::new(year, 3, 1)).to_fixed().get_day_i();
        let correction = if date < march1 {
            0
        } else if Julian::is_leap(year) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373).div_euclid(367) as u8;
        let month_start = Julian(CommonDate::new(year, month, 1))
            .to_fixed()
            .get_day_i();
        let day = ((date - month_start) as u8) + 1;
        debug_assert!(day > 0);
        Julian(CommonDate { year, month, day })
    }
}

impl ToFixed for Julian {
    fn to_fixed(self) -> Fixed {
        let year = self.0.year;
        let month = self.0.month as i64;
        let day = self.0.day as i64;

        let offset_prior = Julian::prior_elapsed_days(year);
        let offset_m = ((367 * month) - 362).div_euclid(12);
        let offset_x = if month <= 2 {
            0
        } else if Julian::is_leap(year) {
            -1
        } else {
            -2
        };
        let offset_d = day;
        Fixed::cast_new(offset_prior + offset_m + offset_x + offset_d)
    }
}

impl ToFromCommonDate for Julian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = JulianMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > month_opt.unwrap().length(Julian::is_leap(date.year)) {
            Err(CalendarError::InvalidDay)
        } else if date.year == 0 {
            Err(CalendarError::InvalidYear)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        let m = JulianMonth::December;
        CommonDate::new(year, m as u8, m.length(Julian::is_leap(year)))
    }
}

impl Quarter for Julian {
    fn quarter(self) -> NonZero<u8> {
        NonZero::new(((self.to_common_date().month - 1) / 3) + 1).expect("(m-1)/3 > -1")
    }
}

impl GuaranteedMonth<JulianMonth> for Julian {}
impl CommonDay for Julian {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::gregorian::Gregorian;
    use proptest::prop_assume;
    use proptest::proptest;

    #[test]
    fn julian_gregorian_conversion() {
        let gap_list = [
            // Official dates of adopting the Gregorian calendar
            // Governments would declare that certain days would be skipped
            // The table below lists Julian dates and the Gregorian dates of the next day.
            // https://en.wikipedia.org/wiki/Adoption_of_the_Gregorian_calendar
            // https://en.wikipedia.org/wiki/List_of_adoption_dates_of_the_Gregorian_calendar_by_country
            (CommonDate::new(1582, 10, 4), CommonDate::new(1582, 10, 15)), //Papal States, Spain, Portugal
            (CommonDate::new(1582, 12, 9), CommonDate::new(1582, 12, 20)), //France
            (CommonDate::new(1582, 12, 14), CommonDate::new(1582, 12, 25)), //"Flanders" (Belgium), Netherlands
            (CommonDate::new(1582, 12, 20), CommonDate::new(1582, 12, 31)), //"Southern Netherlands" (Belgium), Luxembourg
            (CommonDate::new(1582, 12, 31), CommonDate::new(1583, 1, 11)),  //"Aachen" (Germany)
            (CommonDate::new(1583, 1, 1), CommonDate::new(1583, 1, 12)), //"Holland" (Netherlands)
            (CommonDate::new(1583, 2, 10), CommonDate::new(1583, 2, 21)), //"Salzburg" (Austria), "Liege" (Belgium)
            (CommonDate::new(1583, 2, 13), CommonDate::new(1583, 2, 24)), //"Kaufbeuren" (Germany)
            (CommonDate::new(1583, 2, 14), CommonDate::new(1583, 2, 25)), //"Ellwangen" (Germany)
            (CommonDate::new(1583, 3, 1), CommonDate::new(1583, 3, 12)), //"Groningen" (Netherlands)
            (CommonDate::new(1583, 10, 4), CommonDate::new(1583, 10, 15)), //"Tyrol" (Austria)
            (CommonDate::new(1583, 10, 5), CommonDate::new(1583, 10, 16)), //"Bavaria" (Germany)
            (CommonDate::new(1583, 10, 13), CommonDate::new(1583, 10, 24)), //"Austrian Upper Alsace" (France)
            (CommonDate::new(1583, 10, 20), CommonDate::new(1583, 10, 31)), //"Lower Austria" (Austria)
            (CommonDate::new(1583, 11, 2), CommonDate::new(1583, 11, 13)),  //"Cologne" (Germany)
            (CommonDate::new(1583, 11, 11), CommonDate::new(1583, 11, 22)), //"Mainz" (Germany)
            (CommonDate::new(1632, 12, 14), CommonDate::new(1632, 12, 25)), //"Hildesheim" (Germany)
            (CommonDate::new(1700, 2, 18), CommonDate::new(1700, 3, 1)), //"Denmark-Norway" (Denmark, Norway)
            (CommonDate::new(1753, 2, 17), CommonDate::new(1753, 3, 1)), //Sweden (partial?)
            (CommonDate::new(1752, 9, 2), CommonDate::new(1752, 9, 14)), //British Empire (United Kingdom, Ireland, Canada, United States)
            (CommonDate::new(1753, 2, 17), CommonDate::new(1753, 3, 1)), //Sweden
            (CommonDate::new(1912, 11, 14), CommonDate::new(1912, 11, 28)), //Albania
            (CommonDate::new(1916, 3, 31), CommonDate::new(1916, 4, 14)), //Bulgaria
            (CommonDate::new(1918, 1, 31), CommonDate::new(1918, 2, 14)), //Soviet Union (Russia, etc.)
            (CommonDate::new(1918, 2, 15), CommonDate::new(1918, 3, 1)),  //Estonia, Ukraine
            (CommonDate::new(1918, 4, 17), CommonDate::new(1918, 5, 1)), //"Transcaucasian Democratic Federative Republic"
            (CommonDate::new(1919, 1, 14), CommonDate::new(1919, 1, 28)), //Yugoslavia
            (CommonDate::new(1919, 3, 31), CommonDate::new(1919, 4, 14)), //Romania
            (CommonDate::new(1923, 2, 15), CommonDate::new(1923, 3, 1)), //Greece
        ];

        for pair in gap_list {
            let dj = Julian::try_from_common_date(pair.0).unwrap().to_fixed();
            let dg = Gregorian::try_from_common_date(pair.1).unwrap().to_fixed();
            assert_eq!(dj.get_day_i() + 1, dg.get_day_i());
        }
    }

    #[test]
    fn cross_epoch() {
        let new_years_eve = Julian::year_end(NonZero::new(-1).unwrap());
        let new_years_day = Julian::new_year(NonZero::new(1).unwrap());
        assert_eq!(new_years_day.get_day_i(), new_years_eve.get_day_i() + 1);
    }
}

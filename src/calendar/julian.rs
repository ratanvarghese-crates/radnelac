use crate::calendar::common::CommonDate;
use crate::calendar::common::ValidCommonDate;
use crate::calendar::gregorian::GregorianMonth;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::error::CalendarError;
use crate::math::TermNum;
use std::num::NonZero;

pub type JulianMonth = GregorianMonth;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct JulianDate(CommonDate);

impl JulianDate {
    fn is_leap(g_year: i32) -> bool {
        let m4 = g_year.modulus(4);
        if g_year > 0 {
            m4 == 0
        } else {
            m4 == 3
        }
    }

    fn new_year(g_year: NonZero<i16>) -> FixedDate {
        FixedDate::from(JulianDate(CommonDate::new(
            g_year.get(),
            JulianMonth::January as u8,
            1,
        )))
    }

    fn year_end(g_year: NonZero<i16>) -> FixedDate {
        FixedDate::from(JulianDate(CommonDate::new(
            g_year.get(),
            JulianMonth::December as u8,
            31,
        )))
    }
}

impl Epoch for JulianDate {
    fn epoch() -> FixedDate {
        FixedDate::try_from(-1).expect("Epoch known to be within bounds.")
    }
}

impl ValidCommonDate for JulianDate {
    fn is_valid(date: CommonDate) -> bool {
        let month = date.get_month();
        let day = date.get_day();
        let year = date.get_year();
        if year == 0 {
            return false;
        }

        match JulianMonth::try_from(month) {
            Err(_) => false,
            Ok(month) => match month {
                JulianMonth::January => day >= 1 && day <= 31,
                JulianMonth::February => {
                    if JulianDate::is_leap(year) {
                        day >= 1 && day <= 29
                    } else {
                        day >= 1 && day <= 28
                    }
                }
                JulianMonth::March => day >= 1 && day <= 31,
                JulianMonth::April => day >= 1 && day <= 30,
                JulianMonth::May => day >= 1 && day <= 31,
                JulianMonth::June => day >= 1 && day <= 30,
                JulianMonth::July => day >= 1 && day <= 31,
                JulianMonth::August => day >= 1 && day <= 31,
                JulianMonth::September => day >= 1 && day <= 30,
                JulianMonth::October => day >= 1 && day <= 31,
                JulianMonth::November => day >= 1 && day <= 30,
                JulianMonth::December => day >= 1 && day <= 31,
            },
        }
    }
}

impl From<JulianDate> for CommonDate {
    fn from(date: JulianDate) -> CommonDate {
        return date.0;
    }
}

impl TryFrom<CommonDate> for JulianDate {
    type Error = CalendarError;
    fn try_from(date: CommonDate) -> Result<JulianDate, CalendarError> {
        if JulianDate::is_valid(date) {
            Ok(JulianDate(date))
        } else {
            Err(CalendarError::OutOfBounds)
        }
    }
}

impl From<JulianDate> for FixedDate {
    fn from(date: JulianDate) -> FixedDate {
        let year = date.0.get_year();
        let month = date.0.get_month() as i64;
        let day = date.0.get_day() as i64;

        let y = if year < 0 { year + 1 } else { year } as i64;

        let offset_e = JulianDate::epoch() - FixedDate::from(1 as i32);
        let offset_y = 365 * (y - 1);
        let offset_leap = (y - 1).div_euclid(4);
        let offset_m = ((367 * month) - 362).div_euclid(12);
        let offset_x = if month <= 2 {
            0
        } else if JulianDate::is_leap(year) {
            -1
        } else {
            -2
        };
        let offset_d = day;
        let result = offset_e + offset_y + offset_leap + offset_m + offset_x + offset_d;
        FixedDate::try_from(result as i64).expect("CommonDate enforces year limits")
    }
}

impl TryFrom<FixedDate> for JulianDate {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<JulianDate, Self::Error> {
        let e_diff = date - JulianDate::epoch();
        let approx = ((4 * e_diff) + 1464).div_euclid(1461);
        let year = if approx <= 0 { approx - 1 } else { approx } as i32;
        let year_start = FixedDate::from(JulianDate(CommonDate::try_new(
            year,
            JulianMonth::January as u8,
            1,
        )?));
        let prior_days = date - year_start;
        let march1 = FixedDate::from(JulianDate(CommonDate::try_new(
            year,
            JulianMonth::March as u8,
            1,
        )?));
        let correction = if date < march1 {
            0
        } else if JulianDate::is_leap(year) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373).div_euclid(367);
        let m_diff = date - FixedDate::from(JulianDate(CommonDate::try_new(year, month as u8, 1)?));
        let day = m_diff + 1;
        Ok(JulianDate(CommonDate::try_new(
            year as i32,
            month as u8,
            day as u8,
        )?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::common::MAX_YEARS;
    use crate::calendar::gregorian::GregorianDate;
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
            (CommonDate::new(1918, 1, 31), CommonDate::new(1918, 2, 14)), //Soviet Union
            (CommonDate::new(1918, 2, 15), CommonDate::new(1918, 3, 1)), //Estonia, Ukraine
            (CommonDate::new(1918, 4, 17), CommonDate::new(1918, 5, 1)), //"Transcaucasian Democratic Federative Republic"
            (CommonDate::new(1919, 1, 14), CommonDate::new(1919, 1, 28)), //Yugoslavia
            (CommonDate::new(1919, 3, 31), CommonDate::new(1919, 4, 14)), //Romania
            (CommonDate::new(1923, 2, 15), CommonDate::new(1923, 3, 1)), //Greece
        ];

        for pair in gap_list {
            let dj = FixedDate::from(JulianDate::try_from(pair.0).unwrap());
            let dg = FixedDate::from(GregorianDate::try_from(pair.1).unwrap());
            assert_eq!(i64::from(dj) + 1, i64::from(dg));
        }
    }

    proptest! {
        #[test]
        fn julian_roundtrip(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..28) {
            let d = CommonDate::try_new(year, month as u8, day as u8).unwrap();
            let e0 = JulianDate::try_from(d).unwrap();
            let t = FixedDate::from(e0);
            let e1 = JulianDate::try_from(t).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn julian_year_ends(year in i16::MIN..i16::MAX) {
            prop_assume!(year != 0);
            let new_years_eve = JulianDate::year_end(NonZero::new(year).unwrap());
            let next_year = if year == -1 { 1 } else { year + 1 };
            let new_years_day = JulianDate::new_year(NonZero::new(next_year).unwrap());
            assert_eq!(i64::from(new_years_day), i64::from(new_years_eve) + 1);
        }
    }
}

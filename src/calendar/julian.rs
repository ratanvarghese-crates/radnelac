use crate::calendar::common::CommonDate;
use crate::calendar::common::ValidCommonDate;
use crate::calendar::gregorian::GregorianMonth;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::error::CalendarError;
use crate::math::TermNum;

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

    fn new_year(g_year: i16) -> FixedDate {
        FixedDate::from(JulianDate(CommonDate::new(
            g_year,
            JulianMonth::January as u8,
            1,
        )))
    }

    fn year_end(g_year: i16) -> FixedDate {
        FixedDate::from(JulianDate(CommonDate::new(
            g_year,
            JulianMonth::December as u8,
            31,
        )))
    }
}

impl Epoch<FixedDate> for JulianDate {
    fn epoch() -> FixedDate {
        FixedDate::from(-1)
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

        let offset_e = i64::from(JulianDate::epoch()) - 1;
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
        let e_diff = i64::from(date) - i64::from(JulianDate::epoch());
        let approx = ((4 * e_diff) + 1464).div_euclid(1461);
        let year = if approx <= 0 { approx - 1 } else { approx } as i32;
        let year_start = FixedDate::from(JulianDate(CommonDate::try_new(
            year,
            JulianMonth::January as u8,
            1,
        )?));
        let prior_days = i64::from(date) - i64::from(year_start);
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
        let m_diff = i64::from(date)
            - i64::from(FixedDate::from(JulianDate(CommonDate::try_new(
                year,
                month as u8,
                1,
            )?)));
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
    use proptest::proptest;

    #[test]
    fn julian_gregorian_conversion() {
        let gap_list = [
            // Official dates of adopting the Gregorian calendar
            // Governments would declare that certain days would be skipped
            // The table below lists Julian dates and the Gregorian dates of the next day.
            // https://en.wikipedia.org/wiki/Adoption_of_the_Gregorian_calendar
            (CommonDate::new(1582, 10, 4), CommonDate::new(1582, 10, 15)), // Papal States, Spain
            (CommonDate::new(1582, 12, 9), CommonDate::new(1582, 12, 20)), // France
            (CommonDate::new(1700, 2, 18), CommonDate::new(1700, 3, 1)),   //Denmark-Norway
            (CommonDate::new(1753, 2, 17), CommonDate::new(1753, 3, 1)),   //Sweden (partial?)
            (CommonDate::new(1752, 9, 2), CommonDate::new(1752, 9, 14)),   //British Empire
            (CommonDate::new(1916, 3, 31), CommonDate::new(1916, 4, 14)),  //Bulgaria
            (CommonDate::new(1918, 1, 31), CommonDate::new(1918, 2, 14)),  //Soviet Union
            (CommonDate::new(1919, 3, 31), CommonDate::new(1919, 4, 14)),  //Romania
            (CommonDate::new(1923, 2, 15), CommonDate::new(1923, 3, 1)),   //Greece
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
            let new_years_eve = JulianDate::year_end(year);
            let new_years_day = JulianDate::new_year(year + 1);
            assert_eq!(i64::from(new_years_day), i64::from(new_years_eve) + 1);
        }
    }
}

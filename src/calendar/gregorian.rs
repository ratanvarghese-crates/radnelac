use crate::calendar::common::CommonDate;
use crate::calendar::common::ValidCommonDate;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;
use crate::epoch::rd::RataDie;
use crate::error::CalendarError;
use crate::math::TermNum;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum GregorianMonth {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl TryFrom<u8> for GregorianMonth {
    type Error = CalendarError;
    fn try_from(m: u8) -> Result<GregorianMonth, CalendarError> {
        match m {
            1 => Ok(GregorianMonth::January),
            2 => Ok(GregorianMonth::February),
            3 => Ok(GregorianMonth::March),
            4 => Ok(GregorianMonth::April),
            5 => Ok(GregorianMonth::May),
            6 => Ok(GregorianMonth::June),
            7 => Ok(GregorianMonth::July),
            8 => Ok(GregorianMonth::August),
            9 => Ok(GregorianMonth::September),
            10 => Ok(GregorianMonth::October),
            11 => Ok(GregorianMonth::November),
            12 => Ok(GregorianMonth::December),
            _ => Err(CalendarError::OutOfBounds),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Gregorian(CommonDate);

impl Gregorian {
    pub fn get_month(self) -> GregorianMonth {
        GregorianMonth::try_from(self.0.get_month()).expect("Won't allow setting invalid field")
    }
}

impl Epoch for Gregorian {
    fn epoch() -> FixedDate {
        FixedDate::from(FixedMoment::from(RataDie::from(1)))
    }
}

impl ValidCommonDate for Gregorian {
    fn is_valid(date: CommonDate) -> bool {
        let month = date.get_month();
        let day = date.get_day();
        let year = date.get_year();
        match GregorianMonth::try_from(month) {
            Err(_) => false,
            Ok(month) => match month {
                GregorianMonth::January => day >= 1 && day <= 31,
                GregorianMonth::February => {
                    if Gregorian::is_leap(year) {
                        day >= 1 && day <= 29
                    } else {
                        day >= 1 && day <= 28
                    }
                }
                GregorianMonth::March => day >= 1 && day <= 31,
                GregorianMonth::April => day >= 1 && day <= 30,
                GregorianMonth::May => day >= 1 && day <= 31,
                GregorianMonth::June => day >= 1 && day <= 30,
                GregorianMonth::July => day >= 1 && day <= 31,
                GregorianMonth::August => day >= 1 && day <= 31,
                GregorianMonth::September => day >= 1 && day <= 30,
                GregorianMonth::October => day >= 1 && day <= 31,
                GregorianMonth::November => day >= 1 && day <= 30,
                GregorianMonth::December => day >= 1 && day <= 31,
            },
        }
    }
}

impl From<Gregorian> for CommonDate {
    fn from(date: Gregorian) -> CommonDate {
        return date.0;
    }
}

impl TryFrom<CommonDate> for Gregorian {
    type Error = CalendarError;
    fn try_from(date: CommonDate) -> Result<Gregorian, CalendarError> {
        if Gregorian::is_valid(date) {
            Ok(Gregorian(date))
        } else {
            Err(CalendarError::OutOfBounds)
        }
    }
}

impl Gregorian {
    fn is_leap(g_year: i32) -> bool {
        let m4 = g_year.modulus(4);
        let m400 = g_year.modulus(400);
        m4 == 0 && m400 != 100 && m400 != 200 && m400 != 300
    }

    fn new_year(g_year: i16) -> FixedDate {
        FixedDate::from(Gregorian(CommonDate::new(
            g_year,
            GregorianMonth::January as u8,
            1,
        )))
    }

    fn year_end(g_year: i16) -> FixedDate {
        FixedDate::from(Gregorian(CommonDate::new(
            g_year,
            GregorianMonth::December as u8,
            31,
        )))
    }

    fn year_and_ord_day_from_fixed(date: FixedDate) -> (i32, u16) {
        let d0 = date - Gregorian::epoch();
        let n400 = d0.div_euclid((400 * 365) + 100 - 3);
        let d1 = d0.modulus((400 * 365) + 100 - 3);
        let n100 = d1.div_euclid((365 * 100) + 25 - 1);
        let d2 = d1.modulus((365 * 100) + 25 - 1);
        let n4 = d2.div_euclid(365 * 4 + 1);
        let d3 = d2.modulus(365 * 4 + 1);
        let n1 = d3.div_euclid(365);
        let year = (400 * n400) + (100 * n100) + (4 * n4) + n1;
        if n100 == 4 || n1 == 4 {
            (year as i32, 366)
        } else {
            ((year + 1) as i32, (d3.modulus(365) + 1) as u16)
        }
    }
}

impl From<Gregorian> for FixedDate {
    fn from(date: Gregorian) -> FixedDate {
        let year = date.0.get_year() as i64;
        let month = date.0.get_month() as i64;
        let day = date.0.get_day() as i64;

        let offset_e = Gregorian::epoch() - FixedDate::from(1 as i32);
        let offset_y = 365 * (year - 1);
        let offset_leap =
            (year - 1).div_euclid(4) - (year - 1).div_euclid(100) + (year - 1).div_euclid(400);
        let offset_m = ((367 * month) - 362).div_euclid(12);
        let offset_x = if month <= 2 {
            0
        } else if Gregorian::is_leap(year as i32) {
            -1
        } else {
            -2
        };
        let offset_d = day;

        let result = offset_e + offset_y + offset_leap + offset_m + offset_x + offset_d;
        FixedDate::try_from(result as i64).expect("CommonDate enforces year limits")
    }
}

impl TryFrom<FixedDate> for Gregorian {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<Gregorian, Self::Error> {
        let (year, ord_day) = Gregorian::year_and_ord_day_from_fixed(date);

        let prior_days: i32 = (ord_day as i32) - 1; //Modification
        let march1 = FixedDate::from(Gregorian(CommonDate::try_new(
            year,
            GregorianMonth::March as u8,
            1,
        )?));
        let correction: i32 = if date < march1 {
            0
        } else if Gregorian::is_leap(year) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373).div_euclid(367);
        let day = date - FixedDate::from(Gregorian(CommonDate::try_new(year, month as u8, 1)?)) + 1;
        Ok(Gregorian(CommonDate::try_new(
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
    use proptest::proptest;

    #[test]
    fn gregorian_notable_days() {
        let dlist = [
            //Calendrical Calculations Table 1.2
            (CommonDate::new(-4713, 11, 24), -1721425), //Julian Day epoch
            (CommonDate::new(-3760, 9, 7), -1373427),   //Hebrew epoch
            (CommonDate::new(-3113, 8, 11), -1137142),  //Mayan epoch
            (CommonDate::new(-3101, 1, 23), -1132959),  //Hindu epoch (Kali Yuga)
            (CommonDate::new(-2636, 2, 15), -963099),   //Chinese epoch
            //(CommonDate::new(-1638, 3, 3), -598573),    //Samaritan epoch ... is it correct?
            (CommonDate::new(-746, 2, 18), -272787), //Egyptian epoch (Nabonassar era)
            (CommonDate::new(-310, 3, 29), -113502), //Babylonian epoch???????
            (CommonDate::new(-127, 12, 7), -46410),  //Tibetan epoch
            (CommonDate::new(0, 12, 30), -1),        // Julian calendar epoch
            (CommonDate::new(1, 1, 1), 1),           //Gregorian/ISO/Rata Die epoch
            (CommonDate::new(1, 2, 6), 37),          //Akan epoch
            (CommonDate::new(8, 8, 27), 2796),       //Ethiopic epoch
            (CommonDate::new(284, 8, 29), 103605),   //Coptic epoch
            (CommonDate::new(552, 7, 13), 201443),   //Armenian epoch
            (CommonDate::new(622, 3, 22), 226896),   //Persian epoch
            (CommonDate::new(622, 7, 19), 227015),   //Islamic epoch
            (CommonDate::new(632, 6, 19), 230638),   //Zoroastrian epoch?????
            (CommonDate::new(1792, 9, 22), 654415),  //French Revolutionary epoch
            (CommonDate::new(1844, 3, 21), 673222),  //Bahai epoch
            (CommonDate::new(1858, 11, 17), 678576), //Modified Julian Day epoch
            (CommonDate::new(1970, 1, 1), 719163),   //Unix epoch
            //Days which can be calculated by hand, or are at least easy to reason about
            (CommonDate::new(1, 1, 2), 2),
            (CommonDate::new(1, 1, 31), 31),
            (CommonDate::new(400, 12, 31), 146097),
            (CommonDate::new(800, 12, 31), 146097 * 2),
            (CommonDate::new(1200, 12, 31), 146097 * 3),
            (CommonDate::new(1600, 12, 31), 146097 * 4),
            (CommonDate::new(2000, 12, 31), 146097 * 5),
            (CommonDate::new(2003, 12, 31), (146097 * 5) + (365 * 3)),
        ];

        for pair in dlist {
            let d = FixedDate::from(Gregorian::try_from(pair.0).unwrap());
            assert_eq!(i64::from(d), pair.1);
        }
    }

    proptest! {
        #[test]
        fn gregorian_roundtrip(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..28) {
            let d = CommonDate::try_new(year, month as u8, day as u8).unwrap();
            let e0 = Gregorian::try_from(d).unwrap();
            let t = FixedDate::from(e0);
            let e1 = Gregorian::try_from(t).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn gregorian_year_ends(year in i16::MIN..i16::MAX) {
            let new_years_eve = Gregorian::year_end(year);
            let new_years_day = Gregorian::new_year(year + 1);
            assert_eq!(i64::from(new_years_day), i64::from(new_years_eve) + 1);
        }
    }
}

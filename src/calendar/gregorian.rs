use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::CommonDay;
use crate::common::date::CommonYear;
use crate::common::date::GuaranteedMonth;
use crate::common::date::HasLeapYears;
use crate::common::date::OrdinalDate;
use crate::common::date::Quarter;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::date::CommonWeekOfYear;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::RataDie;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use std::num::NonZero;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const GREGORIAN_EPOCH_RD: i32 = 1;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
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

impl GregorianMonth {
    pub fn length(self, leap: bool) -> u8 {
        match self {
            GregorianMonth::January => 31,
            GregorianMonth::February => {
                if leap {
                    29
                } else {
                    28
                }
            }
            GregorianMonth::March => 31,
            GregorianMonth::April => 30,
            GregorianMonth::May => 31,
            GregorianMonth::June => 30,
            GregorianMonth::July => 31,
            GregorianMonth::August => 31,
            GregorianMonth::September => 30,
            GregorianMonth::October => 31,
            GregorianMonth::November => 30,
            GregorianMonth::December => 31,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Gregorian(CommonDate);

impl Gregorian {
    pub fn month(self) -> GregorianMonth {
        GregorianMonth::from_u8(self.0.month).expect("Will not allow setting invalid value.")
    }

    pub fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        let date = fixed_date.get_day_i();
        let epoch = Gregorian::epoch().get_day_i();
        let d0 = date - epoch;
        let n400 = d0.div_euclid((400 * 365) + 100 - 3);
        let d1 = d0.modulus((400 * 365) + 100 - 3);
        let n100 = d1.div_euclid((365 * 100) + 25 - 1);
        let d2 = d1.modulus((365 * 100) + 25 - 1);
        let n4 = d2.div_euclid(365 * 4 + 1);
        let d3 = d2.modulus(365 * 4 + 1);
        let n1 = d3.div_euclid(365);
        let year = (400 * n400) + (100 * n100) + (4 * n4) + n1;
        if n100 == 4 || n1 == 4 {
            OrdinalDate {
                year: year as i32,
                day_of_year: 366,
            }
        } else {
            OrdinalDate {
                year: (year + 1) as i32,
                day_of_year: (d3.modulus(365) + 1) as u16,
            }
        }
    }

    pub fn to_ordinal(self) -> OrdinalDate {
        let month = self.0.month as i16;
        let day = self.0.day as i16;
        let correction = if month > (GregorianMonth::February as i16) {
            if Gregorian::is_leap(self.0.year) {
                -1
            } else {
                -2
            }
        } else {
            0
        };
        let ordinal_day = ((367 * month) - 362).div_euclid(12) + day + correction;
        OrdinalDate {
            year: self.0.year,
            day_of_year: ordinal_day as u16,
        }
    }

    pub fn prior_elapsed_days(year: i32) -> i64 {
        let year = year as i64;
        let offset_e = Gregorian::epoch().get_day_i() - 1;
        let offset_y = 365 * (year - 1);
        let offset_leap =
            (year - 1).div_euclid(4) - (year - 1).div_euclid(100) + (year - 1).div_euclid(400);
        offset_e + offset_y + offset_leap
    }

    //Arguments swapped from the original
    pub fn nth_kday(self, nz: NonZero<i16>, k: Weekday) -> Fixed {
        let x = self.to_fixed().get_day_i();
        let n = nz.get();
        let result = if n > 0 {
            k.unchecked_before(x) + (7 * n as i64)
        } else {
            k.unchecked_after(x) + (7 * n as i64)
        };

        Fixed::cast_new(result)
    }
}

impl HasLeapYears for Gregorian {
    fn is_leap(g_year: i32) -> bool {
        let m4 = g_year.modulus(4);
        let m400 = g_year.modulus(400);
        m4 == 0 && m400 != 100 && m400 != 200 && m400 != 300
    }
}

impl CalculatedBounds for Gregorian {}

impl Epoch for Gregorian {
    fn epoch() -> Fixed {
        RataDie::new(GREGORIAN_EPOCH_RD as f64).to_fixed()
    }
}

impl FromFixed for Gregorian {
    fn from_fixed(date: Fixed) -> Gregorian {
        let ord = Gregorian::ordinal_from_fixed(date);
        let year = ord.year;
        let prior_days: i32 = (ord.day_of_year as i32) - 1; //Modification
        let march1 = Gregorian(CommonDate::new(year, 3, 1)).to_fixed();
        let correction: i32 = if date < march1 {
            0
        } else if Gregorian::is_leap(year) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373).div_euclid(367) as u8;
        let month_start = Gregorian(CommonDate::new(year, month, 1)).to_fixed();
        let day = ((date.get_day_i() - month_start.get_day_i()) as u8) + 1;
        debug_assert!(day > 0);
        Gregorian(CommonDate { year, month, day })
    }
}

impl ToFixed for Gregorian {
    fn to_fixed(self) -> Fixed {
        let month = self.0.month as i64;
        let day = self.0.day as i64;

        let offset_prior = Gregorian::prior_elapsed_days(self.0.year);
        let offset_m = ((367 * month) - 362).div_euclid(12);
        let offset_x = if month <= 2 {
            0
        } else if Gregorian::is_leap(self.0.year) {
            -1
        } else {
            -2
        };
        let offset_d = day;
        Fixed::cast_new(offset_prior + offset_m + offset_x + offset_d)
    }
}

impl ToFromCommonDate for Gregorian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = GregorianMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > month_opt.unwrap().length(Gregorian::is_leap(date.year)) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        let m = GregorianMonth::December;
        CommonDate::new(year, m as u8, m.length(Gregorian::is_leap(year)))
    }
}

impl Quarter for Gregorian {
    fn quarter(self) -> NonZero<u8> {
        NonZero::new(((self.to_common_date().month - 1) / 3) + 1).expect("(m-1)/3 > -1")
    }
}

impl CommonYear for Gregorian {}
impl GuaranteedMonth<GregorianMonth> for Gregorian {}
impl CommonDay for Gregorian {}
impl CommonWeekOfYear for Gregorian {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use crate::day_cycle::Weekday;
    use proptest::proptest;
    use std::num::NonZero;

    #[test]
    fn us_canada_labor_day() {
        let lbd = Gregorian::try_from_common_date(CommonDate {
            year: 2024,
            month: 9,
            day: 2,
        })
        .unwrap();
        let start = Gregorian::try_from_common_date(CommonDate {
            year: 2024,
            month: 9,
            day: 1,
        })
        .unwrap();
        let finish = start.nth_kday(NonZero::new(1).unwrap(), Weekday::Monday);
        assert_eq!(lbd, Gregorian::from_fixed(finish));
    }

    #[test]
    fn us_memorial_day() {
        let mmd = Gregorian::try_from_common_date(CommonDate::new(2024, 5, 27)).unwrap();
        let start = Gregorian::try_from_common_date(CommonDate::new(2024, 6, 1)).unwrap();
        let finish = start.nth_kday(NonZero::new(-1).unwrap(), Weekday::Monday);
        assert_eq!(mmd, Gregorian::from_fixed(finish));
    }

    #[test]
    fn prior_elapsed_days() {
        // https://kalendis.free.nf/Symmetry454-Arithmetic.pdf
        let count = Gregorian::prior_elapsed_days(2009);
        assert_eq!(count, 733407);
    }

    #[test]
    fn ordinal_from_common() {
        // https://kalendis.free.nf/Symmetry454-Arithmetic.pdf
        let g = Gregorian::try_from_common_date(CommonDate::new(2009, 7, 14)).unwrap();
        let ord = g.to_ordinal();
        assert_eq!(ord.day_of_year, 195);
    }

    #[test]
    fn notable_days() {
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
            let d = Gregorian::try_from_common_date(pair.0).unwrap().to_fixed();
            assert_eq!(d.get_day_i(), pair.1);
        }
    }

    proptest! {
        #[test]
        fn cycle_146097(t in FIXED_MIN..(FIXED_MAX-146097.0), w in 1..55) {
            let f_start = Fixed::new(t);
            let f_end = Fixed::new(t + 146097.0);
            let g_start = Gregorian::from_fixed(f_start);
            let g_end = Gregorian::from_fixed(f_end);
            assert_eq!(g_start.year() + 400, g_end.year());
            assert_eq!(g_start.month(), g_end.month());
            assert_eq!(g_start.day(), g_start.day());

            let w = NonZero::new(w as i16).unwrap();
            let start_sum_kday = Fixed::new(g_start.nth_kday(w, Weekday::Sunday).get() + 146097.0);
            let end_kday = g_end.nth_kday(w, Weekday::Sunday);
            assert_eq!(start_sum_kday, end_kday);
        }
    }
}

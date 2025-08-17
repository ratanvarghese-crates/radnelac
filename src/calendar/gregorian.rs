// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::CommonWeekOfYear;
use crate::calendar::prelude::GuaranteedMonth;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::OrdinalDate;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::AllowYearZero;
use crate::calendar::CalendarMoment;
use crate::calendar::ToFromOrdinalDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::RataDie;
use crate::day_count::ToFixed;
use std::num::NonZero;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

//LISTING 2.3 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
const GREGORIAN_EPOCH_RD: i32 = 1;

/// Represents a month in the Gregorian calendar
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum GregorianMonth {
    //LISTING 2.4-2.15 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
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

/// Represents a date in the proleptic Gregorian calendar
///
/// ## Introduction
///
/// The Gregorian calendar is the calendar system used in most countries in the world today.
/// It was originally designed by Aloysius Lilius. It officially replaced the Julian calendar
/// in October 1582 in the Papal States, as part of a decree by Pope Gregory XIII.
///
/// ### Proleptic Modification
///
/// According to Wikipedia:
/// > The proleptic Gregorian calendar is produced by extending the Gregorian
/// > calendar backward to the dates preceding its official introduction in 1582.
///
/// This means there are no "skipped days" at the point where the Gregorian
/// calendar was introduced. Additionally, this means that year 0 is considered
/// valid for this implementation of the Gregorian calendar.
///
/// The Gregorian reform was implemented at different times in different countries.
/// For consistency with historical dates before the Gregorian reform, applications
/// should probably use the Julian calendar.
///
/// ## Basic Structure
///
/// Years are divided into 12 months. Every month has either 30 or 31 days except for the
/// second month, February. February has 28 days in a common year and 29 days in a leap year.
///
/// Leap years occur on every year divisible by 400, and additionally on every year divisible
/// by 4 but not divisible by 100.
///
/// ## Epoch
///
/// The first day of the first year of the proleptic Gregorian calendar differs slightly from
/// that of the Julian calendar. This is one of the side effects of using a proleptic calendar
/// as mentioned in the "Proleptic Modification" section.
///
/// This crate uses the term "Common Era" (abbreviated "CE") specifically for the proleptic
/// Gregorian calendar epoch. The term "Anno Domini" (abbreviated "AD") is used for the Julian
/// calendar instead.
///
/// ## Representation and Examples
///
/// ### Months
///
/// The months are represented in this crate as [`GregorianMonth`].
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_1_1 = CommonDate::new(2025, 1, 1);
/// let a_1_1 = Gregorian::try_from_common_date(c_1_1).unwrap();
/// assert_eq!(a_1_1.month(), GregorianMonth::January);
/// ```
///
/// ### Conversion from Julian
///
/// For historical dates, it is often necessary to convert from the Julian system.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let j = Julian::try_new(1752, JulianMonth::September, 3).unwrap();
/// let g = j.convert::<Gregorian>();
/// assert_eq!(g, Gregorian::try_new(1752, GregorianMonth::September, 14).unwrap());
/// ```
///
/// ## Inconsistencies with Other Implementations
///
/// Some other tools might use non-proleptic Gregorian calendars. See the "Proleptic
/// Modification" section for details.
///
/// For example, the UNIX `cal` command uses a non-proleptic Gregorian calendar by default.
/// The default settings assume 3 September 1752 was the date of the Gregorian reform (this
/// was the date used in the British Empire). Thus, some days of September 1752 are skipped:
///
/// ```bash
/// $ cal September 1752
///   September 1752   
/// Su Mo Tu We Th Fr Sa
///        1  2 14 15 16
/// 17 18 19 20 21 22 23
/// 24 25 26 27 28 29 30
/// ```
///
/// To imitate such behaviour in this crate, callers must explicitly switch between the
/// Julian and the Gregorian calendar. See the "Conversion from Julian" section for an example.
///
///
/// ## Further reading
/// + Wikipedia
///   + [Gregorian calendar](https://en.wikipedia.org/wiki/Gregorian_calendar)
///   + [Proleptic Gregorian calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar)
/// + [OpenGroup `cal`](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cal.html)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Gregorian(CommonDate);

impl Gregorian {
    pub fn prior_elapsed_days(year: i32) -> i64 {
        //LISTING 2.17 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //These are the terms of the sum which do not rely on the month or day.
        //LISTING PriorElapsedDays (*Basic Symmetry454 and Symmetry010 Calendar Arithmetic* by Dr. Irvin L. Bromberg)
        let year = year as i64;
        let offset_e = Gregorian::epoch().get_day_i() - 1;
        let offset_y = 365 * (year - 1);
        let offset_leap =
            (year - 1).div_euclid(4) - (year - 1).div_euclid(100) + (year - 1).div_euclid(400);
        offset_e + offset_y + offset_leap
    }
}

impl AllowYearZero for Gregorian {}

impl ToFromOrdinalDate for Gregorian {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        let correction = if Gregorian::is_leap(ord.year) { 1 } else { 0 };
        if ord.day_of_year > 0 && ord.day_of_year <= (365 + correction) {
            Ok(())
        } else {
            Err(CalendarError::InvalidDayOfYear)
        }
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        //LISTING 2.21-2.22 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
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

    fn to_ordinal(self) -> OrdinalDate {
        //LISTING 2.17 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //These are the terms of the sum which rely on the month or day
        let month = self.0.month as i64;
        let day = self.0.day as i64;
        let offset_m = ((367 * month) - 362).div_euclid(12);
        let offset_x = if month <= 2 {
            0
        } else if Gregorian::is_leap(self.0.year) {
            -1
        } else {
            -2
        };
        let offset_d = day;
        OrdinalDate {
            year: self.0.year,
            day_of_year: (offset_m + offset_x + offset_d) as u16,
        }
    }

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        //LISTING 2.23 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to use ordinal day counts instead of day counts from the epoch
        let year = ord.year;
        let prior_days: i32 = (ord.day_of_year as i32) - 1; //Modification
        let ord_march1 = Gregorian(CommonDate::new(year, 3, 1)).to_ordinal(); //Modification
        let correction: i32 = if ord < ord_march1 {
            //Modification
            0
        } else if Gregorian::is_leap(year) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373).div_euclid(367) as u8;
        let ord_month = Gregorian(CommonDate::new(year, month, 1)).to_ordinal(); //Modification
        let day = ((ord.day_of_year - ord_month.day_of_year) as u8) + 1; //Modification
        debug_assert!(day > 0);
        Gregorian(CommonDate { year, month, day })
    }
}

impl HasLeapYears for Gregorian {
    fn is_leap(g_year: i32) -> bool {
        //LISTING 2.16 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
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
        Gregorian::from_ordinal_unchecked(ord)
    }
}

impl ToFixed for Gregorian {
    fn to_fixed(self) -> Fixed {
        //LISTING 2.17 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Method is split compared to the original
        let offset_prior = Gregorian::prior_elapsed_days(self.0.year);
        let ord = self.to_ordinal().day_of_year as i64;
        Fixed::cast_new(offset_prior + ord)
    }
}

impl ToFromCommonDate<GregorianMonth> for Gregorian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_ymd(date).is_ok());
        Self(date)
    }

    fn valid_ymd(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = GregorianMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > Self::month_length(date.year, month_opt.unwrap()) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        //LISTING 2.19 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let m = GregorianMonth::December;
        CommonDate::new(year, m as u8, Self::month_length(year, m))
    }

    fn month_length(year: i32, month: GregorianMonth) -> u8 {
        //LISTING ?? SECTION 2.1 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //TODO: use listing 2.1 here?
        match month {
            GregorianMonth::January => 31,
            GregorianMonth::February => {
                if Gregorian::is_leap(year) {
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

impl Quarter for Gregorian {
    fn quarter(self) -> NonZero<u8> {
        NonZero::new(((self.to_common_date().month - 1) / 3) + 1).expect("(m-1)/3 > -1")
    }
}

impl GuaranteedMonth<GregorianMonth> for Gregorian {}
impl CommonWeekOfYear<GregorianMonth> for Gregorian {}

/// Represents a date *and time* in the Gregorian Calendar
pub type GregorianMoment = CalendarMoment<Gregorian>;

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

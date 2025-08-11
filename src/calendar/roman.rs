// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::julian::Julian;
use crate::calendar::julian::JulianMonth;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use std::cmp::Ordering;
use std::num::NonZero;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

//LISTING 3.12 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
const YEAR_ROME_FOUNDED_JULIAN: i32 = -753;

/// Represents key events in a Roman month
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum RomanMonthlyEvent {
    //LISTING 3.5-3.7 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
    Kalends = 1,
    Nones,
    Ides,
}

/// Represents a month in the Roman calendar after the Julian reform
pub type RomanMonth = JulianMonth;

impl RomanMonth {
    pub fn ides_of_month(self) -> u8 {
        //LISTING 3.8 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        match self {
            RomanMonth::July => 15,
            RomanMonth::March => 15,
            RomanMonth::May => 15,
            RomanMonth::October => 15,
            _ => 13,
        }
    }

    pub fn nones_of_month(self) -> u8 {
        //LISTING 3.9 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        self.ides_of_month() - 8
    }
}

/// Represents a date in the Roman calendar after the Julian reform
///
/// This is essentially an alternative system for naming Julian dates.
///
/// ## Year 0
///
/// Year 0 is **not** supported because they are not supported in the Julian calendar.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Roman {
    year: NonZero<i32>,
    month: RomanMonth,
    event: RomanMonthlyEvent,
    count: NonZero<u8>,
    leap: bool,
}

impl Roman {
    pub fn year(self) -> NonZero<i32> {
        self.year
    }

    pub fn month(self) -> RomanMonth {
        self.month
    }

    pub fn event(self) -> RomanMonthlyEvent {
        self.event
    }

    pub fn count(self) -> NonZero<u8> {
        self.count
    }

    pub fn leap(self) -> bool {
        self.leap
    }

    /// Converts from BC/AD year to AUC year
    pub fn julian_year_from_auc(year: NonZero<i32>) -> NonZero<i32> {
        //LISTING 3.13 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to use NonZero
        let j_year = year.get();
        if j_year >= 1 && j_year <= -YEAR_ROME_FOUNDED_JULIAN {
            NonZero::new(j_year + YEAR_ROME_FOUNDED_JULIAN - 1).expect("Checked by if")
        } else {
            NonZero::new(j_year + YEAR_ROME_FOUNDED_JULIAN).expect("Checked by if")
        }
    }

    /// Converts from AUC year to BC/AD year
    pub fn auc_year_from_julian(year: NonZero<i32>) -> NonZero<i32> {
        //LISTING 3.14 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to use NonZero
        let a_year = year.get();
        if YEAR_ROME_FOUNDED_JULIAN <= a_year && a_year <= -1 {
            NonZero::new(a_year - YEAR_ROME_FOUNDED_JULIAN + 1).expect("Checked by if")
        } else {
            NonZero::new(a_year - YEAR_ROME_FOUNDED_JULIAN).expect("Checked by if")
        }
    }
}

impl PartialOrd for Roman {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.year != other.year {
            self.year.partial_cmp(&other.year)
        } else if self.month != other.month {
            self.month.partial_cmp(&other.month)
        } else if self.event != other.event {
            self.event.partial_cmp(&other.event)
        } else if self.count != other.count {
            other.count.partial_cmp(&self.count) //Intentionally reversed, "count" decreases with time
        } else {
            // "the second sixth day before the kalends of March"
            (self.leap as u8).partial_cmp(&(other.leap as u8))
        }
    }
}

impl CalculatedBounds for Roman {}

impl FromFixed for Roman {
    fn from_fixed(date: Fixed) -> Roman {
        //LISTING 3.11 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let j = Julian::from_fixed(date);
        let j_cdate = j.to_common_date();
        let month = (j_cdate.month as i64).adjusted_remainder(12) as u8;
        let year = j_cdate.year;
        let day = j_cdate.day;
        let month1 = (month as i64 + 1).adjusted_remainder(12) as u8;
        let year1 = if month1 != 1 {
            year
        } else if year != -1 {
            year + 1
        } else {
            1
        };
        let month_r = RomanMonth::from_u8(month).expect("Kept in range by adjusted_remainder");
        let month1_r = RomanMonth::from_u8(month1).expect("Kept in range by adjusted_remainder");
        let kalends1 = Roman {
            year: NonZero::new(year1).expect("From Julian date"),
            month: month1_r,
            event: RomanMonthlyEvent::Kalends,
            count: NonZero::new(1).expect("1 != 0"),
            leap: false,
        }
        .to_fixed()
        .get_day_i();
        if day == 1 {
            Roman {
                year: NonZero::new(year).expect("From Julian date"),
                month: month_r,
                event: RomanMonthlyEvent::Kalends,
                count: NonZero::new(1).expect("1 != 0"),
                leap: false,
            }
        } else if day <= month_r.nones_of_month() {
            Roman {
                year: NonZero::new(year).expect("From Julian date"),
                month: month_r,
                event: RomanMonthlyEvent::Nones,
                count: NonZero::new(month_r.nones_of_month() - day + 1).expect("Checked in if"),
                leap: false,
            }
        } else if day <= month_r.ides_of_month() {
            Roman {
                year: NonZero::new(year).expect("From Julian date"),
                month: month_r,
                event: RomanMonthlyEvent::Ides,
                count: NonZero::new(month_r.ides_of_month() - day + 1).expect("Checked in if"),
                leap: false,
            }
        } else if month_r != RomanMonth::February || !Julian::is_leap(year) {
            Roman {
                year: NonZero::new(year1).expect("From Julian date"),
                month: month1_r,
                event: RomanMonthlyEvent::Kalends,
                count: NonZero::new(((kalends1 - date.get_day_i()) + 1) as u8)
                    .expect("kalends1 > date"),
                leap: false,
            }
        } else if day < 25 {
            Roman {
                year: NonZero::new(year).expect("From Julian date"),
                month: RomanMonth::March,
                event: RomanMonthlyEvent::Kalends,
                count: NonZero::new((30 - day) as u8).expect("day < 25 < 30"),
                leap: false,
            }
        } else {
            Roman {
                year: NonZero::new(year).expect("From Julian date"),
                month: RomanMonth::March,
                event: RomanMonthlyEvent::Kalends,
                count: NonZero::new((31 - day) as u8).expect("days in February < 31"),
                leap: day == 25,
            }
        }
    }
}

impl ToFixed for Roman {
    fn to_fixed(self) -> Fixed {
        //LISTING 3.10 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let jld = match self.event {
            RomanMonthlyEvent::Kalends => 1,
            RomanMonthlyEvent::Nones => self.month.nones_of_month(),
            RomanMonthlyEvent::Ides => self.month.ides_of_month(),
        };
        let jlc = CommonDate::new(self.year.get(), self.month as u8, jld);
        let j = Julian::try_from_common_date(jlc)
            .expect("Month/day in range")
            .to_fixed()
            .get_day_i();
        let c = self.count.get() as i64;
        let do_lp = Julian::is_leap(self.year.get())
            && self.month == RomanMonth::March
            && self.event == RomanMonthlyEvent::Kalends
            && self.count.get() <= 16
            && self.count.get() >= 6;
        let lp0 = if do_lp { 0 } else { 1 };
        let lp1 = if self.leap { 1 } else { 0 };
        Fixed::cast_new(j - c + lp0 + lp1)
    }
}

impl Quarter for Roman {
    fn quarter(self) -> NonZero<u8> {
        NonZero::new((((self.month() as u8) - 1) / 3) + 1).expect("m/4 > -1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::prelude::ToFromCommonDate;
    use proptest::prop_assume;
    use proptest::proptest;

    #[test]
    fn second_sixth_day_before_kalends_of_march() {
        let j24 = Julian::try_from_common_date(CommonDate::new(4, 2, 24)).unwrap();
        let j25 = Julian::try_from_common_date(CommonDate::new(4, 2, 25)).unwrap();
        let f24 = j24.to_fixed();
        let f25 = j25.to_fixed();
        let r24 = Roman::from_fixed(f24);
        let r25 = Roman::from_fixed(f25);
        assert_eq!(r24.year(), r25.year());
        assert_eq!(r24.month(), r25.month());
        assert_eq!(r24.event(), r25.event());
        assert_eq!(r24.count(), r25.count());
        assert!(!r24.leap() && r25.leap());
        assert!(r24 < r25);
    }

    #[test]
    fn ides_of_march() {
        let j = Julian::try_from_common_date(CommonDate::new(-44, 3, 15)).unwrap();
        let f = j.to_fixed();
        let r = Roman::from_fixed(f);
        assert_eq!(r.event, RomanMonthlyEvent::Ides);
        assert_eq!(r.month, RomanMonth::March);
        assert_eq!(r.count.get(), 1);
    }

    proptest! {
        #[test]
        fn auc_roundtrip(t in i16::MIN..i16::MAX) {
            prop_assume!(t != 0);
            assert_eq!(t as i32, Roman::julian_year_from_auc(Roman::auc_year_from_julian(NonZero::new(t as i32).unwrap())).get());
        }
    }
}

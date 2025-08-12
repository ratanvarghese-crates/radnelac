// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::gregorian::Gregorian;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::CommonWeekOfYear;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::prelude::ToFromOrdinalDate;
use crate::calendar::CalendarMoment;
use crate::calendar::HasLeapYears;
use crate::clock::TimeOfDay;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::CalendarError;
use num_traits::FromPrimitive;
use std::cmp::Ordering;
use std::num::NonZero;

/// Represents a date in the ISO-8601 week-date calendar
///
/// This is essentially an alternative naming system for Gregorian dates.
///
/// Despite being derived from the Gregorian calendar, **the ISO-8601 has a different year
/// start and year end than the Gregorian.** If the Gregorian year X ends in the middle of
/// the ISO week, the next days may be in Gregorian year X+1 and ISO year X.
///
/// ## Year 0
///
/// Year 0 is supported for this calendar.
///
/// ## Further reading
/// + [Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date)
/// + [Rachel by the Bay](https://rachelbythebay.com/w/2018/04/20/iso/)
///   + describes the confusion of intermingling documentation for ISO and Gregorian dates
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ISO {
    year: i32,
    week: NonZero<u8>,
    day: Weekday,
}

impl ISO {
    /// Attempt to create a new ISO week date
    pub fn try_new(year: i32, week: u8, day: Weekday) -> Result<Self, CalendarError> {
        if week < 1 || week > 53 || (week == 53 && !Self::is_leap(year)) {
            //This if statement is structured specifically to minimize calls to Self::is_leap.
            //Self::is_leap calls Gregorian calendar functions which may exceed the effective
            //bounds.
            return Err(CalendarError::InvalidWeek);
        }
        Ok(ISO {
            year: year,
            week: NonZero::<u8>::new(week).expect("Checked in if"),
            day: day,
        })
    }

    pub fn year(self) -> i32 {
        self.year
    }

    pub fn week(self) -> NonZero<u8> {
        self.week
    }

    /// Note that the numeric values of the Weekday enum are not consistent with ISO-8601.
    /// Use day_num for the numeric day number.
    pub fn day(self) -> Weekday {
        self.day
    }

    /// Represents Sunday as 7 instead of 0, as required by ISO-8601.
    pub fn day_num(self) -> u8 {
        (self.day as u8).adjusted_remainder(7)
    }

    pub fn new_year(year: i32) -> Self {
        ISO::try_new(year, 1, Weekday::Monday).expect("Week 1 known to be valid")
    }
}

impl PartialOrd for ISO {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.year != other.year {
            self.year.partial_cmp(&other.year)
        } else if self.week != other.week {
            self.week.partial_cmp(&other.week)
        } else {
            let self_day = (self.day as i64).adjusted_remainder(7);
            let other_day = (other.day as i64).adjusted_remainder(7);
            self_day.partial_cmp(&other_day)
        }
    }
}

impl CalculatedBounds for ISO {}

impl Epoch for ISO {
    fn epoch() -> Fixed {
        Gregorian::epoch()
    }
}

impl HasLeapYears for ISO {
    fn is_leap(i_year: i32) -> bool {
        //LISTING 5.3 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let jan1 = Gregorian::try_year_start(i_year)
            .expect("Year known to be valid")
            .convert::<Weekday>();
        let dec31 = Gregorian::try_year_end(i_year)
            .expect("Year known to be valid")
            .convert::<Weekday>();
        jan1 == Weekday::Thursday || dec31 == Weekday::Thursday
    }
}

impl FromFixed for ISO {
    fn from_fixed(fixed_date: Fixed) -> ISO {
        //LISTING 5.2 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let date = fixed_date.get_day_i();
        let approx = Gregorian::ordinal_from_fixed(Fixed::cast_new(date - 3)).year;
        let next = ISO::new_year(approx + 1).to_fixed().get_day_i();
        let year = if date >= next { approx + 1 } else { approx };
        let current = ISO::new_year(year).to_fixed().get_day_i();
        let week = (date - current).div_euclid(7) + 1;
        debug_assert!(week < 55 && week > 0);
        //Calendrical Calculations stores "day" as 7 for Sunday, as per ISO.
        //However since we have an unambiguous enum, we can save such details for
        //functions that need it. We also adjust "to_fixed_unchecked"
        let day = Weekday::from_u8(date.modulus(7) as u8).expect("In range due to modulus.");
        ISO::try_new(year, week as u8, day).expect("Week known to be valid")
    }
}

impl ToFixed for ISO {
    fn to_fixed(self) -> Fixed {
        //LISTING 5.1 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let g = CommonDate::new(self.year - 1, 12, 28);
        let w = NonZero::<i16>::from(self.week);
        //Calendrical Calculations stores "day" as 7 for Sunday, as per ISO.
        //However since we have an unambiguous enum, we can save such details for
        //functions that need it. We also adjust "from_fixed_unchecked"
        let day_i = (self.day as i64).adjusted_remainder(7);
        let result = Gregorian::try_from_common_date(g)
            .expect("month 12, day 28 is always valid for Gregorian")
            .nth_kday(w, Weekday::Sunday)
            .get_day_i()
            + day_i;
        Fixed::cast_new(result)
    }
}

impl Quarter for ISO {
    fn quarter(self) -> NonZero<u8> {
        NonZero::new(((self.week().get() - 1) / 14) + 1).expect("(m - 1)/14 > -1")
    }
}

/// Represents a date *and time* in the ISO Calendar
pub type ISOMoment = CalendarMoment<ISO>;

impl ISOMoment {
    pub fn year(self) -> i32 {
        self.date().year()
    }

    pub fn week(self) -> NonZero<u8> {
        self.date().week()
    }

    /// Note that the numeric values of the Weekday enum are not consistent with ISO-8601.
    /// Use day_num for the numeric day number.
    pub fn day(self) -> Weekday {
        self.date().day()
    }

    /// Represents Sunday as 7 instead of 0, as required by ISO-8601.
    pub fn day_num(self) -> u8 {
        self.date().day_num()
    }

    pub fn new_year(year: i32) -> Self {
        ISOMoment::new(ISO::new_year(year), TimeOfDay::midnight())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::prelude::HasLeapYears;
    use crate::calendar::prelude::ToFromCommonDate;
    use crate::day_count::FIXED_MAX;
    use proptest::proptest;
    const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

    #[test]
    fn week_of_impl() {
        let g = Gregorian::try_from_common_date(CommonDate::new(2025, 5, 15))
            .unwrap()
            .to_fixed();
        let i = ISO::from_fixed(g);
        assert_eq!(i.week().get(), 20);
    }

    #[test]
    fn epoch() {
        let i0 = ISO::from_fixed(Fixed::cast_new(0));
        let i1 = ISO::from_fixed(Fixed::cast_new(-1));
        assert!(i0 > i1, "i0: {:?}, i1: {:?}", i0, i1);
    }

    proptest! {
        #[test]
        fn first_week(year in -MAX_YEARS..MAX_YEARS) {
            // https://en.wikipedia.org/wiki/ISO_week_date
            // > If 1 January is on a Monday, Tuesday, Wednesday or Thursday, it is in W01.
            // > If it is on a Friday, it is part of W53 of the previous year. If it is on a
            // > Saturday, it is part of the last week of the previous year which is numbered
            // > W52 in a common year and W53 in a leap year. If it is on a Sunday, it is part
            // > of W52 of the previous year.
            let g = Gregorian::try_from_common_date(CommonDate {
                year,
                month: 1,
                day: 1,
            }).unwrap();
            let f = g.to_fixed();
            let w = Weekday::from_fixed(f);
            let i = ISO::from_fixed(f);
            let expected_week: u8 = match w {
                Weekday::Monday => 1,
                Weekday::Tuesday => 1,
                Weekday::Wednesday => 1,
                Weekday::Thursday => 1,
                Weekday::Friday => 53,
                Weekday::Saturday => if Gregorian::is_leap(year - 1) {53} else {52},
                Weekday::Sunday => 52,
            };
            let expected_year: i32 = if expected_week == 1 { year } else { year - 1 };
            assert_eq!(i.day(), w);
            assert_eq!(i.week().get(), expected_week);
            assert_eq!(i.year(), expected_year);
            if expected_week == 53 {
                assert!(ISO::is_leap(i.year()));
            }
        }

        #[test]
        fn fixed_week_numbers(y1 in -MAX_YEARS..MAX_YEARS, y2 in -MAX_YEARS..MAX_YEARS) {
            // https://en.wikipedia.org/wiki/ISO_week_date
            // > For all years, 8 days have a fixed ISO week number
            // > (between W01 and W08) in January and February
            // Month       Days                Weeks
            // January     04  11  18  25      W01 – W04
            // February    01  08  15  22  29  W05 – W09
            let targets = [
                (1, 4), (1, 11), (1, 18), (1, 25),
                (2, 1), (2, 8), (2, 15), (2, 22),
            ];
            for target in targets {
                let g1 = Gregorian::try_from_common_date(CommonDate {
                    year: y1,
                    month: target.0,
                    day: target.1,
                }).unwrap();
                let g2 = Gregorian::try_from_common_date(CommonDate {
                    year: y2,
                    month: target.0,
                    day: target.1,
                }).unwrap();
                assert_eq!(g1.convert::<ISO>().week(), g2.convert::<ISO>().week());
            }
        }
    }
}

use crate::calendar::gregorian::Gregorian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::ToFromCommonDate;
use crate::common::math::TermNum;
use crate::day_count::CalculatedBounds;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use num_traits::FromPrimitive;
use std::cmp::Ordering;
use std::num::NonZero;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ISO {
    year: i32,
    week: NonZero<u8>,
    day: Weekday,
}

impl ISO {
    pub fn year(self) -> i32 {
        self.year
    }

    pub fn week(self) -> NonZero<u8> {
        self.week
    }

    pub fn day(self) -> Weekday {
        self.day
    }

    pub fn is_long_year(i_year: i16) -> bool {
        let jan1 = Weekday::from_fixed(Gregorian::new_year(i_year));
        let dec31 = Weekday::from_fixed(Gregorian::year_end(i_year));
        jan1 == Weekday::Thursday || dec31 == Weekday::Thursday
    }

    pub fn new_year(year: i32) -> Self {
        ISO {
            year: year,
            week: NonZero::new(1).unwrap(),
            day: Weekday::Monday,
        }
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

impl FromFixed for ISO {
    fn from_fixed(fixed_date: Fixed) -> ISO {
        let date = fixed_date.get_day_i();
        let approx = Gregorian::ordinal_from_fixed(Fixed::cast_new(date - 3)).year;
        let next = ISO {
            year: approx + 1,
            week: NonZero::<u8>::new(1).unwrap(),
            day: Weekday::Monday,
        }
        .to_fixed();
        let year = if date >= next.get_day_i() {
            approx + 1
        } else {
            approx
        };
        let current = ISO {
            year: year,
            week: NonZero::<u8>::new(1).unwrap(),
            day: Weekday::Monday,
        }
        .to_fixed()
        .get_day_i();
        let week = (date - current).div_euclid(7) + 1;
        debug_assert!(week < 55 && week > 0);
        //Calendrical Calculations stores "day" as 7 for Sunday, as per ISO.
        //However since we have an unambiguous enum, we can save such details for
        //functions that need it. We also adjust "to_fixed_unchecked"
        let day = Weekday::from_u8(date.modulus(7) as u8).expect("In range due to modulus.");
        ISO {
            year: year,
            week: NonZero::new(week as u8).unwrap(),
            day: day,
        }
    }
}

impl ToFixed for ISO {
    fn to_fixed(self) -> Fixed {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bound::EffectiveBound;
    use crate::common::date::ToFromCommonDate;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
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
    fn bounds_actually_work() {
        assert!(ISO::from_fixed(Fixed::effective_min()) < ISO::from_fixed(Fixed::cast_new(0)));
        assert!(ISO::from_fixed(Fixed::effective_max()) > ISO::from_fixed(Fixed::cast_new(0)));
    }

    #[test]
    fn epoch() {
        let i0 = ISO::from_fixed(Fixed::cast_new(0));
        let i1 = ISO::from_fixed(Fixed::cast_new(-1));
        assert!(i0 > i1, "i0: {:?}, i1: {:?}", i0, i1);
    }

    proptest! {
        #[test]
        fn roundtrip(t in FIXED_MIN..FIXED_MAX) {
            let f0 = Fixed::new(t).to_day();
            let i = ISO::from_fixed(f0);
            let f1 = i.to_fixed();
            assert_eq!(f0, f1);
        }

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
            assert_eq!(i.year(), expected_year)
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
                let i1 = ISO::from_fixed(g1.to_fixed());
                let i2 = ISO::from_fixed(g2.to_fixed());
                assert_eq!(i1.week().get(), i2.week().get());
            }
        }
    }
}

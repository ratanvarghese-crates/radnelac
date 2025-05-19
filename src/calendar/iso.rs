use crate::calendar::gregorian::Gregorian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::math::TermNum;
use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;
use crate::day_cycle::week::Weekday;
use num_traits::FromPrimitive;
use std::num::NonZero;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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

    pub fn to_fixed_unchecked(year: i32, week: NonZero<u8>, day: Weekday) -> i64 {
        let g = CommonDate::new(year - 1, 12, 28);
        let w = NonZero::<i16>::from(week);
        //Calendrical Calculations stores "day" as 7 for Sunday, as per ISO.
        //However since we have an unambiguous enum, we can save such details for a
        //formatting function. We also adjust "from_fixed_unchecked"
        let day_i = (day as i64).adjusted_remainder(7);
        Gregorian::nth_kday_unchecked(g, w, Weekday::Sunday) + (day_i)
    }

    pub fn from_fixed_unchecked(date: i64) -> (i32, NonZero<u8>, Weekday) {
        let approx = Gregorian::ordinal_from_fixed_generic_unchecked(
            date - 3,
            Gregorian::epoch().get_day_i(),
        )
        .year;
        let next =
            ISO::to_fixed_unchecked(approx + 1, NonZero::<u8>::new(1).unwrap(), Weekday::Monday);
        let year = if date >= next { approx + 1 } else { approx };
        let current =
            ISO::to_fixed_unchecked(year, NonZero::<u8>::new(1).unwrap(), Weekday::Monday);
        let week = (date - current).div_euclid(7) + 1;
        debug_assert!(week < 55 && week > 0);
        //Calendrical Calculations stores "day" as 7 for Sunday, as per ISO.
        //However since we have an unambiguous enum, we can save such details for a
        //formatting function. We also adjust "to_fixed_unchecked"
        let day = Weekday::from_u8(date.modulus(7) as u8).expect("In range due to modulus.");
        (year, NonZero::new(week as u8).unwrap(), day)
    }
}

impl CalculatedBounds for ISO {}

impl FromFixed for ISO {
    fn from_fixed(date: Fixed) -> ISO {
        let result = ISO::from_fixed_unchecked(date.get_day_i());
        ISO {
            year: result.0,
            week: result.1,
            day: result.2,
        }
    }
}

impl ToFixed for ISO {
    fn to_fixed(self) -> Fixed {
        let result = ISO::to_fixed_unchecked(self.year, self.week, self.day);
        Fixed::cast_new(result).expect("TODO: verify")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bound::EffectiveBound;
    use crate::common::date::ToFromCommonDate;
    use crate::common::math::EFFECTIVE_MAX;
    use crate::common::math::EFFECTIVE_MIN;
    use proptest::proptest;
    const MAX_YEARS: i32 = (EFFECTIVE_MAX / 365.25) as i32;

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
        assert!(ISO::from_fixed(Fixed::effective_min()) < ISO::from_fixed(Fixed::new(0)));
        assert!(ISO::from_fixed(Fixed::effective_max()) > ISO::from_fixed(Fixed::new(0)));
    }

    proptest! {
        #[test]
        fn roundtrip(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let f0 = Fixed::checked_new(t).unwrap().to_day();
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

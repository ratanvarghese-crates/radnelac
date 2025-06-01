// "The Importance of Calendar Reform to the Business World"
// by George Eastman, President, Eastman Kodak Company
// "Nation's Business" May 1926
// https://www.freexenon.com/wp-content/uploads/2018/07/The-Importance-of-Calendar-Reform-to-the-Business-World-George-Eastman.pdf

use crate::calendar::gregorian::Gregorian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum CotsworthMonth {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    Sol,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum CotsworthComplementaryDay {
    YearDay = 1,
    LeapDay,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Cotsworth(CommonDate);

impl Cotsworth {
    pub fn year(self) -> i32 {
        self.0.year
    }

    pub fn month(self) -> CotsworthMonth {
        CotsworthMonth::from_u8(self.0.month).expect("Will not allow setting invalid value.")
    }

    pub fn day(self) -> u8 {
        self.0.day
    }

    pub fn complementary(self) -> Option<CotsworthComplementaryDay> {
        if self.0.day == 29 && self.0.month == (CotsworthMonth::December as u8) {
            Some(CotsworthComplementaryDay::YearDay)
        } else if self.0.day == 29 && self.0.month == (CotsworthMonth::June as u8) {
            Some(CotsworthComplementaryDay::LeapDay)
        } else {
            None
        }
    }

    pub fn weekday(self) -> Option<Weekday> {
        if self.complementary().is_some() {
            None
        } else {
            Weekday::from_i64(((self.0.day as i64) - 1).modulus(7))
        }
    }

    pub fn is_leap(c_year: i32) -> bool {
        Gregorian::is_leap(c_year)
    }

    pub fn complementary_count(p_year: i32) -> u8 {
        if Cotsworth::is_leap(p_year) {
            2
        } else {
            1
        }
    }
}

impl CalculatedBounds for Cotsworth {}

impl Epoch for Cotsworth {
    fn epoch() -> Fixed {
        Gregorian::epoch()
    }
}

impl FromFixed for Cotsworth {
    fn from_fixed(fixed_date: Fixed) -> Cotsworth {
        let ord = Gregorian::ordinal_from_fixed(fixed_date);
        const LEAP_DAY_ORD: u16 = (6 * 28) + 1;
        let result = match (ord.day_of_year, Cotsworth::is_leap(ord.year)) {
            (366, true) => CommonDate::new(ord.year, 13, 29),
            (365, false) => CommonDate::new(ord.year, 13, 29),
            (LEAP_DAY_ORD, true) => CommonDate::new(ord.year, 6, 29),
            (doy, is_leap) => {
                let correction = if doy < LEAP_DAY_ORD || !is_leap { 0 } else { 1 };
                let month = ((((doy - correction) - 1) as i64).div_euclid(28) + 1) as u8;
                let day = ((doy - correction) as i64).adjusted_remainder(28) as u8;
                CommonDate::new(ord.year, month, day)
            }
        };
        Cotsworth(result)
    }
}

impl ToFixed for Cotsworth {
    fn to_fixed(self) -> Fixed {
        let offset_y = Gregorian::try_from_common_date(CommonDate::new(self.0.year, 1, 1))
            .expect("month 1, day 1 should always be a valid Gregorian date")
            .to_fixed()
            .get_day_i()
            - 1;
        let approx_m = ((self.0.month as i64) - 1) * 28;
        let offset_m = if self.0.month > 6 && Cotsworth::is_leap(self.0.year) {
            approx_m + 1
        } else {
            approx_m
        };
        Fixed::cast_new(offset_y + offset_m + (self.0.day as i64))
    }
}

impl ToFromCommonDate for Cotsworth {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        if date.month < 1 || date.month > 13 {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 || date.day > 29 {
            Err(CalendarError::InvalidDay)
        } else if date.day == 29 {
            if date.month == 13 || (Cotsworth::is_leap(date.year) && date.month == 6) {
                Ok(())
            } else {
                Err(CalendarError::InvalidDay)
            }
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::gregorian::GregorianMonth;
    use crate::common::bound::EffectiveBound;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use crate::day_count::RataDie;
    use proptest::proptest;
    const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

    #[test]
    fn bounds_actually_work() {
        assert!(
            Cotsworth::from_fixed(Fixed::effective_min())
                < Cotsworth::from_fixed(Fixed::cast_new(0))
        );
        assert!(
            Cotsworth::from_fixed(Fixed::effective_max())
                > Cotsworth::from_fixed(Fixed::cast_new(0))
        );
    }

    proptest! {
        #[test]
        fn valid_day(t0 in FIXED_MIN..FIXED_MAX) {
            let t = Fixed::new(t0);
            let e1 = Cotsworth::from_fixed(t);
            assert!(Cotsworth::valid_month_day(e1.to_common_date()).is_ok());
        }

        #[test]
        fn complementary_xor_weekday(t in FIXED_MIN..FIXED_MAX) {
            let t0 = RataDie::new(t).to_fixed().to_day();
            let r0 = Cotsworth::from_fixed(t0);
            let w0 = r0.weekday();
            let s0 = r0.complementary();
            assert_ne!(w0.is_some(), s0.is_some());
        }

        #[test]
        fn roundtrip(year in -MAX_YEARS..MAX_YEARS, month in 1..13, day in 1..28) {
            let d = CommonDate::new(year, month as u8, day as u8);
            let e0 = Cotsworth::try_from_common_date(d).unwrap();
            let t = e0.to_fixed();
            let e1 = Cotsworth::from_fixed(t);
            if e0 != e1 {
                println!("{:?}\n{:?}\n{:?}\n", e0, e1, Gregorian::from_fixed(t));
            }
            assert_eq!(e0, e1);
        }

        #[test]
        fn start_with_gregorian(year in -MAX_YEARS..MAX_YEARS) {
            let d = CommonDate::new(year, 1, 1);
            let p = Cotsworth::try_from_common_date(d).unwrap();
            let g = Gregorian::from_fixed(p.to_fixed());
            assert_eq!(g.year(), year);
            assert_eq!(g.month(), GregorianMonth::January);
            assert_eq!(g.day(), 1);
            assert_eq!(p.weekday().unwrap(), Weekday::Sunday);
        }

        #[test]
        fn end_with_gregorian(year in -MAX_YEARS..MAX_YEARS) {
            let d = CommonDate::new(year, 13, 29);
            let p = Cotsworth::try_from_common_date(d).unwrap();
            let g = Gregorian::from_fixed(p.to_fixed());

            assert_eq!(p.complementary().unwrap(), CotsworthComplementaryDay::YearDay);
            assert_eq!(g.year(), year);
            assert_eq!(g.month(), GregorianMonth::December);
            assert_eq!(g.day(), 31);
        }

        #[test]
        fn invalid_common(year in -MAX_YEARS..MAX_YEARS, month in 15..u8::MAX, day in 30..u8::MAX) {
            let d_list = [
                CommonDate{ year, month, day },
                CommonDate{ year, month: 1, day},
                CommonDate{ year, month, day: 1 },
                CommonDate{ year, month: 1, day: 0},
                CommonDate{ year, month: 0, day: 1 }
            ];
            for d in d_list {
                assert!(Cotsworth::try_from_common_date(d).is_err());
            }
        }

        #[test]
        fn consistent_order(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
            let f0 = Fixed::new(t0);
            let f1 = Fixed::new(t1);
            let d0 = Cotsworth::from_fixed(f0);
            let d1 = Cotsworth::from_fixed(f1);
            let c0 = d0.to_common_date();
            let c1 = d1.to_common_date();
            assert_eq!(f0 < f1, (d0 < d1) && (c0 < c1));
            assert_eq!(f0 <= f1, (d0 <= d1) && (c0 <= c1));
            assert_eq!(f0 == f1, (d0 == d1) && (c0 == c1));
            assert_eq!(f0 >= f1, (d0 >= d1) && (c0 >= c1));
            assert_eq!(f0 > f1, (d0 > d1) && (c0 > c1));
        }

        #[test]
        fn consistent_order_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
            let f0 = Fixed::new(t0);
            let f1 = Fixed::new(t0 + (diff as f64));
            let d0 = Cotsworth::from_fixed(f0);
            let d1 = Cotsworth::from_fixed(f1);
            let c0 = d0.to_common_date();
            let c1 = d1.to_common_date();
            assert_eq!(f0 < f1, (d0 < d1) && (c0 < c1));
            assert_eq!(f0 <= f1, (d0 <= d1) && (c0 <= c1));
            assert_eq!(f0 == f1, (d0 == d1) && (c0 == c1));
            assert_eq!(f0 >= f1, (d0 >= d1) && (c0 >= c1));
            assert_eq!(f0 > f1, (d0 > d1) && (c0 > c1));
        }
    }
}

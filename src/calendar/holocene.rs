//https://en.wikipedia.org/wiki/Holocene_calendar

use crate::calendar::gregorian::Gregorian;
use crate::calendar::gregorian::GregorianMonth;
use crate::common::date::CommonDate;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;

use num_traits::FromPrimitive;

const HOLOCENE_YEAR_OFFSET: i16 = -10000;

pub type HoloceneMonth = GregorianMonth;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Holocene(CommonDate);

impl Holocene {
    pub fn year(self) -> i32 {
        self.0.year
    }

    pub fn month(self) -> HoloceneMonth {
        HoloceneMonth::from_u8(self.0.month).expect("Will not allow setting invalid value.")
    }

    pub fn day(self) -> u8 {
        self.0.day
    }

    pub fn is_leap(h_year: i32) -> bool {
        Gregorian::is_leap(h_year) //10000 is divisible by 400, so it's ok
    }
}

impl CalculatedBounds for Holocene {}

impl Epoch for Holocene {
    fn epoch() -> Fixed {
        Gregorian::new_year(HOLOCENE_YEAR_OFFSET + 1)
    }
}

impl FromFixed for Holocene {
    fn from_fixed(date: Fixed) -> Holocene {
        let result = Gregorian::from_fixed(date).to_common_date();
        Holocene(CommonDate::new(
            result.year - (HOLOCENE_YEAR_OFFSET as i32),
            result.month,
            result.day,
        ))
    }
}

impl ToFixed for Holocene {
    fn to_fixed(self) -> Fixed {
        let g = Gregorian::try_from_common_date(CommonDate::new(
            self.0.year + (HOLOCENE_YEAR_OFFSET as i32),
            self.0.month,
            self.0.day,
        ))
        .expect("Same month/day rules");
        g.to_fixed()
    }
}

impl ToFromCommonDate for Holocene {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        Gregorian::valid_month_day(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bound::BoundedDayCount;
    use crate::common::bound::EffectiveBound;

    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use crate::day_count::RataDie;
    const MAX_YEARS: i32 = ((FIXED_MAX / 365.25) - 10000.0) as i32;

    use proptest::proptest;

    #[test]
    fn bounds_actually_work() {
        assert!(
            Holocene::from_fixed(Fixed::effective_min()) < Holocene::from_fixed(Fixed::cast_new(0))
        );
        assert!(
            Holocene::from_fixed(Fixed::effective_max()) > Holocene::from_fixed(Fixed::cast_new(0))
        );
    }

    #[test]
    fn date_of_proposal() {
        let dh = CommonDate {
            year: 11993,
            month: 12,
            day: 30,
        };
        let dg = CommonDate {
            year: 1993,
            month: 12,
            day: 30,
        };
        let fh = Holocene::try_from_common_date(dh).unwrap().to_fixed();
        let fg = Gregorian::try_from_common_date(dg).unwrap().to_fixed();
        assert_eq!(fh, fg);
    }

    proptest! {
        #[test]
        fn roundtrip(t in FIXED_MIN..FIXED_MAX) {
            let t0 = RataDie::new(t).to_fixed().to_day();
            let r = Holocene::from_fixed(t0);
            let t1 = r.to_fixed();
            assert_eq!(t0, t1);
        }

        #[test]
        fn locked_to_gregorian(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..28) {
            let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
            let a = Holocene::try_from_common_date(d).unwrap();
            let e = Gregorian::try_from_common_date(d).unwrap();
            let fa = a.to_fixed();
            let fe = e.to_fixed();
            let diff_f = fa.get_day_i() - fe.get_day_i();
            let diff_e = Holocene::epoch().get_day_i() - Gregorian::epoch().get_day_i();
            assert_eq!(diff_f, diff_e);
        }

        #[test]
        fn locked_to_gregorian_alt(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..28) {
            let dh = CommonDate{ year: year, month: month as u8, day: day as u8 };
            let dg = CommonDate{ year: year - 10000, month: month as u8, day: day as u8 };
            let fh = Holocene::try_from_common_date(dh).unwrap().to_fixed();
            let fg = Gregorian::try_from_common_date(dg).unwrap().to_fixed();
            assert_eq!(fh, fg);
        }

        #[test]
        fn invalid_common(year in -MAX_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 32..u8::MAX) {
            let d_list = [
                CommonDate{ year, month, day },
                CommonDate{ year, month: 1, day},
                CommonDate{ year, month, day: 1 },
                CommonDate{ year, month: 1, day: 0},
                CommonDate{ year, month: 0, day: 1 }
            ];
            for d in d_list {
                assert!(Holocene::try_from_common_date(d).is_err());
            }
        }

        #[test]
        fn consistent_order(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
            let f0 = Fixed::new(t0);
            let f1 = Fixed::new(t1);
            let d0 = Holocene::from_fixed(f0);
            let d1 = Holocene::from_fixed(f1);
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
            let d0 = Holocene::from_fixed(f0);
            let d1 = Holocene::from_fixed(f1);
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

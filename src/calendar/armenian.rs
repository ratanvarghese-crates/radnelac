use crate::calendar::egyptian::Egyptian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::RataDie;
use crate::day_count::ToFixed;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const ARMENIAN_EPOCH_RD: i32 = 201443;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum ArmenianMonth {
    Nawasardi = 1,
    Hori,
    Sahmi,
    Tre,
    Kaloch,
    Arach,
    Mehekani,
    Areg,
    Ahekani,
    Mareri,
    Margach,
    Hrotich,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Armenian(CommonDate);

impl Armenian {
    pub fn year(self) -> i32 {
        self.0.year
    }

    pub fn month(self) -> Option<ArmenianMonth> {
        if self.0.month == 13 {
            None
        } else {
            ArmenianMonth::from_u8(self.0.month)
        }
    }

    pub fn day(self) -> u8 {
        self.0.day
    }
}

impl CalculatedBounds for Armenian {}

impl Epoch for Armenian {
    fn epoch() -> Fixed {
        RataDie::new(ARMENIAN_EPOCH_RD as f64).to_fixed()
    }
}

impl FromFixed for Armenian {
    fn from_fixed(date: Fixed) -> Armenian {
        let f = Fixed::new(date.get() + Egyptian::epoch().get() - Armenian::epoch().get());
        Armenian::try_from_common_date(Egyptian::from_fixed(f).to_common_date())
            .expect("Same month/day validity")
    }
}

impl ToFixed for Armenian {
    fn to_fixed(self) -> Fixed {
        let e =
            Egyptian::try_from_common_date(self.to_common_date()).expect("Same month/day validity");
        Fixed::new(Armenian::epoch().get() + e.to_fixed().get() - Egyptian::epoch().get())
    }
}

impl ToFromCommonDate for Armenian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        Egyptian::valid_month_day(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bound::EffectiveBound;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use proptest::proptest;
    const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

    #[test]
    fn bounds_actually_work() {
        assert!(
            Armenian::from_fixed(Fixed::effective_min()) < Armenian::from_fixed(Fixed::cast_new(0))
        );
        assert!(
            Armenian::from_fixed(Fixed::effective_max()) > Armenian::from_fixed(Fixed::cast_new(0))
        );
    }

    proptest! {
        #[test]
        fn roundtrip(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
            let e0 = Armenian::try_from_common_date(d).unwrap();
            let e1 = Armenian::from_fixed(e0.to_fixed());
            assert_eq!(e0, e1);
        }

        #[test]
        fn roundtrip_month13(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
            let d = CommonDate{ year: year, month: 13, day: day as u8 };
            let e0 = Armenian::try_from_common_date(d).unwrap();
            let e1 = Armenian::from_fixed(e0.to_fixed());
            assert_eq!(e0, e1);
        }

        #[test]
        fn month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
            let e0 = Armenian::try_from_common_date(d).unwrap();
            assert!(e0.month().is_some());
            assert_eq!(e0.to_common_date(), d);
        }

        #[test]
        fn month_is_none(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
            let d = CommonDate{ year: year, month: 13, day: day as u8 };
            let e0 = Armenian::try_from_common_date(d).unwrap();
            assert!(e0.month().is_none());
            assert_eq!(e0.to_common_date(), d);
        }

        #[test]
        fn locked_to_egyptian(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
            let a = Armenian::try_from_common_date(d).unwrap();
            let e = Egyptian::try_from_common_date(d).unwrap();
            let fa = a.to_fixed();
            let fe = e.to_fixed();
            let diff_f = fa.get() - fe.get();
            let diff_e = Armenian::epoch().get() - Egyptian::epoch().get();
            assert_eq!(diff_f, diff_e);
        }

        #[test]
        fn invalid_common(year in -MAX_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
            let d_list = [
                CommonDate{ year, month, day },
                CommonDate{ year, month: 1, day},
                CommonDate{ year, month, day: 1 },
                CommonDate{ year, month: 1, day: 0},
                CommonDate{ year, month: 0, day: 1 }
            ];
            for d in d_list {
                assert!(Armenian::try_from_common_date(d).is_err());
            }
        }
    }
}

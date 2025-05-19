use crate::calendar::egyptian::Egyptian;
use crate::common::bound::BoundedDayCount;
use crate::common::bound::EffectiveBound;
use crate::common::date::CommonDate;
use crate::common::date::ToCommonDate;
use crate::common::date::TryFromCommonDate;
use crate::common::error::CalendarError;
use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;
use crate::day_count::rd::RataDie;
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
        RataDie::new(ARMENIAN_EPOCH_RD).to_fixed()
    }
}

impl FromFixed for Armenian {
    fn from_fixed(date: Fixed) -> Armenian {
        // Deliberately diverging from Calendrical Calculations to avoid crossing bounds checks
        let e =
            Egyptian::from_fixed_generic_unchecked(date.get_day_i(), Armenian::epoch().get_day_i());
        Armenian(e)
    }
}

impl ToFixed for Armenian {
    fn to_fixed(self) -> Fixed {
        // Deliberately diverging from Calendrical Calculations to avoid crossing bounds checks
        let e = Egyptian::to_fixed_generic_unchecked(self.0, Armenian::epoch().get_day_i());
        Fixed::cast_new(e).expect("TODO: verify")
    }
}

impl ToCommonDate for Armenian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }
}

impl TryFromCommonDate for Armenian {
    fn try_from_common_date(date: CommonDate) -> Result<Self, CalendarError> {
        if date.month > 13 {
            Err(CalendarError::InvalidMonth)
        } else if date.month < 13 && date.day > 30 {
            Err(CalendarError::InvalidDay)
        } else if date.month == 13 && date.day > 5 {
            Err(CalendarError::InvalidDay)
        } else {
            let e = Armenian(date);
            if e < Armenian::effective_min() || e > Armenian::effective_max() {
                Err(CalendarError::OutOfBounds)
            } else {
                Ok(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::math::EFFECTIVE_MAX;
    use proptest::proptest;
    const MAX_YEARS: i32 = (EFFECTIVE_MAX / 365.25) as i32;

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
            let diff_f = fa.get_day_i() - fe.get_day_i();
            let diff_e = Armenian::epoch().get_day_i() - Egyptian::epoch().get_day_i();
            assert_eq!(diff_f, diff_e);
        }
    }
}

use crate::common::bound::BoundedDayCount;
use crate::common::bound::EffectiveBound;
use crate::common::date::CommonDate;
use crate::common::date::ToCommonDate;
use crate::common::date::TryFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;
use crate::day_count::jd::JulianDay;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const NABONASSAR_ERA_JD: i32 = 1448638;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum EgyptianMonth {
    Thoth = 1,
    Phaophi,
    Athyr,
    Choiak,
    Tybi,
    Mechir,
    Phamenoth,
    Pharmuthi,
    Pachon,
    Payni,
    Epiphi,
    Mesori,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Egyptian(CommonDate);

impl Egyptian {
    pub fn year(self) -> i32 {
        self.0.year
    }

    pub fn month(self) -> Option<EgyptianMonth> {
        if self.0.month == 13 {
            None
        } else {
            EgyptianMonth::from_u8(self.0.month)
        }
    }

    pub fn day(self) -> u8 {
        self.0.day
    }

    pub fn from_fixed_generic_unchecked(date: i64, epoch: i64) -> CommonDate {
        let days = date - epoch;
        let year = days.div_euclid(365) + 1;
        let month = days.modulus(365).div_euclid(30) + 1;
        let day = days - (365 * (year - 1)) - (30 * (month - 1)) + 1;
        CommonDate {
            year: year as i32,
            month: month as u8,
            day: day as u8,
        }
    }

    pub fn to_fixed_generic_unchecked(date: CommonDate, epoch: i64) -> i64 {
        let year = date.year as i64;
        let month = date.month as i64;
        let day = date.day as i64;
        let offset = (365 * (year - 1)) + (30 * (month - 1)) + day - 1;
        (epoch as i64) + offset
    }
}

impl CalculatedBounds for Egyptian {}

impl Epoch for Egyptian {
    fn epoch() -> Fixed {
        JulianDay::new(NABONASSAR_ERA_JD).to_fixed()
    }
}

impl FromFixed for Egyptian {
    fn from_fixed(date: Fixed) -> Egyptian {
        Egyptian(Egyptian::from_fixed_generic_unchecked(
            date.get_day_i(),
            Egyptian::epoch().get_day_i(),
        ))
    }
}

impl ToFixed for Egyptian {
    fn to_fixed(self) -> Fixed {
        Fixed::cast_new(Egyptian::to_fixed_generic_unchecked(
            self.0,
            Egyptian::epoch().get_day_i(),
        ))
        .expect("TODO: verify")
    }
}

impl ToCommonDate for Egyptian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }
}

impl TryFromCommonDate for Egyptian {
    fn try_from_common_date(date: CommonDate) -> Result<Self, CalendarError> {
        if date.month > 13 {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.month < 13 && date.day > 30 {
            Err(CalendarError::InvalidDay)
        } else if date.month == 13 && date.day > 5 {
            Err(CalendarError::InvalidDay)
        } else {
            let e = Egyptian(date);
            if e < Egyptian::effective_min() || e > Egyptian::effective_max() {
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
            let e0 = Egyptian::try_from_common_date(d).unwrap();
            let e1 = Egyptian::from_fixed(e0.to_fixed());
            assert_eq!(e0, e1);
        }

        #[test]
        fn roundtrip_month13(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
            let d = CommonDate{ year: year, month: 13, day: day as u8 };
            let e0 = Egyptian::try_from_common_date(d).unwrap();
            let e1 = Egyptian::from_fixed(e0.to_fixed());
            assert_eq!(e0, e1);
        }

        #[test]
        fn month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
            let e0 = Egyptian::try_from_common_date(d).unwrap();
            assert!(e0.month().is_some());
            assert_eq!(e0.to_common_date(), d);
        }

        #[test]
        fn month_is_none(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
            let d = CommonDate{ year: year, month: 13, day: day as u8 };
            let e0 = Egyptian::try_from_common_date(d).unwrap();
            assert!(e0.month().is_none());
            assert_eq!(e0.to_common_date(), d);
        }
    }
}

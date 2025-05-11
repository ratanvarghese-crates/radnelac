use crate::calendar::common::CommonDate;
use crate::calendar::common::ValidCommonDate;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::epoch::jd::JulianDate;
use crate::error::CalendarError;
use crate::math::modulus_i;
use std::num::NonZero;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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
pub struct EgyptianDate(CommonDate);

impl EgyptianDate {
    fn get_month(self) -> Option<EgyptianMonth> {
        match self.0.month {
            1 => Some(EgyptianMonth::Thoth),
            2 => Some(EgyptianMonth::Phaophi),
            3 => Some(EgyptianMonth::Athyr),
            4 => Some(EgyptianMonth::Choiak),
            5 => Some(EgyptianMonth::Tybi),
            6 => Some(EgyptianMonth::Mechir),
            7 => Some(EgyptianMonth::Phamenoth),
            8 => Some(EgyptianMonth::Pharmuthi),
            9 => Some(EgyptianMonth::Pachon),
            10 => Some(EgyptianMonth::Payni),
            11 => Some(EgyptianMonth::Epiphi),
            12 => Some(EgyptianMonth::Mesori),
            _ => None,
        }
    }
}

impl Epoch<FixedDate> for EgyptianDate {
    fn epoch() -> FixedDate {
        const NABONASSAR_ERA_JD: f64 = 1448638.0;
        FixedDate::try_from(JulianDate(NABONASSAR_ERA_JD)).expect("Epoch is known to be in range")
    }
}

impl ValidCommonDate for EgyptianDate {
    fn is_valid(date: CommonDate) -> bool {
        let a = date.month > 0;
        let b = date.month < 13 && date.day < 31;
        let c = date.month == 13 && date.day < 6;
        a && (b || c)
    }
}

impl From<EgyptianDate> for CommonDate {
    fn from(date: EgyptianDate) -> CommonDate {
        return date.0;
    }
}

impl TryFrom<CommonDate> for EgyptianDate {
    type Error = CalendarError;
    fn try_from(date: CommonDate) -> Result<EgyptianDate, CalendarError> {
        if EgyptianDate::is_valid(date) {
            Ok(EgyptianDate(date))
        } else {
            Err(CalendarError::OutOfBounds)
        }
    }
}

impl From<EgyptianDate> for FixedDate {
    fn from(date: EgyptianDate) -> FixedDate {
        let year = date.0.year as i64;
        let month = date.0.month as i64;
        let day = date.0.day as i64;
        let offset = (365 * (year - 1)) + (30 * (month - 1)) + day - 1;
        let result = (EgyptianDate::epoch().0 as i64) + offset;
        FixedDate(result as i32)
    }
}

impl TryFrom<FixedDate> for EgyptianDate {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<EgyptianDate, Self::Error> {
        let nz365 = NonZero::new(365).expect("365 is known to be non-zero");

        let days = date.0 - EgyptianDate::epoch().0;
        let year = (((days as f64) / 365.0).floor() + 1.0) as i32;
        let month = ((modulus_i(days, nz365) as f64 / 30.0).floor() + 1.0) as i32;
        let day = days - (365 * (year - 1)) - (30 * (month - 1)) + 1;
        Ok(EgyptianDate(CommonDate {
            year: year as i16,
            month: month as u8,
            day: day as u8,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    proptest! {
        #[test]
        fn egyptian_roundtrip(year: i16, month in 1..12, day in 1..30) {
            let e0 = EgyptianDate::try_from(CommonDate {
                year,
                month: month as u8,
                day: day as u8
            }).unwrap();
            let e1 = EgyptianDate::try_from(FixedDate::try_from(e0).unwrap()).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn egyptian_roundtrip_month13(year: i16, day in 1..5) {
            let e0 = EgyptianDate::try_from(CommonDate {
                year,
                month: 13 as u8,
                day: day as u8
            }).unwrap();
            let e1 = EgyptianDate::try_from(FixedDate::try_from(e0).unwrap()).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn egyptian_month_is_some(year: i16, month in 1..12, day in 1..30) {
            let e0 = EgyptianDate::try_from(CommonDate {
                year,
                month: month as u8,
                day: day as u8
            }).unwrap();
            assert!(e0.get_month().is_some());
        }

        #[test]
        fn egyptian_roundtrip_is_none(year: i16, day in 1..5) {
            let e0 = EgyptianDate::try_from(CommonDate {
                year,
                month: 13 as u8,
                day: day as u8
            }).unwrap();
            assert!(e0.get_month().is_none())
        }
    }
}

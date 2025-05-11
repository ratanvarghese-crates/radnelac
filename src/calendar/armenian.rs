use crate::calendar::common::CommonDate;
use crate::calendar::common::ValidCommonDate;
use crate::calendar::egyptian::*;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;
use crate::epoch::rd::RataDie;
use crate::error::CalendarError;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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
pub struct ArmenianDate(CommonDate);

impl ArmenianDate {
    fn get_month(self) -> Option<ArmenianMonth> {
        match self.0.month {
            1 => Some(ArmenianMonth::Nawasardi),
            2 => Some(ArmenianMonth::Hori),
            3 => Some(ArmenianMonth::Sahmi),
            4 => Some(ArmenianMonth::Tre),
            5 => Some(ArmenianMonth::Kaloch),
            6 => Some(ArmenianMonth::Arach),
            7 => Some(ArmenianMonth::Mehekani),
            8 => Some(ArmenianMonth::Areg),
            9 => Some(ArmenianMonth::Ahekani),
            10 => Some(ArmenianMonth::Mareri),
            11 => Some(ArmenianMonth::Margach),
            12 => Some(ArmenianMonth::Hrotich),
            _ => None,
        }
    }
}

impl Epoch<FixedDate> for ArmenianDate {
    fn epoch() -> FixedDate {
        FixedDate::try_from(FixedMoment::from(RataDie(201443.0)))
            .expect("Epoch is known to be in range")
    }
}

impl ValidCommonDate for ArmenianDate {
    fn is_valid(date: CommonDate) -> bool {
        EgyptianDate::is_valid(date)
    }
}

impl From<ArmenianDate> for CommonDate {
    fn from(date: ArmenianDate) -> CommonDate {
        return date.0;
    }
}

impl TryFrom<CommonDate> for ArmenianDate {
    type Error = CalendarError;
    fn try_from(date: CommonDate) -> Result<ArmenianDate, CalendarError> {
        if ArmenianDate::is_valid(date) {
            Ok(ArmenianDate(date))
        } else {
            Err(CalendarError::OutOfBounds)
        }
    }
}

impl From<ArmenianDate> for FixedDate {
    fn from(date: ArmenianDate) -> FixedDate {
        let e = FixedDate::from(EgyptianDate::try_from(date.0).expect("Same validity rules"));
        FixedDate(ArmenianDate::epoch().0 + e.0 - EgyptianDate::epoch().0)
    }
}

impl TryFrom<FixedDate> for ArmenianDate {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<ArmenianDate, Self::Error> {
        let e = EgyptianDate::try_from(FixedDate(
            date.0 + EgyptianDate::epoch().0 - ArmenianDate::epoch().0,
        ))?;
        Ok(ArmenianDate(CommonDate::from(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    proptest! {
        #[test]
        fn armenian_roundtrip(year in i16::MIN..i16::MAX, month in 1..12, day in 1..30) {
            let e0 = ArmenianDate::try_from(CommonDate {
                year,
                month: month as u8,
                day: day as u8
            }).unwrap();
            let e1 = ArmenianDate::try_from(FixedDate::try_from(e0).unwrap()).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn armenian_roundtrip_month13(year in i16::MIN..i16::MAX, day in 1..5) {
            let e0 = ArmenianDate::try_from(CommonDate {
                year,
                month: 13 as u8,
                day: day as u8
            }).unwrap();
            let e1 = ArmenianDate::try_from(FixedDate::try_from(e0).unwrap()).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn armenian_month_is_some(year in i16::MIN..i16::MAX, month in 1..12, day in 1..30) {
            let e0 = ArmenianDate::try_from(CommonDate {
                year,
                month: month as u8,
                day: day as u8
            }).unwrap();
            assert!(e0.get_month().is_some());
        }

        #[test]
        fn armenian_roundtrip_is_none(year in i16::MIN..i16::MAX, day in 1..5) {
            let e0 = ArmenianDate::try_from(CommonDate {
                year,
                month: 13 as u8,
                day: day as u8
            }).unwrap();
            assert!(e0.get_month().is_none())
        }

    }
}

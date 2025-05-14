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
pub struct Armenian(CommonDate);

impl Armenian {
    fn get_month(self) -> Option<ArmenianMonth> {
        match self.0.get_month() {
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

impl Epoch for Armenian {
    fn epoch() -> FixedDate {
        FixedDate::from(FixedMoment::from(RataDie::from(201443)))
    }
}

impl ValidCommonDate for Armenian {
    fn is_valid(date: CommonDate) -> bool {
        Egyptian::is_valid(date)
    }
}

impl From<Armenian> for CommonDate {
    fn from(date: Armenian) -> CommonDate {
        return date.0;
    }
}

impl TryFrom<CommonDate> for Armenian {
    type Error = CalendarError;
    fn try_from(date: CommonDate) -> Result<Armenian, CalendarError> {
        if Armenian::is_valid(date) {
            Ok(Armenian(date))
        } else {
            Err(CalendarError::OutOfBounds)
        }
    }
}

impl From<Armenian> for FixedDate {
    fn from(date: Armenian) -> FixedDate {
        let e = FixedDate::from(Egyptian::try_from(date.0).expect("Same field limits"));
        let result = (Armenian::epoch() - Egyptian::epoch()) + i64::from(e);
        FixedDate::try_from(result).expect("CommonDate enforces year limits")
    }
}

impl TryFrom<FixedDate> for Armenian {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<Armenian, Self::Error> {
        let d = date + (Egyptian::epoch() - Armenian::epoch());
        let e = Egyptian::try_from(FixedDate::try_from(d)?)?;
        Ok(Armenian(CommonDate::from(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::common::MAX_YEARS;
    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate::try_new(year, month as u8, day as u8).unwrap();
            let e0 = Armenian::try_from(d).unwrap();
            let e1 = Armenian::try_from(FixedDate::from(e0)).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn roundtrip_month13(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
            let d = CommonDate::try_new(year, 13, day as u8).unwrap();
            let e0 = Armenian::try_from(d).unwrap();
            let e1 = Armenian::try_from(FixedDate::from(e0)).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate::try_new(year, month as u8, day as u8).unwrap();
            let e0 = Armenian::try_from(d).unwrap();
            assert!(e0.get_month().is_some());
            assert_eq!(CommonDate::from(e0), d);
        }

        #[test]
        fn roundtrip_is_none(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
            let d = CommonDate::try_new(year, 13, day as u8).unwrap();
            let e0 = Armenian::try_from(d).unwrap();
            assert!(e0.get_month().is_none());
            assert_eq!(CommonDate::from(e0), d);
        }
    }
}

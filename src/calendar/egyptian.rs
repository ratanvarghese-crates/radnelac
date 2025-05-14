use crate::calendar::common::CommonDate;
use crate::calendar::common::ValidCommonDate;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::epoch::jd::JulianDate;
use crate::error::CalendarError;
use crate::math::TermNum;

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
        match self.0.get_month() {
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

impl Epoch for EgyptianDate {
    fn epoch() -> FixedDate {
        const NABONASSAR_ERA_JD: f64 = 1448638.0;
        FixedDate::try_from(JulianDate(NABONASSAR_ERA_JD))
            .expect("Epoch known to be within bounds.")
    }
}

impl ValidCommonDate for EgyptianDate {
    fn is_valid(date: CommonDate) -> bool {
        let a = date.get_month() > 0;
        let b = date.get_month() < 13 && date.get_day() < 31;
        let c = date.get_month() == 13 && date.get_day() < 6;
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
        let year = date.0.get_year() as i64;
        let month = date.0.get_month() as i64;
        let day = date.0.get_day() as i64;
        let offset = (365 * (year - 1)) + (30 * (month - 1)) + day - 1;
        let result = EgyptianDate::epoch() + offset;
        FixedDate::try_from(result as i64).expect("CommonDate enforces year limits")
    }
}

impl TryFrom<FixedDate> for EgyptianDate {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<EgyptianDate, Self::Error> {
        let days = date - EgyptianDate::epoch();
        let year = days.div_euclid(365) + 1;
        let month = days.modulus(365).div_euclid(30) + 1;
        let day = days - (365 * (year - 1)) - (30 * (month - 1)) + 1;
        Ok(EgyptianDate(CommonDate::try_new(
            year as i32,
            month as u8,
            day as u8,
        )?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::common::MAX_YEARS;
    use proptest::proptest;

    proptest! {
        #[test]
        fn egyptian_roundtrip(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate::try_new(year, month as u8, day as u8).unwrap();
            let e0 = EgyptianDate::try_from(d).unwrap();
            let e1 = EgyptianDate::try_from(FixedDate::from(e0)).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn egyptian_roundtrip_month13(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
            let d = CommonDate::try_new(year, 13, day as u8).unwrap();
            let e0 = EgyptianDate::try_from(d).unwrap();
            let e1 = EgyptianDate::try_from(FixedDate::from(e0)).unwrap();
            assert_eq!(e0, e1);
        }

        #[test]
        fn egyptian_month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate::try_new(year, month as u8, day as u8).unwrap();
            let e0 = EgyptianDate::try_from(d).unwrap();
            assert!(e0.get_month().is_some());
            assert_eq!(CommonDate::from(e0), d);
        }

        #[test]
        fn egyptian_roundtrip_is_none(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
            let d = CommonDate::try_new(year, 13, day as u8).unwrap();
            let e0 = EgyptianDate::try_from(d).unwrap();
            assert!(e0.get_month().is_none());
            assert_eq!(CommonDate::from(e0), d);
        }
    }
}

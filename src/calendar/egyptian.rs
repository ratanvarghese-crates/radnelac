use crate::calendar::common::CommonDate;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::epoch::jd::JulianDate;
use crate::math::modulus;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct EgyptianDate(pub CommonDate);

impl Epoch for EgyptianDate {
    type Output = FixedDate;
    fn epoch() -> FixedDate {
        FixedDate::from(JulianDate(1448638.0)) //Nabonassar era
    }
}

impl From<EgyptianDate> for FixedDate {
    fn from(date: EgyptianDate) -> FixedDate {
        let year = date.0.year as f64;
        let month = date.0.month as f64;
        let day = date.0.day as f64;
        let offset = (365.0 * (year - 1.0)) + (30.0 * (month - 1.0)) + day - 1.0;
        EgyptianDate::epoch() + FixedDate(offset)
    }
}

impl From<FixedDate> for EgyptianDate {
    fn from(date: FixedDate) -> EgyptianDate {
        let days = (date - EgyptianDate::epoch()).0;
        let year = (days / 365.0).floor() + 1.0;
        let month = (modulus(days, 365.0) / 30.0).floor() + 1.0;
        let day = days - (365.0 * (year - 1.0)) - (30.0 * (month - 1.0)) + 1.0;
        EgyptianDate(CommonDate {
            year: year as i32,
            month: month as u8,
            day: day as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egyptian_roundtrip() {
        let e0 = EgyptianDate(CommonDate {
            year: 747,
            month: 6,
            day: 29,
        });
        let e1 = EgyptianDate::from(FixedDate::from(e0));
        assert_eq!(e0, e1);
    }
}

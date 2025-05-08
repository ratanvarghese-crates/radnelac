use crate::epoch::Epoch;
use crate::epoch::FixedDate;
use crate::epoch::JulianDate;
use crate::math::modulus;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct EgyptianDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Epoch for EgyptianDate {
    type Output = FixedDate;
    fn epoch() -> FixedDate {
        FixedDate::from(JulianDate(1448638.0)) //Nabonassar era
    }
}

impl From<EgyptianDate> for FixedDate {
    fn from(date: EgyptianDate) -> FixedDate {
        let year = date.year as f64;
        let month = date.month as f64;
        let day = date.day as f64;
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
        EgyptianDate {
            year: year as i32,
            month: month as u8,
            day: day as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egyptian_roundtrip() {
        let e0 = EgyptianDate {
            year: 747,
            month: 6,
            day: 29,
        };
        let e1 = EgyptianDate::from(FixedDate::from(e0));
        assert_eq!(e0, e1);
    }
}

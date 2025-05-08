use crate::calendar::egyptian::*;
use crate::epoch::Epoch;
use crate::epoch::FixedDate;
use crate::epoch::FixedMoment;
use crate::epoch::RataDie;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ArmenianDate {
    year: i32,
    month: u8,
    day: u8,
}

impl Epoch for ArmenianDate {
    type Output = FixedDate;
    fn epoch() -> FixedDate {
        FixedDate::from(FixedMoment::from(RataDie(201443.0)))
    }
}

impl From<ArmenianDate> for FixedDate {
    fn from(date: ArmenianDate) -> FixedDate {
        let e = FixedDate::from(EgyptianDate {
            year: date.year,
            month: date.month,
            day: date.day,
        });
        ArmenianDate::epoch() + e - EgyptianDate::epoch()
    }
}

impl From<FixedDate> for ArmenianDate {
    fn from(date: FixedDate) -> ArmenianDate {
        let e = EgyptianDate::from(date + EgyptianDate::epoch() - ArmenianDate::epoch());
        ArmenianDate {
            year: e.year,
            month: e.month,
            day: e.day,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armenian_roundtrip() {
        let a0 = ArmenianDate {
            year: 747,
            month: 6,
            day: 29,
        };
        let a1 = ArmenianDate::from(FixedDate::from(a0));
        assert_eq!(a0, a1);
    }
}

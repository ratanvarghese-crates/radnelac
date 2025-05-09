use crate::calendar::common::CommonDate;
use crate::calendar::egyptian::*;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;
use crate::epoch::rd::RataDie;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ArmenianDate(pub CommonDate);

impl Epoch for ArmenianDate {
    type Output = FixedDate;
    fn epoch() -> FixedDate {
        FixedDate::from(FixedMoment::from(RataDie(201443.0)))
    }
}

impl From<ArmenianDate> for FixedDate {
    fn from(date: ArmenianDate) -> FixedDate {
        let e = FixedDate::from(EgyptianDate(date.0));
        ArmenianDate::epoch() + e - EgyptianDate::epoch()
    }
}

impl From<FixedDate> for ArmenianDate {
    fn from(date: FixedDate) -> ArmenianDate {
        let e = EgyptianDate::from(date + EgyptianDate::epoch() - ArmenianDate::epoch());
        ArmenianDate(e.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armenian_roundtrip() {
        let a0 = ArmenianDate(CommonDate {
            year: 747,
            month: 6,
            day: 29,
        });
        let a1 = ArmenianDate::from(FixedDate::from(a0));
        assert_eq!(a0, a1);
    }
}

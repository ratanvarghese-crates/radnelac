use crate::calendar::common::CommonDate;
use crate::calendar::common::ValidCommonDate;
use crate::calendar::coptic::Coptic;
use crate::calendar::julian::Julian;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::error::CalendarError;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum EthiopicMonth {
    Maskaram = 1,
    Teqemt,
    Hedar,
    Takhsas,
    Ter,
    Yakatit,
    Magabit,
    Miyazya,
    Genbot,
    Sane,
    Hamle,
    Nahase,
    Paguemen,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Ethiopic(CommonDate);

impl TryFrom<u8> for EthiopicMonth {
    type Error = CalendarError;
    fn try_from(m: u8) -> Result<EthiopicMonth, CalendarError> {
        match m {
            1 => Ok(EthiopicMonth::Maskaram),
            2 => Ok(EthiopicMonth::Teqemt),
            3 => Ok(EthiopicMonth::Hedar),
            4 => Ok(EthiopicMonth::Takhsas),
            5 => Ok(EthiopicMonth::Ter),
            6 => Ok(EthiopicMonth::Yakatit),
            7 => Ok(EthiopicMonth::Magabit),
            8 => Ok(EthiopicMonth::Miyazya),
            9 => Ok(EthiopicMonth::Genbot),
            10 => Ok(EthiopicMonth::Sane),
            11 => Ok(EthiopicMonth::Hamle),
            12 => Ok(EthiopicMonth::Nahase),
            13 => Ok(EthiopicMonth::Paguemen),
            _ => Err(CalendarError::OutOfBounds),
        }
    }
}

impl Epoch for Ethiopic {
    fn epoch() -> FixedDate {
        FixedDate::from(
            Julian::try_from(CommonDate::new(8, 8, 29)).expect("Epoch known to be in range."),
        )
    }
}

impl ValidCommonDate for Ethiopic {
    fn is_valid(date: CommonDate) -> bool {
        Coptic::is_valid(date)
    }
}

impl From<Ethiopic> for CommonDate {
    fn from(date: Ethiopic) -> CommonDate {
        return date.0;
    }
}

impl TryFrom<CommonDate> for Ethiopic {
    type Error = CalendarError;
    fn try_from(date: CommonDate) -> Result<Ethiopic, CalendarError> {
        if Ethiopic::is_valid(date) {
            Ok(Ethiopic(date))
        } else {
            Err(CalendarError::OutOfBounds)
        }
    }
}

impl From<Ethiopic> for FixedDate {
    fn from(date: Ethiopic) -> FixedDate {
        let e = FixedDate::from(Coptic::try_from(date.0).expect("Same field limits"));
        let result = (Ethiopic::epoch() - Coptic::epoch()) + i64::from(e);
        FixedDate::try_from(result).expect("CommonDate enforces year limits")
    }
}

impl TryFrom<FixedDate> for Ethiopic {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<Ethiopic, Self::Error> {
        let d = date + (Coptic::epoch() - Ethiopic::epoch());
        let e = Coptic::try_from(FixedDate::try_from(d)?)?;
        Ok(Ethiopic(CommonDate::from(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epoch::fixed::FixedMoment;
    use crate::epoch::rd::RataDie;

    use crate::epoch::rd::MAX_RD;
    use crate::epoch::rd::MIN_RD;
    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip(t in MIN_RD..MAX_RD) {
            let t0 = FixedDate::from(FixedMoment::from(RataDie::try_from(t).unwrap()));
            let r = Ethiopic::try_from(t0).unwrap();
            let t1 = FixedDate::from(r);
            assert_eq!(t0, t1);
        }
    }
}

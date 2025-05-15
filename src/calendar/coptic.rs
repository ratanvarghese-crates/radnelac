use crate::calendar::common::CommonDate;
use crate::calendar::common::ValidCommonDate;
use crate::calendar::julian::Julian;
use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::error::CalendarError;
use crate::math::TermNum;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum CopticMonth {
    Thoout = 1,
    Paope,
    Athor,
    Koiak,
    Tobe,
    Meshir,
    Paremotep,
    Parmoute,
    Pashons,
    Paone,
    Epep,
    Mesore,
    Epagomene,
}

impl TryFrom<u8> for CopticMonth {
    type Error = CalendarError;
    fn try_from(m: u8) -> Result<CopticMonth, CalendarError> {
        match m {
            1 => Ok(CopticMonth::Thoout),
            2 => Ok(CopticMonth::Paope),
            3 => Ok(CopticMonth::Athor),
            4 => Ok(CopticMonth::Koiak),
            5 => Ok(CopticMonth::Tobe),
            6 => Ok(CopticMonth::Meshir),
            7 => Ok(CopticMonth::Paremotep),
            8 => Ok(CopticMonth::Parmoute),
            9 => Ok(CopticMonth::Pashons),
            10 => Ok(CopticMonth::Paone),
            11 => Ok(CopticMonth::Epep),
            12 => Ok(CopticMonth::Mesore),
            13 => Ok(CopticMonth::Epagomene),
            _ => Err(CalendarError::OutOfBounds),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coptic(CommonDate);

impl Coptic {
    fn is_leap(year: i32) -> bool {
        year.modulus(4) == 3
    }

    fn get_month(self) -> CopticMonth {
        CopticMonth::try_from(self.0.get_month()).expect("Won't allow setting invalid field")
    }
}

impl Epoch for Coptic {
    fn epoch() -> FixedDate {
        FixedDate::from(
            Julian::try_from(CommonDate::new(284, 8, 29)).expect("Epoch known to be in range."),
        )
    }
}

impl ValidCommonDate for Coptic {
    fn is_valid(date: CommonDate) -> bool {
        if date.get_month() <= 12 {
            date.get_day() <= 30
        } else if date.get_month() == 13 {
            if Coptic::is_leap(date.get_year()) {
                date.get_day() <= 6
            } else {
                date.get_day() <= 5
            }
        } else {
            false
        }
    }
}

impl From<Coptic> for CommonDate {
    fn from(date: Coptic) -> CommonDate {
        return date.0;
    }
}

impl TryFrom<CommonDate> for Coptic {
    type Error = CalendarError;
    fn try_from(date: CommonDate) -> Result<Coptic, CalendarError> {
        if Coptic::is_valid(date) {
            Ok(Coptic(date))
        } else {
            Err(CalendarError::OutOfBounds)
        }
    }
}

impl From<Coptic> for FixedDate {
    fn from(date: Coptic) -> FixedDate {
        let year = date.0.get_year() as i64;
        let month = date.0.get_month() as i64;
        let day = date.0.get_day() as i64;

        let result = i64::from(Coptic::epoch()) - 1
            + (365 * (year - 1))
            + year.div_euclid(4)
            + (30 * (month - 1))
            + day;
        return FixedDate::try_from(result).expect("TODO: Range enforced elsewhere????");
    }
}

impl TryFrom<FixedDate> for Coptic {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<Coptic, Self::Error> {
        let year = (4 * (date - Coptic::epoch()) + 1463).div_euclid(1461);
        let month = (i64::from(
            date - FixedDate::from(Coptic::try_from(CommonDate::try_new(year as i32, 1, 1)?)?),
        )
        .div_euclid(30))
            + 1;
        let day = date
            - FixedDate::from(Coptic::try_from(CommonDate::try_new(
                year as i32,
                month as u8,
                1,
            )?)?)
            + 1;
        Ok(Coptic(
            CommonDate::try_new(year as i32, month as u8, day as u8).expect("TODO: some reason"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::julian::JulianMonth;
    use crate::epoch::fixed::FixedMoment;
    use crate::epoch::rd::RataDie;

    use crate::epoch::rd::MAX_RD;
    use crate::epoch::rd::MIN_RD;
    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip(t in MIN_RD..MAX_RD) {
            let t0 = FixedDate::from(FixedMoment::from(RataDie::try_from(t).unwrap()));
            let r = Coptic::try_from(t0).unwrap();
            let t1 = FixedDate::from(r);
            assert_eq!(t0, t1);
        }

        #[test]
        fn christmas(y in i16::MIN..i16::MAX) {
            let c = Coptic::try_from(CommonDate::new(y, CopticMonth::Koiak as u8, 29))?;
            let j = Julian::try_from(FixedDate::from(c))?;
            assert_eq!(j.get_month(), JulianMonth::December);
            assert!(j.get_day() == 25 || j.get_day() == 26);
        }
    }
}

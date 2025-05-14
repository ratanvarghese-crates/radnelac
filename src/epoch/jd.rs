use crate::epoch::fixed::EpochMoment;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;
use crate::error::CalendarError;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;

pub const MAX_JD: f64 = EFFECTIVE_MAX + 1721424.5;
pub const MIN_JD: f64 = -MAX_JD;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDay(pub f64);

impl EpochMoment for JulianDay {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(-1721424.5).expect("Epoch known to be within bounds.")
    }
}

impl TryFrom<JulianDay> for FixedMoment {
    type Error = CalendarError;
    fn try_from(jd: JulianDay) -> Result<FixedMoment, CalendarError> {
        FixedMoment::try_from(JulianDay::epoch_moment() + jd.0)
    }
}

impl From<FixedMoment> for JulianDay {
    fn from(t: FixedMoment) -> JulianDay {
        JulianDay(t - JulianDay::epoch_moment())
    }
}

impl TryFrom<JulianDay> for FixedDate {
    type Error = CalendarError;
    fn try_from(jd: JulianDay) -> Result<FixedDate, Self::Error> {
        Ok(FixedDate::from(FixedMoment::try_from(jd)?))
    }
}

impl From<FixedDate> for JulianDay {
    fn from(t: FixedDate) -> JulianDay {
        JulianDay::from(FixedMoment::from(t))
    }
}

pub const MAX_MJD: f64 = EFFECTIVE_MAX - 678576.0;
pub const MIN_MJD: f64 = -MAX_MJD;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ModifiedJulianDay(pub f64);

impl EpochMoment for ModifiedJulianDay {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(678576.0).expect("Epoch known to be within bounds.")
    }
}

impl TryFrom<ModifiedJulianDay> for FixedMoment {
    type Error = CalendarError;
    fn try_from(mjd: ModifiedJulianDay) -> Result<FixedMoment, CalendarError> {
        FixedMoment::try_from(ModifiedJulianDay::epoch_moment() + mjd.0)
    }
}

impl From<FixedMoment> for ModifiedJulianDay {
    fn from(t: FixedMoment) -> ModifiedJulianDay {
        ModifiedJulianDay(t - ModifiedJulianDay::epoch_moment())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::TermNum;
    use proptest::proptest;

    proptest! {
        #[test]
        fn jd_roundtrip(t in MIN_JD..MAX_JD) {
            let j0 = JulianDay(t);
            let j1 = JulianDay::from(FixedMoment::try_from(j0).unwrap());
            let j2 = JulianDay::from(FixedDate::try_from(j0).unwrap());
            assert!(j0.0.approx_eq(j1.0));
            assert!((j0.0 - j2.0).abs() < 1.0);
        }

        #[test]
        fn mjd_roundtrip(t in MIN_MJD..MAX_MJD) {
            let j0 = ModifiedJulianDay(t);
            let j1 = ModifiedJulianDay::from(FixedMoment::try_from(j0).unwrap());
            assert!(j0.0.approx_eq(j1.0));
        }

        #[test]
        fn mjd_from_jd(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let x = FixedMoment::try_from(t).unwrap();
            let j0 = JulianDay::from(x);
            let mjd0 = ModifiedJulianDay::from(x);
            assert!(mjd0.0.approx_eq(j0.0 - 2400000.5));
        }
    }
}

use crate::epoch::fixed::EpochMoment;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;
use crate::error::CalendarError;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDate(pub f64);

impl EpochMoment for JulianDate {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(-1721424.5).expect("Epoch known to be within bounds.")
    }
}

impl TryFrom<JulianDate> for FixedMoment {
    type Error = CalendarError;
    fn try_from(jd: JulianDate) -> Result<FixedMoment, CalendarError> {
        FixedMoment::try_from(JulianDate::epoch_moment() + jd.0)
    }
}

impl From<FixedMoment> for JulianDate {
    fn from(t: FixedMoment) -> JulianDate {
        JulianDate(t - JulianDate::epoch_moment())
    }
}

impl TryFrom<JulianDate> for FixedDate {
    type Error = CalendarError;
    fn try_from(jd: JulianDate) -> Result<FixedDate, Self::Error> {
        Ok(FixedDate::from(FixedMoment::try_from(jd)?))
    }
}

impl From<FixedDate> for JulianDate {
    fn from(t: FixedDate) -> JulianDate {
        JulianDate::from(FixedMoment::from(t))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ModifiedJulianDate(pub f64);

impl EpochMoment for ModifiedJulianDate {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(678576.0).expect("Epoch known to be within bounds.")
    }
}

impl TryFrom<ModifiedJulianDate> for FixedMoment {
    type Error = CalendarError;
    fn try_from(mjd: ModifiedJulianDate) -> Result<FixedMoment, CalendarError> {
        FixedMoment::try_from(ModifiedJulianDate::epoch_moment() + mjd.0)
    }
}

impl From<FixedMoment> for ModifiedJulianDate {
    fn from(t: FixedMoment) -> ModifiedJulianDate {
        ModifiedJulianDate(t - ModifiedJulianDate::epoch_moment())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::TermNum;
    use crate::math::EFFECTIVE_MAX;
    use crate::math::EFFECTIVE_MIN;
    use proptest::proptest;

    proptest! {
        #[test]
        fn jd_roundtrip(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let j0 = JulianDate(t);
            let j1 = JulianDate::from(FixedMoment::try_from(j0).unwrap());
            let j2 = JulianDate::from(FixedDate::try_from(j0).unwrap());
            assert!(j0.0.approx_eq(j1.0));
            assert!((j0.0 - j2.0).abs() < 1.0);
        }

        #[test]
        fn mjd_roundtrip(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let j0 = ModifiedJulianDate(t);
            let j1 = ModifiedJulianDate::from(FixedMoment::try_from(j0).unwrap());
            assert!(j0.0.approx_eq(j1.0));
        }

        #[test]
        fn mjd_from_jd(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let x = FixedMoment::try_from(t).unwrap();
            let j0 = JulianDate::from(x);
            let mjd0 = ModifiedJulianDate::from(x);
            assert!(mjd0.0.approx_eq(j0.0 - 2400000.5));
        }
    }
}

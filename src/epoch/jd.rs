use crate::epoch::fixed::EpochMoment;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;
use crate::error::CalendarError;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;
use num_traits::Bounded;

const JD_EPOCH: f64 = -1721424.5;
pub const MAX_JD: f64 = EFFECTIVE_MAX - JD_EPOCH;
pub const MIN_JD: f64 = EFFECTIVE_MIN - JD_EPOCH;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDay(pub f64);

impl EpochMoment for JulianDay {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(JD_EPOCH).expect("Epoch known to be within bounds.")
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

impl Bounded for JulianDay {
    fn min_value() -> Self {
        JulianDay(MIN_JD)
    }

    fn max_value() -> Self {
        JulianDay(MAX_JD)
    }
}

const MJD_EPOCH: f64 = 678576.0;
pub const MAX_MJD: f64 = EFFECTIVE_MAX - MJD_EPOCH;
pub const MIN_MJD: f64 = EFFECTIVE_MIN - MJD_EPOCH;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ModifiedJulianDay(pub f64);

impl EpochMoment for ModifiedJulianDay {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(MJD_EPOCH).expect("Epoch known to be within bounds.")
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

impl Bounded for ModifiedJulianDay {
    fn min_value() -> Self {
        ModifiedJulianDay(MIN_MJD)
    }

    fn max_value() -> Self {
        ModifiedJulianDay(MAX_MJD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::TermNum;
    use proptest::proptest;

    #[test]
    fn around_jd_epoch() {
        let before = FixedMoment::try_from(JulianDay::epoch_moment() + (-1.0)).unwrap();
        let exact = FixedMoment::try_from(JulianDay::epoch_moment() + 0.0).unwrap();
        let after = FixedMoment::try_from(JulianDay::epoch_moment() + 1.0).unwrap();
        assert_eq!(JulianDay::from(before).0, -1.0);
        assert_eq!(JulianDay::from(exact).0, 0.0);
        assert_eq!(JulianDay::from(after).0, 1.0);
    }

    #[test]
    fn bounds_jd() {
        assert!(FixedMoment::try_from(JulianDay::min_value()).is_ok());
        assert!(FixedMoment::try_from(JulianDay::max_value()).is_ok());
        let beyond_min = JulianDay(JulianDay::min_value().0 - 1.0);
        let beyond_max = JulianDay(JulianDay::max_value().0 + 1.0);
        assert!(FixedMoment::try_from(beyond_min).is_err());
        assert!(FixedMoment::try_from(beyond_max).is_err());
    }

    #[test]
    fn around_mjd_epoch() {
        let before = FixedMoment::try_from(ModifiedJulianDay::epoch_moment() + (-1.0)).unwrap();
        let exact = FixedMoment::try_from(ModifiedJulianDay::epoch_moment() + 0.0).unwrap();
        let after = FixedMoment::try_from(ModifiedJulianDay::epoch_moment() + 1.0).unwrap();
        assert_eq!(ModifiedJulianDay::from(before).0, -1.0);
        assert_eq!(ModifiedJulianDay::from(exact).0, 0.0);
        assert_eq!(ModifiedJulianDay::from(after).0, 1.0);
    }

    #[test]
    fn bounds_mjd() {
        assert!(FixedMoment::try_from(ModifiedJulianDay::min_value()).is_ok());
        assert!(FixedMoment::try_from(ModifiedJulianDay::max_value()).is_ok());
        let beyond_min = ModifiedJulianDay(ModifiedJulianDay::min_value().0 - 1.0);
        let beyond_max = ModifiedJulianDay(ModifiedJulianDay::max_value().0 + 1.0);
        assert!(FixedMoment::try_from(beyond_min).is_err());
        assert!(FixedMoment::try_from(beyond_max).is_err());
    }

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

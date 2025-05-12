use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;
use crate::error::CalendarError;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDate(pub f64);

impl Epoch<FixedMoment> for JulianDate {
    fn epoch() -> FixedMoment {
        FixedMoment(-1721424.5)
    }
}

impl From<JulianDate> for FixedMoment {
    fn from(jd: JulianDate) -> FixedMoment {
        FixedMoment(jd.0 + JulianDate::epoch().0)
    }
}

impl From<FixedMoment> for JulianDate {
    fn from(t: FixedMoment) -> JulianDate {
        JulianDate(t.0 - JulianDate::epoch().0)
    }
}

impl TryFrom<JulianDate> for FixedDate {
    type Error = CalendarError;
    fn try_from(jd: JulianDate) -> Result<FixedDate, Self::Error> {
        FixedDate::try_from(FixedMoment::from(jd))
    }
}

impl From<FixedDate> for JulianDate {
    fn from(t: FixedDate) -> JulianDate {
        JulianDate::from(FixedMoment::from(t))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ModifiedJulianDate(pub f64);

impl Epoch<FixedMoment> for ModifiedJulianDate {
    fn epoch() -> FixedMoment {
        FixedMoment(678576.0)
    }
}

impl From<ModifiedJulianDate> for FixedMoment {
    fn from(mjd: ModifiedJulianDate) -> FixedMoment {
        FixedMoment(mjd.0 + ModifiedJulianDate::epoch().0)
    }
}

impl From<FixedMoment> for ModifiedJulianDate {
    fn from(t: FixedMoment) -> ModifiedJulianDate {
        ModifiedJulianDate(t.0 - ModifiedJulianDate::epoch().0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::TermNum;
    use proptest::proptest;

    proptest! {
        #[test]
        fn jd_roundtrip(t in ((i32::MIN/2) as f64)..((i32::MAX/2) as f64)) {
            let j0 = JulianDate(t);
            let j1 = JulianDate::from(FixedMoment::from(j0));
            let j2 = JulianDate::from(FixedDate::try_from(j0).unwrap());
            assert!(j0.0.approx_eq(j1.0));
            assert!((j0.0 - j2.0).abs() < 1.0);
        }

        #[test]
        fn mjd_roundtrip(t: f64) {
            let j0 = ModifiedJulianDate(t);
            let j1 = ModifiedJulianDate::from(FixedMoment::from(j0));
            assert!(j0.0.approx_eq(j1.0));
        }

        #[test]
        fn mjd_from_jd(t: f64) {
            let x = FixedMoment(t);
            let j0 = JulianDate::from(x);
            let mjd0 = ModifiedJulianDate::from(x);
            assert!(mjd0.0.approx_eq(j0.0 - 2400000.5));
        }
    }
}

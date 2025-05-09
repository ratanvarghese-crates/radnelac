use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDate(pub f64);

impl Epoch for JulianDate {
    type Output = FixedMoment;
    fn epoch() -> FixedMoment {
        FixedMoment(-1721424.5)
    }
}

impl From<JulianDate> for FixedMoment {
    fn from(jd: JulianDate) -> FixedMoment {
        FixedMoment(jd.0) + JulianDate::epoch()
    }
}

impl From<FixedMoment> for JulianDate {
    fn from(t: FixedMoment) -> JulianDate {
        JulianDate((t - JulianDate::epoch()).0)
    }
}

impl From<JulianDate> for FixedDate {
    fn from(jd: JulianDate) -> FixedDate {
        FixedDate::from(FixedMoment::from(jd))
    }
}

impl From<FixedDate> for JulianDate {
    fn from(t: FixedDate) -> JulianDate {
        JulianDate::from(FixedMoment::from(t))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ModifiedJulianDate(pub f64);

impl Epoch for ModifiedJulianDate {
    type Output = FixedMoment;
    fn epoch() -> FixedMoment {
        FixedMoment(678576.0)
    }
}

impl From<ModifiedJulianDate> for FixedMoment {
    fn from(mjd: ModifiedJulianDate) -> FixedMoment {
        FixedMoment(mjd.0) + ModifiedJulianDate::epoch()
    }
}

impl From<FixedMoment> for ModifiedJulianDate {
    fn from(t: FixedMoment) -> ModifiedJulianDate {
        ModifiedJulianDate((t - ModifiedJulianDate::epoch()).0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jd_roundtrip() {
        let j0 = JulianDate(12345678.9);
        let j1 = JulianDate::from(FixedMoment::from(j0));
        let j2 = JulianDate::from(FixedDate::from(j0));
        assert_eq!(j0, j1);
        assert_eq!(j0.0.floor(), j2.0.floor());
    }

    #[test]
    fn mjd_roundtrip() {
        let j0 = ModifiedJulianDate(12345678.9);
        let j1 = ModifiedJulianDate::from(FixedMoment::from(j0));
        assert_eq!(j0, j1);
    }

    #[test]
    fn mjd_from_jd() {
        let x = FixedMoment(12345678.0);
        let j0 = JulianDate::from(x);
        let mjd0 = ModifiedJulianDate::from(x);
        assert_eq!(mjd0.0, j0.0 - 2400000.5);
    }
}

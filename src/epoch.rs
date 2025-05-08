// Calendrical Calculations chapter 1
use crate::math::from_mixed_radix;
use crate::math::modulus;
use crate::math::to_mixed_radix;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct FixedMoment(pub f64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct FixedDate(pub f64);

impl From<FixedDate> for FixedMoment {
    fn from(date: FixedDate) -> FixedMoment {
        FixedMoment(date.0)
    }
}

impl From<FixedMoment> for FixedDate {
    fn from(t: FixedMoment) -> FixedDate {
        FixedDate(t.0.floor())
    }
}

impl Sub for FixedDate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        FixedDate(self.0 - other.0)
    }
}

impl Add for FixedDate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        FixedDate(self.0 + other.0)
    }
}

impl Sub for FixedMoment {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        FixedMoment(self.0 - other.0)
    }
}

impl Add for FixedMoment {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        FixedMoment(self.0 + other.0)
    }
}

pub trait Epoch {
    type Output: Add + Sub;
    fn epoch() -> Self::Output;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct RataDie(pub f64);

impl Epoch for RataDie {
    type Output = FixedMoment;
    fn epoch() -> FixedMoment {
        FixedMoment(0.0)
    }
}

impl From<RataDie> for FixedMoment {
    fn from(rd: RataDie) -> FixedMoment {
        FixedMoment(rd.0) + RataDie::epoch()
    }
}

impl From<FixedMoment> for RataDie {
    fn from(t: FixedMoment) -> RataDie {
        RataDie((t - RataDie::epoch()).0)
    }
}

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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct UnixMoment(pub f64);

impl Epoch for UnixMoment {
    type Output = FixedMoment;
    fn epoch() -> FixedMoment {
        FixedMoment(719163.0)
    }
}

impl From<UnixMoment> for FixedMoment {
    fn from(s: UnixMoment) -> FixedMoment {
        UnixMoment::epoch() + FixedMoment(s.0 / (24.0 * 60.0 * 60.0))
    }
}

impl From<FixedMoment> for UnixMoment {
    fn from(t: FixedMoment) -> UnixMoment {
        UnixMoment(24.0 * 60.0 * 60. * (t - UnixMoment::epoch()).0)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct TimeOfDay(pub f64);

impl From<FixedMoment> for TimeOfDay {
    fn from(t: FixedMoment) -> TimeOfDay {
        TimeOfDay(modulus(t.0, 1.0))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ClockTime {
    hours: f64,
    minutes: f32,
    seconds: f32,
}

impl From<ClockTime> for TimeOfDay {
    fn from(clock: ClockTime) -> TimeOfDay {
        let a = [clock.hours, clock.minutes as f64, clock.seconds as f64];
        let b = [24.0, 60.0, 60.0];
        TimeOfDay(from_mixed_radix(&a, &b, 0.0) / 24.0)
    }
}

impl From<FixedMoment> for ClockTime {
    fn from(t: FixedMoment) -> ClockTime {
        let b = [24.0, 60.0, 60.0];
        let a = to_mixed_radix(t.0, &b, 0.0);
        ClockTime {
            hours: a[1],
            minutes: a[2] as f32,
            seconds: a[3] as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rd_is_epoch() {
        assert_eq!(RataDie(0.0), RataDie::from(FixedMoment(0.0)));
    }

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

    #[test]
    fn unix_roundtrip() {
        let unix0 = UnixMoment(1746587115.0);
        let unix1 = UnixMoment::from(FixedMoment::from(unix0));
        assert_eq!(unix0.0.floor(), unix1.0.floor());
    }

    #[test]
    fn time() {
        let j0: JulianDate = Default::default();
        assert_eq!(TimeOfDay::from(FixedMoment::from(j0)).0, 0.5);
    }
}

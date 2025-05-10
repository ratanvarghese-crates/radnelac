use crate::epoch::fixed::FixedMoment;
use crate::math::from_mixed_radix;
use crate::math::modulus;
use crate::math::to_mixed_radix;

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
        TimeOfDay(from_mixed_radix(&a, &b, 0) / 24.0)
    }
}

impl From<FixedMoment> for ClockTime {
    fn from(t: FixedMoment) -> ClockTime {
        let b = [24.0, 60.0, 60.0];
        let a = to_mixed_radix(t.0, &b, 0);
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
    use crate::epoch::jd::JulianDate;

    #[test]
    fn time() {
        let j0: JulianDate = Default::default();
        assert_eq!(TimeOfDay::from(FixedMoment::from(j0)).0, 0.5);
    }
}

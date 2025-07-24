use crate::clock::ClockTime;
use crate::common::bound::BoundedDayCount;
use crate::common::math::TermNum;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;

/// Represents a clock time as a fraction of a day
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct TimeOfDay(f64);

impl TimeOfDay {
    pub fn new(t: f64) -> Self {
        TimeOfDay(t)
    }

    pub fn get(self) -> f64 {
        self.0
    }

    pub fn new_from_clock(clock: ClockTime) -> TimeOfDay {
        let a = [
            0.0,
            clock.hours as f64,
            clock.minutes as f64,
            clock.seconds as f64,
        ];
        let b = [24.0, 60.0, 60.0];
        TimeOfDay::new(TermNum::from_mixed_radix(&a, &b, 0).expect("Inputs are valid"))
    }
}

impl FromFixed for TimeOfDay {
    fn from_fixed(t: Fixed) -> TimeOfDay {
        TimeOfDay::new(t.get().modulus(1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bound::BoundedDayCount;
    use crate::day_count::JulianDay;
    use crate::day_count::ToFixed;

    #[test]
    fn time() {
        let j0: JulianDay = JulianDay::new(0.0);
        assert_eq!(j0.convert::<TimeOfDay>().0, 0.5);
    }
}

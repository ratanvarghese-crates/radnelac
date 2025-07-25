use crate::clock::ClockTime;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;

/// Represents a clock time as a fraction of a day
///
/// This is internally a floating point number, where the fractional portion represents
/// a particular time of day. For example 1.0 is midnight at the start of day 1, and 1.5 is
/// noon on day 1.
///
/// Note that equality and ordering operations are subject to limitations similar to
/// equality and ordering operations on a floating point number. Two `TimeOfDay` values represent
/// the same day or even the same second, but still appear different on the sub-second level.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct TimeOfDay(f64);

impl TimeOfDay {
    /// Create a new `TimeOfDay`
    pub fn new(t: f64) -> Self {
        TimeOfDay(t)
    }

    /// Get underlying floating point from `TimeOfDay`
    pub fn get(self) -> f64 {
        self.0
    }

    /// Aggregate `ClockTime` hours, minutes and second fields into a `TimeOfDay`
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
    use crate::day_count::BoundedDayCount;
    use crate::day_count::JulianDay;
    use crate::day_count::ToFixed;

    #[test]
    fn time() {
        let j0: JulianDay = JulianDay::new(0.0);
        assert_eq!(j0.convert::<TimeOfDay>().0, 0.5);
    }
}

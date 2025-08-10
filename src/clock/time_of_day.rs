use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::CalendarError;

/// Represents a clock time as hours, minutes and seconds
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ClockTime {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: f32,
}

impl ClockTime {
    /// Returns an error if the ClockTime is invalid.
    pub fn validate(self) -> Result<(), CalendarError> {
        if self.hours > 23 {
            Err(CalendarError::InvalidHour)
        } else if self.minutes >= 60 {
            Err(CalendarError::InvalidMinute)
        } else if self.seconds > 60.0 {
            //Allow 60.0 for leap second
            Err(CalendarError::InvalidSecond)
        } else {
            Ok(())
        }
    }

    pub fn hour_1_to_12(self) -> u8 {
        (self.hours as i64).adjusted_remainder(12) as u8
    }
}

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

    pub fn midnight() -> Self {
        TimeOfDay(0.0)
    }

    pub fn noon() -> Self {
        TimeOfDay(0.5)
    }

    /// Get underlying floating point from `TimeOfDay`
    pub fn get(self) -> f64 {
        self.0
    }

    /// Split `TimeOfDay` into hours, minutes, and seconds and aggregate into `ClockTime`
    pub fn to_clock(self) -> ClockTime {
        let b = [24.0, 60.0, 60.0];
        let mut a = [0.0, 0.0, 0.0, 0.0];
        TermNum::to_mixed_radix(self.get(), &b, 0, &mut a)
            .expect("Valid inputs, other failures are impossible.");
        ClockTime {
            hours: a[1] as u8,
            minutes: a[2] as u8,
            seconds: a[3] as f32,
        }
    }

    /// Aggregate `ClockTime` hours, minutes and second fields into a `TimeOfDay`
    pub fn try_from_clock(clock: ClockTime) -> Result<Self, CalendarError> {
        clock.validate()?;
        let a = [
            0.0,
            clock.hours as f64,
            clock.minutes as f64,
            clock.seconds as f64,
        ];
        let b = [24.0, 60.0, 60.0];
        let t = TermNum::from_mixed_radix(&a, &b, 0)?;
        Ok(TimeOfDay::new(t))
    }
}

impl FromFixed for TimeOfDay {
    fn from_fixed(t: Fixed) -> TimeOfDay {
        TimeOfDay::new(t.to_time_of_day().get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::BoundedDayCount;
    use crate::day_count::JulianDay;
    use crate::day_count::ToFixed;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use proptest::proptest;

    #[test]
    fn time() {
        let j0: JulianDay = JulianDay::new(0.0);
        assert_eq!(j0.convert::<TimeOfDay>().0, 0.5);
    }

    #[test]
    fn obvious_clock_times() {
        assert_eq!(
            TimeOfDay::try_from_clock(ClockTime {
                hours: 0,
                minutes: 0,
                seconds: 0.0
            })
            .unwrap(),
            TimeOfDay::new(0.0)
        );
        assert_eq!(
            TimeOfDay::try_from_clock(ClockTime {
                hours: 0,
                minutes: 0,
                seconds: 1.0
            })
            .unwrap(),
            TimeOfDay::new(1.0 / (24.0 * 60.0 * 60.0))
        );
        assert_eq!(
            TimeOfDay::try_from_clock(ClockTime {
                hours: 0,
                minutes: 1,
                seconds: 0.0
            })
            .unwrap(),
            TimeOfDay::new(1.0 / (24.0 * 60.0))
        );
        assert_eq!(
            TimeOfDay::try_from_clock(ClockTime {
                hours: 6,
                minutes: 0,
                seconds: 0.0
            })
            .unwrap(),
            TimeOfDay::new(0.25)
        );
        assert_eq!(
            TimeOfDay::try_from_clock(ClockTime {
                hours: 12,
                minutes: 0,
                seconds: 0.0
            })
            .unwrap(),
            TimeOfDay::new(0.5)
        );
        assert_eq!(
            TimeOfDay::try_from_clock(ClockTime {
                hours: 18,
                minutes: 0,
                seconds: 0.0
            })
            .unwrap(),
            TimeOfDay::new(0.75)
        );
    }

    proptest! {
        #[test]
        fn clock_time_round_trip(ahr in 0..24,amn in 0..59,asc in 0..59) {
            let hours = ahr as u8;
            let minutes = amn as u8;
            let seconds = asc as f32;
            let c0 = ClockTime { hours, minutes, seconds };
            let t = TimeOfDay::try_from_clock(c0).unwrap();
            let c1 = t.to_clock();
            assert_eq!(c0, c1);
        }

        #[test]
        fn clock_time_from_moment(x in FIXED_MIN..FIXED_MAX) {
            let t = TimeOfDay::from_fixed(Fixed::new(x));
            let c = t.to_clock();
            c.validate().unwrap();
        }

        #[test]
        fn invalid_hour(ahr in 25..u8::MAX,amn in 0..59,asc in 0..59) {
            let c0 = ClockTime { hours: ahr as u8, minutes: amn as u8, seconds: asc as f32 };
            assert!(TimeOfDay::try_from_clock(c0).is_err());
        }

        #[test]
        fn invalid_minute(ahr in 0..59,amn in 60..u8::MAX,asc in 0..59) {
            let c0 = ClockTime { hours: ahr as u8, minutes: amn as u8, seconds: asc as f32 };
            assert!(TimeOfDay::try_from_clock(c0).is_err());
        }

        #[test]
        fn invalid_second(ahr in 0..59,amn in 0..59,asc in 61..u8::MAX) {
            let c0 = ClockTime { hours: ahr as u8, minutes: amn as u8, seconds: asc as f32 };
            assert!(TimeOfDay::try_from_clock(c0).is_err());
        }

    }
}

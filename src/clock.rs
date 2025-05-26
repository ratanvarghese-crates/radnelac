use crate::common::bound::BoundedDayCount;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct TimeOfDay(f64);

impl TimeOfDay {
    pub fn get(self) -> f64 {
        self.0
    }

    pub fn new(t: f64) -> Self {
        TimeOfDay(t)
    }
}

impl FromFixed for TimeOfDay {
    fn from_fixed(t: Fixed) -> TimeOfDay {
        TimeOfDay::new(t.get().modulus(1.0))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ClockTime {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: f32,
}

fn check_fields(hours: u8, minutes: u8, seconds: f32) -> Result<(), CalendarError> {
    if hours > 23 {
        Err(CalendarError::InvalidHour)
    } else if minutes >= 60 {
        Err(CalendarError::InvalidMinute)
    } else if seconds > 60.0 {
        //Allow 60.0 for leap second
        Err(CalendarError::InvalidSecond)
    } else {
        Ok(())
    }
}

impl ClockTime {
    pub fn set(hours: u8, minutes: u8, seconds: f32) -> Result<ClockTime, CalendarError> {
        match check_fields(hours, minutes, seconds) {
            Ok(()) => Ok(ClockTime {
                hours,
                minutes,
                seconds,
            }),
            Err(error) => Err(error),
        }
    }
}

impl Default for ClockTime {
    fn default() -> Self {
        ClockTime {
            hours: 0,
            minutes: 0,
            seconds: 0.0,
        }
    }
}

impl From<ClockTime> for TimeOfDay {
    fn from(clock: ClockTime) -> TimeOfDay {
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

impl TryFrom<TimeOfDay> for ClockTime {
    type Error = CalendarError;

    fn try_from(t: TimeOfDay) -> Result<ClockTime, CalendarError> {
        let b = [24.0, 60.0, 60.0];
        let mut a = [0.0, 0.0, 0.0, 0.0];
        TermNum::to_mixed_radix(t.0, &b, 0, &mut a)?;
        Ok(ClockTime {
            hours: a[1] as u8,
            minutes: a[2] as u8,
            seconds: a[3] as f32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::fixed::ToFixed;
    use crate::day_count::fixed::FIXED_MAX;
    use crate::day_count::fixed::FIXED_MIN;
    use crate::day_count::jd::JulianDay;
    use proptest::proptest;

    #[test]
    fn time() {
        let j0: JulianDay = JulianDay::new(0.0);
        assert_eq!(TimeOfDay::from_fixed(j0.to_fixed()).0, 0.5);
    }

    #[test]
    fn obvious_clock_times() {
        assert_eq!(
            TimeOfDay::from(ClockTime::set(0, 0, 0.0).unwrap()),
            TimeOfDay::new(0.0)
        );
        assert_eq!(
            TimeOfDay::from(ClockTime::set(0, 0, 1.0).unwrap()),
            TimeOfDay::new(1.0 / (24.0 * 60.0 * 60.0))
        );
        assert_eq!(
            TimeOfDay::from(ClockTime::set(0, 1, 0.0).unwrap()),
            TimeOfDay::new(1.0 / (24.0 * 60.0))
        );
        assert_eq!(
            TimeOfDay::from(ClockTime::set(6, 0, 0.0).unwrap()),
            TimeOfDay::new(0.25)
        );
        assert_eq!(
            TimeOfDay::from(ClockTime::set(12, 0, 0.0).unwrap()),
            TimeOfDay::new(0.5)
        );
        assert_eq!(
            TimeOfDay::from(ClockTime::set(18, 0, 0.0).unwrap()),
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
            let t = TimeOfDay::from(c0);
            let c1 = ClockTime::try_from(t).unwrap();
            assert_eq!(c0, c1);
        }

        #[test]
        fn clock_time_from_moment(x in FIXED_MIN..FIXED_MAX) {
            let t = TimeOfDay::from_fixed(Fixed::new(x));
            let c = ClockTime::try_from(t).unwrap();
            check_fields(c.hours, c.minutes, c.seconds).unwrap();
        }
    }
}

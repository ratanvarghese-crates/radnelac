use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;

const MJD_EPOCH: f64 = 678576.0;

/// Represents a Modified Julian Day Number (not to be confused with the Julian Calendar)
///
/// The Modified Julian Day Number is the count of days since midnight November 17,
/// 1858 CE in the proleptic Gregorian Calendar.
///
/// This is internally a floating point number, where the integer portion represents a
/// particular day and the fractional portion represents a particular time of day.
///
/// Note that equality and ordering operations are subject to limitations similar to
/// equality and ordering operations on a floating point number. Two `ModifiedJulianDay` values represent
/// the same day or even the same second, but still appear different on the sub-second level.
///
/// Further reading:
/// + [Wikipedia](https://en.m.wikipedia.org/wiki/Julian_day#Variants)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ModifiedJulianDay(f64);

impl CalculatedBounds for ModifiedJulianDay {}

impl FromFixed for ModifiedJulianDay {
    fn from_fixed(t: Fixed) -> ModifiedJulianDay {
        ModifiedJulianDay(t.get() - MJD_EPOCH)
    }
}

impl ToFixed for ModifiedJulianDay {
    fn to_fixed(self) -> Fixed {
        Fixed::new(MJD_EPOCH + self.0)
    }
}

impl Epoch for ModifiedJulianDay {
    fn epoch() -> Fixed {
        Fixed::new(MJD_EPOCH)
    }
}

impl BoundedDayCount<f64> for ModifiedJulianDay {
    fn new(t: f64) -> ModifiedJulianDay {
        debug_assert!(ModifiedJulianDay::in_effective_bounds(t).is_ok());
        ModifiedJulianDay(t)
    }
    fn get(self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::math::TermNum;
    use crate::day_count::Fixed;
    use crate::day_count::JulianDay;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use proptest::proptest;

    proptest! {
        #[test]
        fn from_jd(t in FIXED_MIN..FIXED_MAX) {
            let x = Fixed::new(t);
            let j0 = JulianDay::from_fixed(x);
            let mjd0 = ModifiedJulianDay::from_fixed(x);
            assert!(mjd0.0.approx_eq(j0.get() - 2400000.5));
        }

    }
}

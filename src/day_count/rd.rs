use crate::day_count::prelude::BoundedDayCount;
use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;

const RD_EPOCH: f64 = 0.0;

/// Represents a Rata Die
///
/// The Rata Die is the count of days since midnight December 31, 0 CE in the
/// proleptic Gregorian Calendar.
///
/// This is internally a floating point number, where the integer portion represents a
/// particular day and the fractional portion represents a particular time of day.
///
/// Note that equality and ordering operations are subject to limitations similar to
/// equality and ordering operations on a floating point number. Two `RataDie` values represent
/// the same day or even the same second, but still appear different on the sub-second level.
///
/// Further reading:
/// + [Wikipedia](https://en.m.wikipedia.org/wiki/Rata_Die)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct RataDie(f64);

impl CalculatedBounds for RataDie {}

impl FromFixed for RataDie {
    fn from_fixed(t: Fixed) -> RataDie {
        RataDie(t.get() - RD_EPOCH)
    }
}

impl ToFixed for RataDie {
    fn to_fixed(self) -> Fixed {
        Fixed::new(RD_EPOCH + self.0)
    }
}

impl Epoch for RataDie {
    fn epoch() -> Fixed {
        Fixed::new(RD_EPOCH)
    }
}

impl BoundedDayCount<f64> for RataDie {
    fn new(t: f64) -> RataDie {
        debug_assert!(RataDie::in_effective_bounds(t).is_ok());
        RataDie(t)
    }
    fn get(self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rd_is_epoch() {
        assert_eq!(RataDie::new(0.0), RataDie::from_fixed(Fixed::new(0.0)));
    }
}

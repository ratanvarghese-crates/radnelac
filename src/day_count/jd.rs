use crate::common::bound::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;

const JD_EPOCH: f64 = -1721424.5;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDay(f64);

impl CalculatedBounds for JulianDay {}

impl FromFixed for JulianDay {
    fn from_fixed(t: Fixed) -> JulianDay {
        JulianDay(t.get() - JD_EPOCH)
    }
}

impl ToFixed for JulianDay {
    fn to_fixed(self) -> Fixed {
        Fixed::new(JD_EPOCH + self.0)
    }
}

impl Epoch for JulianDay {
    fn epoch() -> Fixed {
        Fixed::new(JD_EPOCH)
    }
}

impl BoundedDayCount<f64> for JulianDay {
    fn new(t: f64) -> JulianDay {
        debug_assert!(JulianDay::in_effective_bounds(t).is_ok());
        JulianDay(t)
    }
    fn get(self) -> f64 {
        self.0
    }
}

use crate::common::bound::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;

const UNIX_EPOCH: f64 = 719163.0;
const UNIX_DAY: f64 = 24.0 * 60.0 * 60.0;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct UnixMoment(i64);

impl CalculatedBounds for UnixMoment {}

impl FromFixed for UnixMoment {
    fn from_fixed(t: Fixed) -> UnixMoment {
        UnixMoment((UNIX_DAY * (t.get() - UNIX_EPOCH)).round() as i64)
    }
}

impl ToFixed for UnixMoment {
    fn to_fixed(self) -> Fixed {
        Fixed::new(UNIX_EPOCH + ((self.0 as f64) / UNIX_DAY))
    }
}

impl Epoch for UnixMoment {
    fn epoch() -> Fixed {
        Fixed::new(UNIX_EPOCH)
    }
}

impl BoundedDayCount<i64> for UnixMoment {
    fn new(t: i64) -> UnixMoment {
        debug_assert!(UnixMoment::in_effective_bounds(t).is_ok());
        UnixMoment(t)
    }
    fn get(self) -> i64 {
        self.0
    }
}

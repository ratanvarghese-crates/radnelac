use crate::common::bound::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;

const RD_EPOCH: f64 = 0.0;

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
    use proptest::proptest;

    #[test]
    fn rd_is_epoch() {
        assert_eq!(RataDie::new(0.0), RataDie::from_fixed(Fixed::new(0.0)));
    }

    proptest! {
        #[test]
        fn easy_i32(t in i32::MIN..i32::MAX) {
            let j0 = RataDie::new(t as f64);
            let j1 = RataDie::new(t as f64);
            assert_eq!(j0, j1);
        }
    }
}

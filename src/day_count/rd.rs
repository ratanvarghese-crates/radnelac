use crate::common::bound::BoundedDayCount;
use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;

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
        Fixed::unchecked_new(RD_EPOCH + self.0)
    }
}

impl Epoch for RataDie {
    fn epoch() -> Fixed {
        Fixed::unchecked_new(RD_EPOCH)
    }
}

impl BoundedDayCount<f64> for RataDie {
    fn unchecked_new(t: f64) -> RataDie {
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
    use crate::common::bound::EffectiveBound;
    use crate::common::math::EFFECTIVE_MAX;
    use crate::common::math::EFFECTIVE_MIN;
    use proptest::proptest;

    #[test]
    fn rd_is_epoch() {
        assert_eq!(RataDie::new(0), RataDie::from_fixed(Fixed::new(0)));
    }

    #[test]
    fn bounds() {
        assert!(RataDie::checked_new(RataDie::effective_min().get()).is_ok());
        assert!(RataDie::checked_new(RataDie::effective_max().get()).is_ok());
        let beyond_min = RataDie::effective_min().get() - 1.0;
        let beyond_max = RataDie::effective_max().get() + 1.0;
        assert!(RataDie::checked_new(beyond_min).is_err());
        assert!(RataDie::checked_new(beyond_max).is_err());
    }

    proptest! {
        #[test]
        fn roundtrip(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let rd0 = RataDie::checked_new(t).unwrap();
            let f0 = rd0.to_fixed();
            let rd1 = RataDie::from_fixed(f0);
            assert_eq!(rd0.0, rd1.0);
        }

        #[test]
        fn easy_i32(t in i32::MIN..i32::MAX) {
            let j0 = RataDie::checked_new(t as f64).unwrap();
            let j1 = RataDie::new(t);
            assert_eq!(j0, j1);
        }
    }
}

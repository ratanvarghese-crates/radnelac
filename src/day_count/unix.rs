use crate::common::bound::BoundedDayCount;
use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;

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
        Fixed::unchecked_new(UNIX_EPOCH + ((self.0 as f64) / UNIX_DAY))
    }
}

impl Epoch for UnixMoment {
    fn epoch() -> Fixed {
        Fixed::unchecked_new(UNIX_EPOCH)
    }
}

impl BoundedDayCount<i64> for UnixMoment {
    fn unchecked_new(t: i64) -> UnixMoment {
        debug_assert!(UnixMoment::in_effective_bounds(t).is_ok());
        UnixMoment(t)
    }
    fn get(self) -> i64 {
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
    const MAX_UNIX: i64 = ((EFFECTIVE_MAX - UNIX_EPOCH) * UNIX_DAY) as i64;
    const MIN_UNIX: i64 = ((EFFECTIVE_MIN - UNIX_EPOCH) * UNIX_DAY) as i64;

    #[test]
    fn bounds() {
        assert!(UnixMoment::checked_new(UnixMoment::effective_min().get()).is_ok());
        assert!(UnixMoment::checked_new(UnixMoment::effective_max().get()).is_ok());
        let beyond_min = UnixMoment::effective_min().get() - 1;
        let beyond_max = UnixMoment::effective_max().get() + 1;
        assert!(UnixMoment::checked_new(beyond_min).is_err());
        assert!(UnixMoment::checked_new(beyond_max).is_err());
    }

    #[test]
    fn around_epoch() {
        let before = Fixed::checked_new(UNIX_EPOCH - 1.0).unwrap();
        let exact = Fixed::checked_new(UNIX_EPOCH + 0.0).unwrap();
        let after = Fixed::checked_new(UNIX_EPOCH + 1.0).unwrap();
        assert_eq!(UnixMoment::from_fixed(before).0, -UNIX_DAY as i64);
        assert_eq!(UnixMoment::from_fixed(exact).0, 0);
        assert_eq!(UnixMoment::from_fixed(after).0, UNIX_DAY as i64);
    }

    proptest! {
        #[test]
        fn roundtrip(t in MIN_UNIX..MAX_UNIX) {
            let unix0 = UnixMoment::checked_new(t).unwrap();
            let f0 = UnixMoment::to_fixed(unix0);
            let unix1 = UnixMoment::from_fixed(f0);
            assert_eq!(unix0.0, unix1.0);
        }
    }
}

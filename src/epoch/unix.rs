use crate::epoch::common::simple_bounded;
use crate::epoch::fixed::EpochMoment;
use crate::epoch::fixed::FixedMoment;
use crate::error::CalendarError;
use crate::math::TermNum;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;
use num_traits::Bounded;

const UNIX_EPOCH: f64 = 719163.0;
const UNIX_DAY: i64 = 24 * 60 * 60;
pub const MAX_UNIX: i64 = ((EFFECTIVE_MAX - UNIX_EPOCH) as i64) * UNIX_DAY;
pub const MIN_UNIX: i64 = ((EFFECTIVE_MIN - UNIX_EPOCH) as i64) * UNIX_DAY;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct UnixMoment(i64);

impl EpochMoment for UnixMoment {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(UNIX_EPOCH).expect("Epoch known to be within bounds.")
    }
}

impl TryFrom<UnixMoment> for FixedMoment {
    type Error = CalendarError;
    fn try_from(s: UnixMoment) -> Result<FixedMoment, Self::Error> {
        FixedMoment::try_from(UnixMoment::epoch_moment() + ((s.0 as f64) / (UNIX_DAY as f64)))
    }
}

impl From<FixedMoment> for UnixMoment {
    fn from(t: FixedMoment) -> UnixMoment {
        UnixMoment(((UNIX_DAY as f64) * (t - UnixMoment::epoch_moment())).round() as i64)
    }
}

simple_bounded!(i64, UnixMoment, MIN_UNIX, MAX_UNIX);
// Let's not encourage code vulnerable to the Year 2038 problem
// bounded_small_int_guaranteed!(i32, i64, UnixMoment);
// bounded_small_int_guaranteed!(i16, i64, UnixMoment);
// bounded_small_int_guaranteed!(i8, i64, UnixMoment);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn bounds() {
        assert!(UnixMoment::try_from(UnixMoment::min_value()).is_ok());
        assert!(UnixMoment::try_from(UnixMoment::max_value()).is_ok());
        let beyond_min = i64::from(UnixMoment::min_value()) - 1;
        let beyond_max = i64::from(UnixMoment::max_value()) + 1;
        assert!(UnixMoment::try_from(beyond_min).is_err());
        assert!(UnixMoment::try_from(beyond_max).is_err());
    }

    #[test]
    fn around_epoch() {
        let before = FixedMoment::try_from(UnixMoment::epoch_moment() + (-1.0)).unwrap();
        let exact = FixedMoment::try_from(UnixMoment::epoch_moment() + 0.0).unwrap();
        let after = FixedMoment::try_from(UnixMoment::epoch_moment() + 1.0).unwrap();
        assert_eq!(UnixMoment::from(before).0, -UNIX_DAY);
        assert_eq!(UnixMoment::from(exact).0, 0);
        assert_eq!(UnixMoment::from(after).0, UNIX_DAY);
    }

    proptest! {
        #[test]
        fn roundtrip(t in MIN_UNIX..MAX_UNIX) {
            let unix0 = UnixMoment(t as i64);
            let f0 = FixedMoment::try_from(unix0).unwrap();
            let unix1 = UnixMoment::from(f0);
            assert_eq!(unix0.0, unix1.0);
        }
    }
}

use crate::epoch::fixed::EpochMoment;
use crate::error::CalendarError;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;
use num_traits::Bounded;

use crate::epoch::fixed::FixedMoment;

const RD_EPOCH: f64 = 0.0;
pub const MAX_RD: f64 = EFFECTIVE_MAX - RD_EPOCH;
pub const MIN_RD: f64 = EFFECTIVE_MIN - RD_EPOCH;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct RataDie(pub f64);

impl EpochMoment for RataDie {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(RD_EPOCH).expect("Epoch known to be within bounds.")
    }
}

impl TryFrom<RataDie> for FixedMoment {
    type Error = CalendarError;
    fn try_from(rd: RataDie) -> Result<FixedMoment, Self::Error> {
        FixedMoment::try_from(RataDie::epoch_moment() + rd.0)
    }
}

impl From<FixedMoment> for RataDie {
    fn from(t: FixedMoment) -> RataDie {
        RataDie(t - RataDie::epoch_moment())
    }
}

impl Bounded for RataDie {
    fn min_value() -> Self {
        RataDie(MIN_RD)
    }

    fn max_value() -> Self {
        RataDie(MAX_RD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn rd_is_epoch() {
        assert_eq!(
            RataDie(0.0),
            RataDie::from(FixedMoment::try_from(0.0).unwrap())
        );
    }

    #[test]
    fn bounds() {
        assert!(FixedMoment::try_from(RataDie::min_value()).is_ok());
        assert!(FixedMoment::try_from(RataDie::max_value()).is_ok());
        let beyond_min = RataDie(RataDie::min_value().0 - 1.0);
        let beyond_max = RataDie(RataDie::max_value().0 + 1.0);
        assert!(FixedMoment::try_from(beyond_min).is_err());
        assert!(FixedMoment::try_from(beyond_max).is_err());
    }

    proptest! {
        #[test]
        fn roundtrip(t in MIN_RD..MAX_RD) {
            let rd0 = RataDie(t);
            let f0 = FixedMoment::try_from(rd0).unwrap();
            let rd1 = RataDie::from(f0);
            assert_eq!(rd0.0, rd1.0);
        }
    }
}

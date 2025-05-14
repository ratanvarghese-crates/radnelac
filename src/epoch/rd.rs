use crate::epoch::fixed::EpochMoment;
use crate::error::CalendarError;

use crate::epoch::fixed::FixedMoment;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct RataDie(pub f64);

impl EpochMoment for RataDie {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(0).expect("Epoch known to be within bounds.")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::EFFECTIVE_MAX;
    use crate::math::EFFECTIVE_MIN;
    use proptest::proptest;

    #[test]
    fn rd_is_epoch() {
        assert_eq!(
            RataDie(0.0),
            RataDie::from(FixedMoment::try_from(0.0).unwrap())
        );
    }

    proptest! {
        #[test]
        fn roundtrip(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let rd0 = RataDie(t);
            let f0 = FixedMoment::try_from(rd0).unwrap();
            let rd1 = RataDie::from(f0);
            assert_eq!(rd0.0, rd1.0);
        }
    }
}

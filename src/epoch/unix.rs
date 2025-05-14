use crate::epoch::fixed::EpochMoment;
use crate::epoch::fixed::FixedMoment;
use crate::error::CalendarError;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;
pub const MAX_UNIX: i64 = (EFFECTIVE_MAX as i64) * (24 * 60 * 60);
pub const MIN_UNIX: i64 = (EFFECTIVE_MIN as i64) * (24 * 60 * 60);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct UnixMoment(pub i64);

impl EpochMoment for UnixMoment {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(719163.0).expect("Epoch known to be within bounds.")
    }
}

impl TryFrom<UnixMoment> for FixedMoment {
    type Error = CalendarError;
    fn try_from(s: UnixMoment) -> Result<FixedMoment, Self::Error> {
        FixedMoment::try_from(
            f64::from(UnixMoment::epoch_moment()) + ((s.0 as f64) / (24.0 * 60.0 * 60.0)),
        )
    }
}

impl From<FixedMoment> for UnixMoment {
    fn from(t: FixedMoment) -> UnixMoment {
        UnixMoment(
            (24.0 * 60.0 * 60.0 * (f64::from(t) - f64::from(UnixMoment::epoch_moment()))).round()
                as i64,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

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

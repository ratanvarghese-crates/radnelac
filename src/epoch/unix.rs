use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedMoment;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct UnixMoment(pub f64);

impl Epoch<FixedMoment> for UnixMoment {
    fn epoch() -> FixedMoment {
        FixedMoment(719163.0)
    }
}

impl From<UnixMoment> for FixedMoment {
    fn from(s: UnixMoment) -> FixedMoment {
        FixedMoment(UnixMoment::epoch().0 + (s.0 / (24.0 * 60.0 * 60.0)))
    }
}

impl From<FixedMoment> for UnixMoment {
    fn from(t: FixedMoment) -> UnixMoment {
        UnixMoment(24.0 * 60.0 * 60.0 * (t.0 - UnixMoment::epoch().0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::EFFECTIVE_MAX;
    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip(t in -EFFECTIVE_MAX..EFFECTIVE_MAX) {
            let unix0 = UnixMoment(t);
            let unix1 = UnixMoment::from(FixedMoment::from(unix0));
            assert_eq!(unix0.0.floor(), unix1.0.floor());
        }
    }
}

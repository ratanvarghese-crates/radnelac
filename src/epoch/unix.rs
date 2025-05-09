use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedMoment;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct UnixMoment(pub f64);

impl Epoch for UnixMoment {
    type Output = FixedMoment;
    fn epoch() -> FixedMoment {
        FixedMoment(719163.0)
    }
}

impl From<UnixMoment> for FixedMoment {
    fn from(s: UnixMoment) -> FixedMoment {
        UnixMoment::epoch() + FixedMoment(s.0 / (24.0 * 60.0 * 60.0))
    }
}

impl From<FixedMoment> for UnixMoment {
    fn from(t: FixedMoment) -> UnixMoment {
        UnixMoment(24.0 * 60.0 * 60. * (t - UnixMoment::epoch()).0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let unix0 = UnixMoment(1746587115.0);
        let unix1 = UnixMoment::from(FixedMoment::from(unix0));
        assert_eq!(unix0.0.floor(), unix1.0.floor());
    }
}

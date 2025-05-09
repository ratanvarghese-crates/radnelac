use crate::epoch::fixed::Epoch;

use crate::epoch::fixed::FixedMoment;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct RataDie(pub f64);

impl Epoch for RataDie {
    type Output = FixedMoment;
    fn epoch() -> FixedMoment {
        FixedMoment(0.0)
    }
}

impl From<RataDie> for FixedMoment {
    fn from(rd: RataDie) -> FixedMoment {
        FixedMoment(rd.0) + RataDie::epoch()
    }
}

impl From<FixedMoment> for RataDie {
    fn from(t: FixedMoment) -> RataDie {
        RataDie((t - RataDie::epoch()).0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rd_is_epoch() {
        assert_eq!(RataDie(0.0), RataDie::from(FixedMoment(0.0)));
    }
}

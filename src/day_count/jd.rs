use crate::common::bound::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;

const JD_EPOCH: f64 = -1721424.5;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDay(f64);

impl CalculatedBounds for JulianDay {}

impl FromFixed for JulianDay {
    fn from_fixed(t: Fixed) -> JulianDay {
        JulianDay(t.get() - JD_EPOCH)
    }
}

impl ToFixed for JulianDay {
    fn to_fixed(self) -> Fixed {
        Fixed::new(JD_EPOCH + self.0)
    }
}

impl Epoch for JulianDay {
    fn epoch() -> Fixed {
        Fixed::new(JD_EPOCH)
    }
}

impl BoundedDayCount<f64> for JulianDay {
    fn new(t: f64) -> JulianDay {
        debug_assert!(JulianDay::in_effective_bounds(t).is_ok());
        JulianDay(t)
    }
    fn get(self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::math::TermNum;
    use crate::day_count::Fixed;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use proptest::proptest;
    const MAX_JD: f64 = FIXED_MAX - JD_EPOCH;
    const MIN_JD: f64 = FIXED_MIN - JD_EPOCH;

    #[test]
    fn around_epoch() {
        let before = Fixed::new(JD_EPOCH + (-1.0));
        let exact = Fixed::new(JD_EPOCH + 0.0);
        let after = Fixed::new(JD_EPOCH + 1.0);
        assert_eq!(JulianDay::from_fixed(before).get(), -1.0);
        assert_eq!(JulianDay::from_fixed(exact).get(), 0.0);
        assert_eq!(JulianDay::from_fixed(after).get(), 1.0);
    }

    proptest! {
        #[test]
        fn easy_i32(t in i32::MIN..i32::MAX) {
            let j0 = JulianDay::new(t as f64);
            let j1 = JulianDay::new(t as f64);
            assert_eq!(j0, j1);
        }
    }
}

use crate::common::bound::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;

const MJD_EPOCH: f64 = 678576.0;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ModifiedJulianDay(f64);

impl CalculatedBounds for ModifiedJulianDay {}

impl FromFixed for ModifiedJulianDay {
    fn from_fixed(t: Fixed) -> ModifiedJulianDay {
        ModifiedJulianDay(t.get() - MJD_EPOCH)
    }
}

impl ToFixed for ModifiedJulianDay {
    fn to_fixed(self) -> Fixed {
        Fixed::new(MJD_EPOCH + self.0)
    }
}

impl Epoch for ModifiedJulianDay {
    fn epoch() -> Fixed {
        Fixed::new(MJD_EPOCH)
    }
}

impl BoundedDayCount<f64> for ModifiedJulianDay {
    fn new(t: f64) -> ModifiedJulianDay {
        debug_assert!(ModifiedJulianDay::in_effective_bounds(t).is_ok());
        ModifiedJulianDay(t)
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
    use crate::day_count::JulianDay;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use proptest::proptest;
    const MAX_JD: f64 = FIXED_MAX - MJD_EPOCH;
    const MIN_JD: f64 = FIXED_MIN - MJD_EPOCH;

    #[test]
    fn around_epoch() {
        let before = Fixed::new(MJD_EPOCH + (-1.0));
        let exact = Fixed::new(MJD_EPOCH + 0.0);
        let after = Fixed::new(MJD_EPOCH + 1.0);
        assert_eq!(ModifiedJulianDay::from_fixed(before).get(), -1.0);
        assert_eq!(ModifiedJulianDay::from_fixed(exact).get(), 0.0);
        assert_eq!(ModifiedJulianDay::from_fixed(after).get(), 1.0);
    }

    proptest! {
        #[test]
        fn easy_i32(t in i32::MIN..i32::MAX) {
            let j0 = ModifiedJulianDay::new(t as f64);
            let j1 = ModifiedJulianDay::new(t as f64);
            assert_eq!(j0, j1);
        }

        #[test]
        fn from_jd(t in FIXED_MIN..FIXED_MAX) {
            let x = Fixed::new(t);
            let j0 = JulianDay::from_fixed(x);
            let mjd0 = ModifiedJulianDay::from_fixed(x);
            assert!(mjd0.0.approx_eq(j0.get() - 2400000.5));
        }

    }
}

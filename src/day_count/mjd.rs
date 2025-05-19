use crate::common::bound::BoundedDayCount;
use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;

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
        Fixed::unchecked_new(MJD_EPOCH + self.0)
    }
}

impl Epoch for ModifiedJulianDay {
    fn epoch() -> Fixed {
        Fixed::unchecked_new(MJD_EPOCH)
    }
}

impl BoundedDayCount<f64> for ModifiedJulianDay {
    fn unchecked_new(t: f64) -> ModifiedJulianDay {
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
    use crate::common::bound::EffectiveBound;
    use crate::common::math::TermNum;
    use crate::common::math::EFFECTIVE_MAX;
    use crate::common::math::EFFECTIVE_MIN;
    use crate::day_count::fixed::Fixed;
    use crate::day_count::jd::JulianDay;
    use proptest::proptest;
    const MAX_JD: f64 = EFFECTIVE_MAX - MJD_EPOCH;
    const MIN_JD: f64 = EFFECTIVE_MIN - MJD_EPOCH;

    #[test]
    fn around_epoch() {
        let before = Fixed::checked_new(MJD_EPOCH + (-1.0)).unwrap();
        let exact = Fixed::checked_new(MJD_EPOCH + 0.0).unwrap();
        let after = Fixed::checked_new(MJD_EPOCH + 1.0).unwrap();
        assert_eq!(ModifiedJulianDay::from_fixed(before).get(), -1.0);
        assert_eq!(ModifiedJulianDay::from_fixed(exact).get(), 0.0);
        assert_eq!(ModifiedJulianDay::from_fixed(after).get(), 1.0);
    }

    #[test]
    fn bounds() {
        assert!(ModifiedJulianDay::checked_new(ModifiedJulianDay::effective_min().get()).is_ok());
        assert!(ModifiedJulianDay::checked_new(ModifiedJulianDay::effective_max().get()).is_ok());
        let beyond_min = ModifiedJulianDay::effective_min().get() - 1.0;
        let beyond_max = ModifiedJulianDay::effective_max().get() + 1.0;
        assert!(ModifiedJulianDay::checked_new(beyond_min).is_err());
        assert!(ModifiedJulianDay::checked_new(beyond_max).is_err());
    }

    proptest! {
        #[test]
        fn roundtrip(t in MIN_JD..MAX_JD) {
            let j0 = ModifiedJulianDay::checked_new(t).unwrap();
            let j1 = ModifiedJulianDay::from_fixed(ModifiedJulianDay::to_fixed(j0));
            let j2 = ModifiedJulianDay::from_fixed(ModifiedJulianDay::to_fixed(j0).to_day());
            assert!(j0.0.approx_eq(j1.0));
            assert!((j0.0 - j2.0).abs() < 1.0);
        }

        #[test]
        fn easy_i32(t in i32::MIN..i32::MAX) {
            let j0 = ModifiedJulianDay::checked_new(t as f64).unwrap();
            let j1 = ModifiedJulianDay::new(t);
            assert_eq!(j0, j1);
        }

                #[test]
        fn from_jd(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let x = Fixed::checked_new(t).unwrap();
            let j0 = JulianDay::from_fixed(x);
            let mjd0 = ModifiedJulianDay::from_fixed(x);
            assert!(mjd0.0.approx_eq(j0.get() - 2400000.5));
        }

    }
}

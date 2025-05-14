use crate::epoch::common::bounded_small_int_guaranteed;
use crate::epoch::common::simple_bounded;
use crate::epoch::fixed::EpochMoment;
use crate::epoch::fixed::FixedMoment;
use crate::error::CalendarError;
use crate::math::TermNum;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;
use num_traits::Bounded;

const MJD_EPOCH: f64 = 678576.0;
pub const MAX_MJD: f64 = EFFECTIVE_MAX - MJD_EPOCH;
pub const MIN_MJD: f64 = EFFECTIVE_MIN - MJD_EPOCH;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ModifiedJulianDay(f64);

impl EpochMoment for ModifiedJulianDay {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(MJD_EPOCH).expect("Epoch known to be within bounds.")
    }
}

impl From<ModifiedJulianDay> for FixedMoment {
    fn from(mjd: ModifiedJulianDay) -> FixedMoment {
        FixedMoment::try_from(ModifiedJulianDay::epoch_moment() + mjd.0)
            .expect("Known within bounds.")
    }
}

impl From<FixedMoment> for ModifiedJulianDay {
    fn from(t: FixedMoment) -> ModifiedJulianDay {
        ModifiedJulianDay(t - ModifiedJulianDay::epoch_moment())
    }
}

simple_bounded!(f64, ModifiedJulianDay, MIN_MJD, MAX_MJD);
bounded_small_int_guaranteed!(i32, f64, ModifiedJulianDay);
bounded_small_int_guaranteed!(i16, f64, ModifiedJulianDay);
bounded_small_int_guaranteed!(i8, f64, ModifiedJulianDay);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epoch::jd::JulianDay;
    use crate::math::TermNum;
    use proptest::proptest;

    #[test]
    fn around_epoch() {
        let before = FixedMoment::try_from(ModifiedJulianDay::epoch_moment() + (-1.0)).unwrap();
        let exact = FixedMoment::try_from(ModifiedJulianDay::epoch_moment() + 0.0).unwrap();
        let after = FixedMoment::try_from(ModifiedJulianDay::epoch_moment() + 1.0).unwrap();
        assert_eq!(f64::from(ModifiedJulianDay::from(before)), -1.0);
        assert_eq!(f64::from(ModifiedJulianDay::from(exact)), 0.0);
        assert_eq!(f64::from(ModifiedJulianDay::from(after)), 1.0);
    }

    #[test]
    fn bounds() {
        assert!(ModifiedJulianDay::try_from(ModifiedJulianDay::min_value()).is_ok());
        assert!(ModifiedJulianDay::try_from(ModifiedJulianDay::max_value()).is_ok());
        let beyond_min = f64::from(ModifiedJulianDay::min_value()) - 1.0;
        let beyond_max = f64::from(ModifiedJulianDay::max_value()) + 1.0;
        assert!(ModifiedJulianDay::try_from(beyond_min).is_err());
        assert!(ModifiedJulianDay::try_from(beyond_max).is_err());
    }

    proptest! {
        #[test]
        fn roundtrip(t in MIN_MJD..MAX_MJD) {
            let j0 = ModifiedJulianDay::try_from(t).unwrap();
            let j1 = ModifiedJulianDay::from(FixedMoment::from(j0));
            assert!(j0.0.approx_eq(j1.0));
        }

        #[test]
        fn from_jd(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let x = FixedMoment::try_from(t).unwrap();
            let j0 = JulianDay::from(x);
            let mjd0 = ModifiedJulianDay::from(x);
            assert!(mjd0.0.approx_eq(f64::try_from(j0).unwrap() - 2400000.5));
        }

        #[test]
        fn easy_i32(t in i32::MIN..i32::MAX) {
            let j0 = ModifiedJulianDay::try_from(t as f64).unwrap();
            let j1 = ModifiedJulianDay::from(t);
            assert_eq!(j0, j1);
        }
    }
}

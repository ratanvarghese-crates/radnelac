use crate::epoch::common::bounded_small_int_guaranteed;
use crate::epoch::common::simple_bounded;
use crate::epoch::fixed::EpochMoment;
use crate::epoch::fixed::FixedDate;
use crate::epoch::fixed::FixedMoment;
use crate::error::CalendarError;
use crate::math::TermNum;
use crate::math::EFFECTIVE_MAX;
use crate::math::EFFECTIVE_MIN;
use num_traits::Bounded;

const JD_EPOCH: f64 = -1721424.5;
pub const MAX_JD: f64 = EFFECTIVE_MAX - JD_EPOCH;
pub const MIN_JD: f64 = EFFECTIVE_MIN - JD_EPOCH;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct JulianDay(f64);

impl EpochMoment for JulianDay {
    fn epoch_moment() -> FixedMoment {
        FixedMoment::try_from(JD_EPOCH).expect("Epoch known to be within bounds.")
    }
}

impl From<JulianDay> for FixedMoment {
    fn from(jd: JulianDay) -> FixedMoment {
        FixedMoment::try_from(JulianDay::epoch_moment() + jd.0).expect("Known within bounds")
    }
}

impl From<FixedMoment> for JulianDay {
    fn from(t: FixedMoment) -> JulianDay {
        JulianDay(t - JulianDay::epoch_moment())
    }
}

impl From<JulianDay> for FixedDate {
    fn from(jd: JulianDay) -> FixedDate {
        FixedDate::from(FixedMoment::from(jd))
    }
}

impl From<FixedDate> for JulianDay {
    fn from(t: FixedDate) -> JulianDay {
        JulianDay::from(FixedMoment::from(t))
    }
}

simple_bounded!(f64, JulianDay, MIN_JD, MAX_JD);
bounded_small_int_guaranteed!(i32, f64, JulianDay);
bounded_small_int_guaranteed!(i16, f64, JulianDay);
bounded_small_int_guaranteed!(i8, f64, JulianDay);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::TermNum;
    use proptest::proptest;

    #[test]
    fn around_epoch() {
        let before = FixedMoment::try_from(JulianDay::epoch_moment() + (-1.0)).unwrap();
        let exact = FixedMoment::try_from(JulianDay::epoch_moment() + 0.0).unwrap();
        let after = FixedMoment::try_from(JulianDay::epoch_moment() + 1.0).unwrap();
        assert_eq!(f64::from(JulianDay::from(before)), -1.0);
        assert_eq!(f64::from(JulianDay::from(exact)), 0.0);
        assert_eq!(f64::from(JulianDay::from(after)), 1.0);
    }

    #[test]
    fn bounds() {
        assert!(JulianDay::try_from(JulianDay::min_value()).is_ok());
        assert!(JulianDay::try_from(JulianDay::max_value()).is_ok());
        let beyond_min = f64::from(JulianDay::min_value()) - 1.0;
        let beyond_max = f64::from(JulianDay::max_value()) + 1.0;
        assert!(JulianDay::try_from(beyond_min).is_err());
        assert!(JulianDay::try_from(beyond_max).is_err());
    }

    proptest! {
        #[test]
        fn roundtrip(t in MIN_JD..MAX_JD) {
            let j0 = JulianDay::try_from(t).unwrap();
            let j1 = JulianDay::from(FixedMoment::try_from(j0).unwrap());
            let j2 = JulianDay::from(FixedDate::try_from(j0).unwrap());
            assert!(j0.0.approx_eq(j1.0));
            assert!((j0.0 - j2.0).abs() < 1.0);
        }

        #[test]
        fn easy_i32(t in i32::MIN..i32::MAX) {
            let j0 = JulianDay::try_from(t as f64).unwrap();
            let j1 = JulianDay::from(t);
            assert_eq!(j0, j1);
        }
    }
}

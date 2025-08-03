use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_cycle::BoundedCycle;
use crate::day_cycle::OnOrBefore;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

/// Represents a prefix in the Akan day cycle
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum AkanPrefix {
    Nwona = 1,
    Nkyi,
    Kuru,
    Kwa,
    Mono,
    Fo,
}

impl BoundedCycle<6, 1> for AkanPrefix {}

/// Represents a stem in the Akan day cycle
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum AkanStem {
    Wukuo = 1,
    Yaw,
    Fie,
    Memene,
    Kwasi,
    Dwo,
    Bene,
}

impl BoundedCycle<7, 1> for AkanStem {}

/// Represents a specific day in the Akan day cycle
///
/// Further reading:
/// + [Wikipedia](https://en.wikipedia.org/wiki/Akan_calendar)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Akan {
    prefix: AkanPrefix,
    stem: AkanStem,
}

const CYCLE_START: i64 = 37;
const CYCLE_LENGTH: u8 = 42;

impl Akan {
    /// Create a day in the Akan day cycle
    pub fn new(prefix: AkanPrefix, stem: AkanStem) -> Akan {
        Akan { prefix, stem }
    }

    /// Given a position in the Akan day cycle, return the day in the cycle.
    ///
    /// It is assumed that the first day in the cycle is Nwuna-Wukuo.
    pub fn day_name(n: i64) -> Akan {
        Akan::new(AkanPrefix::from_unbounded(n), AkanStem::from_unbounded(n))
    }

    /// Given two days in the Akan day cycle, return the difference in days.
    pub fn name_difference(self, other: Self) -> i16 {
        let prefix1 = self.prefix as i16;
        let stem1 = self.stem as i16;
        let prefix2 = other.prefix as i16;
        let stem2 = other.stem as i16;

        let prefix_diff = prefix2 - prefix1;
        let stem_diff = stem2 - stem1;

        (prefix_diff + 36 * (stem_diff - prefix_diff)).adjusted_remainder(CYCLE_LENGTH as i16)
    }

    /// Given a day in the Akan cycle, return the stem
    pub fn stem(self) -> AkanStem {
        self.stem
    }

    /// Given a day in the Akan cycle, return the stem
    pub fn prefix(self) -> AkanPrefix {
        self.prefix
    }
}

impl FromFixed for Akan {
    fn from_fixed(t: Fixed) -> Akan {
        Akan::day_name(t.get_day_i() - Akan::epoch().get_day_i())
    }
}

impl FromPrimitive for Akan {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Akan::day_name(n))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Akan::from_i64(n.to_i64()?)
    }
}

impl ToPrimitive for Akan {
    fn to_i64(&self) -> Option<i64> {
        let a = Akan::new(AkanPrefix::Nwona, AkanStem::Wukuo);
        Some(a.name_difference(*self) as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.to_i64().expect("Guaranteed in range") as u64)
    }
}

impl Epoch for Akan {
    fn epoch() -> Fixed {
        Fixed::new(CYCLE_START as f64)
    }
}

impl BoundedCycle<CYCLE_LENGTH, 1> for Akan {}

impl OnOrBefore<CYCLE_LENGTH, 1> for Akan {
    fn raw_on_or_before(self, date: i64) -> Fixed {
        let diff = Akan::from_fixed(Fixed::cast_new(0)).name_difference(self) as i64;
        Fixed::cast_new(diff.interval_modulus(date, date - (CYCLE_LENGTH as i64)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use proptest::proptest;

    proptest! {
        #[test]
        fn akan_prefix_stem_repeats(x in FIXED_MIN..(FIXED_MAX - 7.0), d in 1.0..5.0) {
            let a1 = Akan::from_fixed(Fixed::new(x));
            let a2 = Akan::from_fixed(Fixed::new(x + d));
            let a3 = Akan::from_fixed(Fixed::new(x + 6.0));
            let a4 = Akan::from_fixed(Fixed::new(x + 7.0));
            assert_ne!(a1.prefix(), a2.prefix());
            assert_eq!(a1.prefix(), a3.prefix());
            assert_ne!(a1.prefix(), a4.prefix());
            assert_ne!(a1.stem(), a2.stem());
            assert_ne!(a1.stem(), a3.stem());
            assert_eq!(a1.stem(), a4.stem());
        }

        #[test]
        fn prefix_stem_sequence(x in FIXED_MIN..FIXED_MAX) {
            let f0 = Fixed::new(x);
            let f1 = Fixed::new(x + 1.0);
            let a0 = Akan::from_fixed(f0);
            let a1 = Akan::from_fixed(f1);
            let p0 = a0.prefix();
            let p1 = a1.prefix();
            if p0 == AkanPrefix::Fo {
                assert_eq!(p1, AkanPrefix::Nwona);
            } else {
                assert_eq!(p1, AkanPrefix::from_i64(p0 as i64 + 1).unwrap());
            }
            let s0 = a0.stem();
            let s1 = a1.stem();
            if s0 == AkanStem::Bene {
                assert_eq!(s1, AkanStem::Wukuo)
            } else {
                assert_eq!(s1, AkanStem::from_i64(s0 as i64 + 1).unwrap());
            }
        }

    }
}

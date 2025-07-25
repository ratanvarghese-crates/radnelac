use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_cycle::BoundedCycle;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

/// Represents a prefix in the Akan day cycle
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
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
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
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

impl Akan {
    /// Create a day in the Akan day cycle
    pub fn new(stem: AkanStem, prefix: AkanPrefix) -> Akan {
        Akan { stem, prefix }
    }

    /// Given a position in the Akan day cycle, return the day in the cycle.
    ///
    /// It is assumed that the first day in the cycle is Nwuna-Wukuo.
    pub fn day_name(n: i64) -> Akan {
        Akan::new(AkanStem::from_unbounded(n), AkanPrefix::from_unbounded(n))
    }

    /// Given two days in the Akan day cycle, return the difference in days.
    pub fn name_difference(self, other: Self) -> i16 {
        let prefix1 = self.prefix as i16;
        let stem1 = self.stem as i16;
        let prefix2 = other.prefix as i16;
        let stem2 = other.stem as i16;

        let prefix_diff = prefix2 - prefix1;
        let stem_diff = stem2 - stem1;

        (prefix_diff + 36 * (stem_diff - prefix_diff)).adjusted_remainder(42)
    }

    /// Given a day in the Akan cycle, return the stem
    pub fn stem(self) -> AkanStem {
        self.stem
    }

    /// Given a day in the Akan cycle, return the stem
    pub fn prefix(self) -> AkanPrefix {
        self.prefix
    }

    pub fn unchecked_on_or_before(self, date: Fixed) -> i64 {
        let date = date.get_day_i();
        let diff = Akan::from_fixed(Fixed::cast_new(0)).name_difference(self) as i64;
        diff.interval_modulus(date, date - 42)
    }

    pub fn on_or_before(self, date: Fixed) -> Fixed {
        Fixed::cast_new(self.unchecked_on_or_before(date))
    }
}

impl FromFixed for Akan {
    fn from_fixed(t: Fixed) -> Akan {
        Akan::day_name(t.get_day_i() - CYCLE_START)
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

impl Epoch for Akan {
    fn epoch() -> Fixed {
        Fixed::new(CYCLE_START as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use crate::day_cycle::Weekday;
    use proptest::proptest;

    proptest! {
        #[test]
        fn akan_stem_and_weekday(x in (FIXED_MIN+42.0)..(FIXED_MAX-42.0)) {
            //https://en.wikipedia.org/wiki/Akan_calendar
            let f = Fixed::new(x);
            let w = Weekday::from_fixed(f);
            let a = Akan::from_fixed(f).stem();
            let expected_a = match w {
                Weekday::Monday => AkanStem::Dwo,
                Weekday::Tuesday => AkanStem::Bene,
                Weekday::Wednesday => AkanStem::Wukuo,
                Weekday::Thursday => AkanStem::Yaw,
                Weekday::Friday => AkanStem::Fie,
                Weekday::Saturday => AkanStem::Memene,
                Weekday::Sunday => AkanStem::Kwasi,
            };
            assert_eq!(a, expected_a);
        }

        #[test]
        fn akan_day_repeats(x in FIXED_MIN..(FIXED_MAX - 42.0), d in 1..41) {
            let f1 = Fixed::new(x);
            let f2 = Fixed::new(x + (d as f64));
            let f3 = Fixed::new(x + 42.0);
            let a1 = Akan::from_fixed(f1);
            let a2 = Akan::from_fixed(f2);
            let a3 = Akan::from_fixed(f3);
            assert_ne!(a1, a2);
            assert_eq!(a1, a3);
            assert_eq!(a2.name_difference(a1), (42 - d) as i16);
            assert_eq!(a3.name_difference(a1), 42);
            assert_eq!(a1.on_or_before(f1).to_day(), f1.to_day());
            assert_eq!(a1.on_or_before(f2).to_day(), f1.to_day());
        }

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

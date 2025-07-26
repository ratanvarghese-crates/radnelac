use crate::common::math::TermNum;
use std::num::NonZero;

/// Represents a year grouped by Olympiad
///
/// ## Year 0
///
/// Year 0 is **not** supported because they are not supported in the Julian calendar.
///
/// ## Further reading
/// + [Wikipedia](https://en.wikipedia.org/wiki/Olympiad)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Olympiad {
    cycle: i32,
    year: u8,
}

const OLYMPIAD_START: i32 = -776;

impl Olympiad {
    pub fn to_julian_year(self) -> NonZero<i32> {
        let years = OLYMPIAD_START + 4 * (self.cycle - 1) + (self.year as i32) - 1;
        let result = if years < 0 { years } else { years + 1 };
        NonZero::new(result).expect("Prevented by if")
    }

    pub fn from_julian_year(j: NonZero<i32>) -> Self {
        let j_year = j.get();
        let years = j_year - OLYMPIAD_START - (if j_year < 0 { 0 } else { 1 });
        Olympiad {
            cycle: years.div_euclid(4) + 1,
            year: (years.modulus(4) as u8 + 1),
        }
    }

    pub fn cycle(self) -> i32 {
        self.cycle
    }

    pub fn year(self) -> u8 {
        self.year
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prop_assume;
    use proptest::proptest;

    #[test]
    fn next_year_0() {
        let t0 = -1;
        let t1 = 1;
        let o0 = Olympiad::from_julian_year(NonZero::new(t0).unwrap());
        let o1 = Olympiad::from_julian_year(NonZero::new(t1).unwrap());
        assert_eq!(o1.year(), (o0.year() + 1).adjusted_remainder(4));
        if o1.year() == 1 {
            assert_eq!(o1.cycle(), o0.cycle() + 1);
        } else {
            assert_eq!(o1.cycle(), o0.cycle());
        }
    }

    proptest! {
        #[test]
        fn roundtrip(t in i32::MIN..i32::MAX) {
            prop_assume!(t != 0);
            let o = Olympiad::from_julian_year(NonZero::new(t).unwrap());
            let j = o.to_julian_year().get();
            assert_eq!(t, j);
        }

        #[test]
        fn year_range(t0 in i32::MIN..i32::MAX) {
            prop_assume!(t0 != 0);
            let o = Olympiad::from_julian_year(NonZero::new(t0).unwrap());
            assert!(o.year() < 5);
        }

        #[test]
        fn next_year(t0 in i32::MIN..i32::MAX) {
            let t1 = t0 + 1;
            prop_assume!(t0 != 0 && t1 != 0);
            let o0 = Olympiad::from_julian_year(NonZero::new(t0).unwrap());
            let o1 = Olympiad::from_julian_year(NonZero::new(t1).unwrap());
            assert_eq!(o1.year(), (o0.year() + 1).adjusted_remainder(4));
            if o1.year() == 1 {
                assert_eq!(o1.cycle(), o0.cycle() + 1);
            } else {
                assert_eq!(o1.cycle(), o0.cycle());
            }
        }
    }
}

use crate::common::math::TermNum;
use std::num::NonZero;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Olympiad {
    cycle: i32,
    year: i32,
}

const OLYMPIAD_START: i32 = -776;

impl Olympiad {
    pub fn julian_year_from_olympiad(self) -> NonZero<i32> {
        let years = OLYMPIAD_START + 4 * (self.cycle - 1) + self.year - 1;
        let result = if years < 0 { years } else { years + 1 };
        NonZero::new(result).unwrap()
    }

    pub fn olympiad_from_julian_year(j: NonZero<i32>) -> Self {
        let j_year = j.get();
        let years = j_year - OLYMPIAD_START - (if j_year < 0 { 0 } else { 1 });
        Olympiad {
            cycle: years.div_euclid(4) + 1,
            year: years.modulus(4) + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prop_assume;
    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip(t in i16::MIN..i16::MAX) {
            prop_assume!(t != 0);
            let o = Olympiad::olympiad_from_julian_year(NonZero::new(t as i32).unwrap());
            let j = Olympiad::julian_year_from_olympiad(o);
            assert_eq!(t as i32, j.get());
        }
    }
}

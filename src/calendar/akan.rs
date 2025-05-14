use crate::epoch::fixed::Epoch;
use crate::epoch::fixed::FixedDate;
use crate::error::CalendarError;
use crate::math::TermNum;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum AkanPrefix {
    Nwona = 1,
    Nkyi,
    Kuru,
    Kwa,
    Mono,
    Fo,
}

impl From<i64> for AkanPrefix {
    fn from(n: i64) -> AkanPrefix {
        let m = n.modulus(6);
        assert!(m >= 0 && m <= 5);
        match m {
            0 => AkanPrefix::Nwona,
            1 => AkanPrefix::Nkyi,
            2 => AkanPrefix::Kuru,
            3 => AkanPrefix::Kwa,
            4 => AkanPrefix::Mono,
            5 => AkanPrefix::Fo,
            _ => AkanPrefix::Nwona,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum AkanStem {
    Wukuo = 1,
    Yaw,
    Fie,
    Memene,
    Kwasi,
    Dwo,
    Bene,
}

impl From<i64> for AkanStem {
    fn from(n: i64) -> AkanStem {
        let m = n.modulus(7);
        assert!(m >= 0 && m <= 6);
        match m {
            0 => AkanStem::Wukuo,
            1 => AkanStem::Yaw,
            2 => AkanStem::Fie,
            3 => AkanStem::Memene,
            4 => AkanStem::Kwasi,
            5 => AkanStem::Dwo,
            6 => AkanStem::Bene,
            _ => AkanStem::Wukuo,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct AkanDay {
    prefix: AkanPrefix,
    stem: AkanStem,
}

impl AkanDay {
    fn new(stem: AkanStem, prefix: AkanPrefix) -> AkanDay {
        AkanDay { stem, prefix }
    }

    fn day_name(n: i64) -> AkanDay {
        AkanDay::new(AkanStem::from(n), AkanPrefix::from(n))
    }

    fn name_difference(self, other: Self) -> i16 {
        let prefix1 = self.prefix as i16;
        let stem1 = self.stem as i16;
        let prefix2 = other.prefix as i16;
        let stem2 = other.stem as i16;

        let prefix_diff = prefix2 - prefix1;
        let stem_diff = stem2 - stem1;

        (prefix_diff + 36 * (stem_diff - prefix_diff)).interval_modulus(1, 42)
    }

    fn get_stem(self) -> AkanStem {
        self.stem
    }

    fn get_prefix(self) -> AkanPrefix {
        self.prefix
    }

    fn day_name_on_or_before(self, date: FixedDate) -> Result<FixedDate, CalendarError> {
        let date = i64::from(date);
        let diff = AkanDay::name_difference(AkanDay::from(FixedDate::from(0)), self) as i64;
        FixedDate::try_from(diff.interval_modulus(date, date - 42))
    }
}

impl Epoch for AkanDay {
    fn epoch() -> FixedDate {
        const CYCLE_START: i64 = 37;
        FixedDate::try_from(CYCLE_START).expect("Epoch known to be within bounds.")
    }
}

impl From<FixedDate> for AkanDay {
    fn from(date: FixedDate) -> AkanDay {
        AkanDay::day_name(i64::from(date) - i64::from(AkanDay::epoch()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    proptest! {
        #[test]
        fn akan_day_repeats(x in i32::MIN..(i32::MAX - 42)) {
            let a1 = AkanDay::from(FixedDate::from(x));
            let a2 = AkanDay::from(FixedDate::from(x + 1));
            let a3 = AkanDay::from(FixedDate::from(x + 42));
            assert_ne!(a1, a2);
            assert_eq!(a1, a3);
        }
    }
}

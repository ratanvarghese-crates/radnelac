use crate::day_count::BoundedDayCount;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_cycle::BoundedCycle;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

/// Represents a day in the common week cycle
///
/// Note that some calendars such as the ISO calendar assign different numbers to each day.
/// See `ISO` for details about that.
///
/// Additionally, some calendars use different week cycles than the common week cycle. Some
/// calendars have seperate data types representing their weekdays, such as `FrenchRevArith`
/// which has a ten-day week. Other calendars which use the same names for their weekdays
/// re-use the `Weekday` struct such as `Cotsworth`. Monday in the common week cycle does
/// not necessarily correspond to Monday in the Cotsworth calendar.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Weekday {
    Sunday = 0,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl BoundedCycle<7, 0> for Weekday {}

impl FromFixed for Weekday {
    fn from_fixed(t: Fixed) -> Weekday {
        Weekday::from_unbounded(t.get_day_i())
    }
}

impl Weekday {
    fn raw_on_or_before(self, date: i64) -> Fixed {
        let k = self as i64;
        Fixed::cast_new(date - (Weekday::from_unbounded(date - k) as i64))
    }

    pub fn on_or_before(self, date: Fixed) -> Fixed {
        self.raw_on_or_before(date.get_day_i())
    }

    pub fn on_or_after(self, date: Fixed) -> Fixed {
        self.raw_on_or_before(date.get_day_i() + 6)
    }

    pub fn nearest(self, date: Fixed) -> Fixed {
        self.raw_on_or_before(date.get_day_i() + 3)
    }

    pub fn before(self, date: Fixed) -> Fixed {
        self.raw_on_or_before(date.get_day_i() - 1)
    }

    pub fn after(self, date: Fixed) -> Fixed {
        self.raw_on_or_before(date.get_day_i() + 7)
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
        fn day_of_week_sequence(x in (FIXED_MIN+14.0)..(FIXED_MAX - 14.0)) {
            let w = Weekday::Sunday;
            let d0 = w.on_or_before(Fixed::new(x));
            let d1 = Fixed::new(d0.get() + 1.0);
            let d2 = Fixed::new(d0.get() + 2.0);
            let d3 = Fixed::new(d0.get() + 3.0);
            let d4 = Fixed::new(d0.get() + 4.0);
            let d5 = Fixed::new(d0.get() + 5.0);
            let d6 = Fixed::new(d0.get() + 6.0);
            assert_eq!(Weekday::from_fixed(d0), Weekday::Sunday);
            assert_eq!(Weekday::from_fixed(d1), Weekday::Monday);
            assert_eq!(Weekday::from_fixed(d2), Weekday::Tuesday);
            assert_eq!(Weekday::from_fixed(d3), Weekday::Wednesday);
            assert_eq!(Weekday::from_fixed(d4), Weekday::Thursday);
            assert_eq!(Weekday::from_fixed(d5), Weekday::Friday);
            assert_eq!(Weekday::from_fixed(d6), Weekday::Saturday);
        }

        #[test]
        fn day_of_week_repeats(x in FIXED_MIN..(FIXED_MAX - 7.0)) {
            let f1 = Fixed::new(x);
            let a1 = Weekday::from_fixed(f1);
            let a2 = Weekday::from_fixed(Fixed::new(f1.get() + 1.0));
            let a3 = Weekday::from_fixed(Fixed::new(f1.get() + 7.0));
            assert_ne!(a1, a2);
            assert_eq!(a1, a3);
        }

        #[test]
        fn day_of_week_on_or_before(x1 in (FIXED_MIN+14.0)..FIXED_MAX, w in 0..6) {
            let w = Weekday::from_i64(w as i64).unwrap();
            let d1 = w.on_or_before(Fixed::new(x1));
            let d2 = w.on_or_before(d1);
            assert_eq!(d1, d2);
            let x2 = d2.get_day_i() as i32;
            for i in 1..6 {
                let d3 = w.on_or_before(Fixed::cast_new(x2 - i));
                assert_ne!(d1, d3);
            }
        }

        #[test]
        fn day_of_week_nearby(x1 in (FIXED_MIN+14.0)..(FIXED_MAX-14.0), w in 0..6) {
            let w = Weekday::from_i64(w as i64).unwrap();
            let f0 = Fixed::cast_new(x1 as i64);
            let f1 = w.on_or_before(f0);
            let f2 = w.on_or_after(f0);
            let f3 = w.nearest(f0);
            let f4 = w.before(f0);
            let f5 = w.after(f0);
            assert!(f1 <= f0);
            assert!(f2 >= f0);
            assert!(f4 < f0);
            assert!(f5 > f0);

            let diff = f0.get() - f3.get();
            assert!(-4.0 <= diff && diff <= 4.0);
        }
    }
}

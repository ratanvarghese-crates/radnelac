use crate::common::bound::BoundedCycle;
use crate::common::bound::BoundedDayCount;
use crate::common::error::CalendarError;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
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
    pub fn unchecked_on_or_before(self, date: i64) -> i64 {
        let k = self as i64;
        date - (Weekday::from_unbounded(date - k) as i64)
    }

    pub fn on_or_before(self, date: Fixed) -> Result<Fixed, CalendarError> {
        Fixed::cast_new(self.unchecked_on_or_before(date.get_day_i()))
    }

    pub fn unchecked_on_or_after(self, date: i64) -> i64 {
        self.unchecked_on_or_before(date + 6)
    }

    pub fn on_or_after(self, date: Fixed) -> Result<Fixed, CalendarError> {
        Fixed::cast_new(self.unchecked_on_or_after(date.get_day_i()))
    }

    pub fn unchecked_nearest(self, date: i64) -> i64 {
        self.unchecked_on_or_before(date + 3)
    }

    pub fn nearest(self, date: Fixed) -> Result<Fixed, CalendarError> {
        Fixed::cast_new(self.unchecked_on_or_before(date.get_day_i() + 3))
    }

    pub fn unchecked_before(self, date: i64) -> i64 {
        self.unchecked_on_or_before(date - 1)
    }

    pub fn before(self, date: Fixed) -> Result<Fixed, CalendarError> {
        Fixed::cast_new(self.unchecked_on_or_before(date.get_day_i() - 1))
    }

    pub fn unchecked_after(self, date: i64) -> i64 {
        self.unchecked_on_or_before(date + 7)
    }

    pub fn after(self, date: Fixed) -> Result<Fixed, CalendarError> {
        Fixed::cast_new(self.unchecked_on_or_before(date.get_day_i() + 7))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::math::EFFECTIVE_MAX;
    use crate::common::math::EFFECTIVE_MIN;
    use proptest::proptest;

    proptest! {
        #[test]
        fn day_of_week_sequence(x in (EFFECTIVE_MIN+14.0)..(EFFECTIVE_MAX - 14.0)) {
            let w = Weekday::Sunday;
            let d0 = w.on_or_before(Fixed::checked_new(x).unwrap()).unwrap();
            let d1 = d0.checked_add(1).unwrap();
            let d2 = d0.checked_add(2).unwrap();
            let d3 = d0.checked_add(3).unwrap();
            let d4 = d0.checked_add(4).unwrap();
            let d5 = d0.checked_add(5).unwrap();
            let d6 = d0.checked_add(6).unwrap();
            assert_eq!(Weekday::from_fixed(d0), Weekday::Sunday);
            assert_eq!(Weekday::from_fixed(d1), Weekday::Monday);
            assert_eq!(Weekday::from_fixed(d2), Weekday::Tuesday);
            assert_eq!(Weekday::from_fixed(d3), Weekday::Wednesday);
            assert_eq!(Weekday::from_fixed(d4), Weekday::Thursday);
            assert_eq!(Weekday::from_fixed(d5), Weekday::Friday);
            assert_eq!(Weekday::from_fixed(d6), Weekday::Saturday);
        }

        #[test]
        fn day_of_week_repeats(x in EFFECTIVE_MIN..(EFFECTIVE_MAX - 7.0)) {
            let f1 = Fixed::checked_new(x).unwrap();
            let a1 = Weekday::from_fixed(f1);
            let a2 = Weekday::from_fixed(f1.checked_add(1).unwrap());
            let a3 = Weekday::from_fixed(f1.checked_add(7).unwrap());
            assert_ne!(a1, a2);
            assert_eq!(a1, a3);
        }

        #[test]
        fn day_of_week_on_or_before(x1 in (EFFECTIVE_MIN+14.0)..EFFECTIVE_MAX, w in 0..6) {
            let w = Weekday::from_i64(w as i64).unwrap();
            let d1 = w.on_or_before(Fixed::checked_new(x1).unwrap()).unwrap();
            let d2 = w.on_or_before(d1).unwrap();
            assert_eq!(d1, d2);
            let x2 = d2.get_day_i() as i32;
            for i in 1..6 {
                let d3 = w.on_or_before(Fixed::new(x2 - i)).unwrap();
                assert_ne!(d1, d3);
            }
        }

        #[test]
        fn day_of_week_nearby(x1 in (EFFECTIVE_MIN+14.0)..(EFFECTIVE_MAX-14.0), w in 0..6) {
            let w = Weekday::from_i64(w as i64).unwrap();
            let f0 = Fixed::cast_new(x1 as i64).unwrap();
            let f1 = w.on_or_before(f0).unwrap();
            let f2 = w.on_or_after(f0).unwrap();
            let f3 = w.nearest(f0).unwrap();
            let f4 = w.before(f0).unwrap();
            let f5 = w.after(f0).unwrap();
            assert!(f1 <= f0);
            assert!(f2 >= f0);
            assert!(f4 < f0);
            assert!(f5 > f0);

            let diff = f0.clamped_sub(f3.get()).get();
            assert!(-4.0 <= diff && diff <= 4.0);
        }
    }
}

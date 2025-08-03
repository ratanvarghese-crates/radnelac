use crate::day_count::BoundedDayCount;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_cycle::BoundedCycle;
use crate::day_cycle::OnOrBefore;
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

impl OnOrBefore<7, 0> for Weekday {
    fn raw_on_or_before(self, date: i64) -> Fixed {
        let k = self.to_unbounded();
        Fixed::cast_new(date - (Weekday::from_unbounded(date - k) as i64))
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
    }
}

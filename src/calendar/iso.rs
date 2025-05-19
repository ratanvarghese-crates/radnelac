use crate::calendar::gregorian::Gregorian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::TryFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;
use crate::day_cycle::week::Weekday;
use num_traits::FromPrimitive;
use std::num::NonZero;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ISO {
    year: i32,
    week: NonZero<u8>,
    day: Weekday,
}

impl ISO {
    pub fn year(self) -> i32 {
        self.year
    }

    pub fn week(self) -> NonZero<u8> {
        self.week
    }

    pub fn day(self) -> Weekday {
        self.day
    }

    pub fn is_long_year(i_year: i16) -> bool {
        let jan1 = Weekday::from_fixed(Gregorian::new_year(i_year));
        let dec31 = Weekday::from_fixed(Gregorian::year_end(i_year));
        jan1 == Weekday::Thursday || dec31 == Weekday::Thursday
    }

    pub fn new_year(year: i32) -> Self {
        ISO {
            year: year,
            week: NonZero::new(1).unwrap(),
            day: Weekday::Monday,
        }
    }

    pub fn try_to_fixed(self) -> Result<Fixed, CalendarError> {
        let g = Gregorian::try_from_common_date(CommonDate::new(self.year - 1, 12, 28))?;
        let w = NonZero::<i16>::from(self.week);
        g.nth_kday(w, Weekday::Sunday)?.checked_add(self.day as i64)
    }

    pub fn try_from_fixed(date: Fixed) -> Result<Self, CalendarError> {
        let approx = Gregorian::ordinal_from_fixed_generic_unchecked(
            date.get_day_i() - 3,
            Gregorian::epoch().get_day_i(),
        )
        .year;
        let next = ISO::new_year(approx + 1).try_to_fixed()?;
        let year = if date >= next { approx + 1 } else { approx };
        let current = ISO::new_year(year).try_to_fixed()?;
        let week = (date.get_day_i() - current.get_day_i()).div_euclid(7) + 1;
        debug_assert!(week < 55 && week > 0);
        let day = Weekday::from_u8(date.get_day_i().adjusted_remainder(7) as u8)
            .expect("In range due to adjusted remainder.");
        Ok(ISO {
            year,
            week: NonZero::new(week as u8).unwrap(),
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::common::bound::EffectiveBound;

    #[test]
    fn week_of_impl() {
        let g = Gregorian::try_from_common_date(CommonDate::new(2025, 5, 15))
            .unwrap()
            .to_fixed();
        let i = ISO::try_from_fixed(g).unwrap();
        assert_eq!(i.week().get(), 20);
    }

    // #[test]
    // fn bounds() {
    //     assert!(ISO::try_from_fixed(Fixed::effective_min()).is_ok());
    //     assert!(ISO::try_from_fixed(Fixed::effective_max()).is_ok());
    // }
}

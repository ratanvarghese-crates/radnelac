use crate::calendar::common::CommonDate;
use crate::calendar::common::CommonDayOfWeek;
use crate::calendar::gregorian::Gregorian;
use crate::epoch::fixed::Fixed;
use crate::epoch::fixed::FixedDate;
use crate::error::CalendarError;
use crate::math::TermNum;
use std::num::NonZero;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ISO {
    year: i32,
    week: NonZero<u8>,
    day: CommonDayOfWeek,
}

impl ISO {
    fn get_year(self) -> i32 {
        self.year
    }

    fn get_week(self) -> NonZero<u8> {
        self.week
    }

    fn get_day(self) -> CommonDayOfWeek {
        self.day
    }

    fn is_long_year(i_year: i16) -> bool {
        let jan1 = CommonDayOfWeek::from(Gregorian::new_year(i_year));
        let dec31 = CommonDayOfWeek::from(Gregorian::year_end(i_year));
        jan1 == CommonDayOfWeek::Thursday || dec31 == CommonDayOfWeek::Thursday
    }

    fn new_year(year: i32) -> Self {
        ISO {
            year: year,
            week: NonZero::new(1).unwrap(),
            day: CommonDayOfWeek::Monday,
        }
    }
}

impl TryFrom<ISO> for FixedDate {
    type Error = CalendarError;
    fn try_from(date: ISO) -> Result<FixedDate, Self::Error> {
        let g = Gregorian::try_from(CommonDate::try_new(date.year - 1, 12, 28)?)?;
        let w = NonZero::new(date.week.get() as i16).expect("TODO");
        FixedDate::try_from(g.nth_kday(w, CommonDayOfWeek::Sunday)? + (date.day as i64))
    }
}

impl TryFrom<FixedDate> for ISO {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<ISO, Self::Error> {
        let approx =
            Gregorian::year_and_ord_day_from_fixed(FixedDate::try_from(i64::from(date) - 3)?);
        let next = Fixed::try_from(ISO::new_year(approx.0 + 1))?;
        let year = if date >= next { approx.0 + 1 } else { approx.0 };
        let current = Fixed::try_from(ISO::new_year(year))?;
        let week = (date - current).div_euclid(7) + 1;
        debug_assert!(week < 55 && week > 0);
        let day = CommonDayOfWeek::from(i64::from(date).adjusted_remainder(7));
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

    #[test]
    fn week_of_impl() {
        let g = FixedDate::from(Gregorian::try_from(CommonDate::new(2025, 5, 15)).unwrap());
        let i = ISO::try_from(g).unwrap();
        assert_eq!(i.get_week().get(), 20);
    }
}

use crate::calendar::egyptian::Egyptian;
use crate::day_count::BoundedDayCount;
use crate::calendar::CommonDate;
use crate::calendar::CommonDay;
use crate::calendar::CommonYear;
use crate::calendar::Quarter;
use crate::calendar::ToFromCommonDate;
use crate::calendar::TryMonth;
use crate::common::error::CalendarError;
use crate::calendar::CommonWeekOfYear;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::RataDie;
use crate::day_count::ToFixed;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;
use std::num::NonZero;

const ARMENIAN_EPOCH_RD: i32 = 201443;
const NON_MONTH: u8 = 13;

/// Represents a month in the Armenian Calendar
///
/// Note that the epagomenal days at the end of the Armenian calendar year have no
/// month and thus are not represented by ArmenianMonth. When representing an
/// arbitrary day in the Armenian calendar, use an `Option<ArmenianMonth>` for the
/// the month field.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum ArmenianMonth {
    Nawasardi = 1,
    Hori,
    Sahmi,
    Tre,
    Kaloch,
    Arach,
    Mehekani,
    Areg,
    Ahekani,
    Mareri,
    Margach,
    Hrotich,
}

/// Represents a day of month in the Armenian Calendar
///
/// The Armenian calendar has name for each day of month instead of a number.
/// Note that the epagomenal days at the end of the Armenian calendar year have no
/// month therefore they also do not have names.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum ArmenianDaysOfMonth {
    Areg = 1,
    Hrand,
    Aram,
    Margar,
    Ahrank,
    Mazdel,
    Astlik,
    Mihr,
    Jopaber,
    Murc,
    Erezhan,
    Ani,
    Parkhar,
    Vanat,
    Aramazd,
    Mani,
    Asak,
    Masis,
    Anahit,
    Aragats,
    Gorgor,
    Kordvik,
    Tsmak,
    Lusnak,
    Tsron,
    Npat,
    Vahagn,
    Sim,
    Varag,
    Giseravar,
}

/// Represents a date in the Armenian calendar
///
/// Further reading
/// + [Wikipedia](https://en.wikipedia.org/wiki/Armenian_calendar)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Armenian(CommonDate);

impl Armenian {
    /// Returns the day name of month if one exists
    pub fn day_name(self) -> Option<ArmenianDaysOfMonth> {
        if self.0.month == NON_MONTH {
            None
        } else {
            ArmenianDaysOfMonth::from_u8(self.0.day)
        }
    }
}

impl CalculatedBounds for Armenian {}

impl Epoch for Armenian {
    fn epoch() -> Fixed {
        RataDie::new(ARMENIAN_EPOCH_RD as f64).to_fixed()
    }
}

impl FromFixed for Armenian {
    fn from_fixed(date: Fixed) -> Armenian {
        let f = Fixed::new(date.get() + Egyptian::epoch().to_day().get() - Armenian::epoch().get());
        Armenian::try_from_common_date(Egyptian::from_fixed(f).to_common_date())
            .expect("Same month/day validity")
    }
}

impl ToFixed for Armenian {
    fn to_fixed(self) -> Fixed {
        let e =
            Egyptian::try_from_common_date(self.to_common_date()).expect("Same month/day validity");
        Fixed::new(Armenian::epoch().get() + e.to_fixed().get() - Egyptian::epoch().to_day().get())
    }
}

/// The epagomenal days at the end of the Armenian calendar year are represented
/// as month 13 when converting to and from a CommonDate.
impl ToFromCommonDate for Armenian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        Egyptian::valid_month_day(date)
    }

    fn year_end_date(year: i32) -> CommonDate {
        Egyptian::year_end_date(year)
    }
}

impl Quarter for Armenian {
    fn quarter(self) -> NonZero<u8> {
        let m = self.to_common_date().month as u8;
        if m == NON_MONTH {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new(((m - 1) / 3) + 1).expect("(m - 1) / 3 > -1")
        }
    }
}

impl CommonYear for Armenian {}
impl TryMonth<ArmenianMonth> for Armenian {}
impl CommonDay for Armenian {}
impl CommonWeekOfYear for Armenian {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::FIXED_MAX;
    use proptest::proptest;
    const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;
    proptest! {
        #[test]
        fn day_names(y0 in -MAX_YEARS..MAX_YEARS, y1 in -MAX_YEARS..MAX_YEARS, m in 1..12, d in 1..30) {
            let a0 = Armenian::try_from_common_date(CommonDate::new(y0, m as u8, d as u8)).unwrap();
            let a1 = Armenian::try_from_common_date(CommonDate::new(y1, m as u8, d as u8)).unwrap();
            assert_eq!(a0.day_name(), a1.day_name())
        }

        #[test]
        fn day_names_m13(y0 in -MAX_YEARS..MAX_YEARS, y1 in -MAX_YEARS..MAX_YEARS, d in 1..5) {
            let a0 = Armenian::try_from_common_date(CommonDate::new(y0, 13, d as u8)).unwrap();
            let a1 = Armenian::try_from_common_date(CommonDate::new(y1, 13, d as u8)).unwrap();
            assert!(a0.day_name().is_none());
            assert!(a1.day_name().is_none());
        }
    }
}

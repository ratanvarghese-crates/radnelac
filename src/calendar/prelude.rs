use crate::common::error::CalendarError;
use crate::day_count::BoundedDayCount;
use crate::day_count::EffectiveBound;
use crate::day_count::Fixed;
use crate::day_count::ToFixed;
use crate::day_cycle::OnOrBefore;
use crate::day_cycle::Weekday;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use std::num::NonZero;

pub trait HasLeapYears {
    fn is_leap(year: i32) -> bool;
}

/// Represents a combination of numeric year, month and day
///
/// This is not specific to any particular calendar system.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct CommonDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl CommonDate {
    pub fn new(year: i32, month: u8, day: u8) -> CommonDate {
        CommonDate { year, month, day }
    }
}

pub trait ToFromCommonDate<T: FromPrimitive>: Sized + EffectiveBound {
    fn to_common_date(self) -> CommonDate;
    fn from_common_date_unchecked(d: CommonDate) -> Self;
    fn valid_month_day(d: CommonDate) -> Result<(), CalendarError>;
    fn year_start_date(year: i32) -> CommonDate {
        CommonDate::new(year, 1, 1)
    }
    fn year_end_date(year: i32) -> CommonDate;

    fn in_effective_bounds(d: CommonDate) -> bool {
        let min = Self::effective_min().to_common_date();
        let max = Self::effective_max().to_common_date();
        d >= min && d <= max
    }

    fn try_from_common_date(d: CommonDate) -> Result<Self, CalendarError> {
        match Self::valid_month_day(d) {
            Err(e) => Err(e),
            Ok(_) => Ok(Self::from_common_date_unchecked(d)),
        }
    }

    fn try_year_start(year: i32) -> Result<Self, CalendarError> {
        //Might be invalid for calendars without year 0
        let d = Self::year_start_date(year);
        debug_assert!(Self::in_effective_bounds(d));
        Self::try_from_common_date(d)
    }

    fn try_year_end(year: i32) -> Result<Self, CalendarError> {
        //Might be invalid for calendars without year 0
        let d = Self::year_end_date(year);
        debug_assert!(Self::in_effective_bounds(d));
        Self::try_from_common_date(d)
    }

    fn day(self) -> u8 {
        self.to_common_date().day
    }

    fn try_month(self) -> Option<T> {
        T::from_u8(self.to_common_date().month)
    }

    fn year(self) -> i32 {
        self.to_common_date().year
    }
}

pub trait GuaranteedMonth<T: ToPrimitive + FromPrimitive>: ToFromCommonDate<T> {
    fn month(self) -> T {
        self.try_month().expect("Month is guaranteed")
    }

    fn try_new(year: i32, month: T, day: u8) -> Result<Self, CalendarError> {
        let m = month.to_u8().expect("Month is correct type");
        Self::try_from_common_date(CommonDate::new(year, m, day))
    }
}

pub trait HasIntercalaryDays<T: FromPrimitive + ToPrimitive> {
    fn complementary(self) -> Option<T>;
    fn complementary_count(year: i32) -> u8;
}

pub trait Perennial<S, T>: ToFromCommonDate<S>
where
    S: ToPrimitive + FromPrimitive,
    T: FromPrimitive + ToPrimitive,
{
    fn weekday(self) -> Option<T>;
    fn days_per_week() -> u8;
    fn weeks_per_month() -> u8;

    fn try_week_of_year(self) -> Option<u8> {
        match (self.weekday(), self.try_month()) {
            (Some(_), Some(month)) => {
                let d = self.day() as i64;
                let m = month.to_i64().expect("Guaranteed in range");
                let dpw = Self::days_per_week() as i64;
                let wpm = Self::weeks_per_month() as i64;
                let dpm = dpw * wpm;
                Some(((((m - 1) * dpm) + d - 1) / dpw + 1) as u8)
            }
            (_, _) => None,
        }
    }
}

pub trait Quarter {
    fn quarter(self) -> NonZero<u8>;
}

pub trait CommonWeekOfYear<T: FromPrimitive>: ToFromCommonDate<T> + ToFixed {
    fn week_of_year(self) -> u8 {
        let today = self.to_fixed();
        let start = Self::try_year_start(self.year())
            .expect("Year known to be valid")
            .to_fixed();
        let diff = today.get_day_i() - start.get_day_i();
        (diff.div_euclid(7) + 1) as u8
    }

    //Arguments swapped from the original
    fn nth_kday(self, nz: NonZero<i16>, k: Weekday) -> Fixed {
        let n = nz.get();
        let result = if n > 0 {
            k.before(self.to_fixed()).get_day_i() + (7 * n as i64)
        } else {
            k.after(self.to_fixed()).get_day_i() + (7 * n as i64)
        };
        Fixed::cast_new(result)
    }
}

/// Represents a numeric year and day of year.
///
/// This is not specific to any particular calendar system.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct OrdinalDate {
    pub year: i32,
    pub day_of_year: u16,
}

pub trait ToFromOrdinalDate: Sized {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError>;
    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate;
    fn to_ordinal(self) -> OrdinalDate;
    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self;
    fn try_from_ordinal(ord: OrdinalDate) -> Result<Self, CalendarError> {
        match Self::valid_ordinal(ord) {
            Err(e) => Err(e),
            Ok(_) => Ok(Self::from_ordinal_unchecked(ord)),
        }
    }
}

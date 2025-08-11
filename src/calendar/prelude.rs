// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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

/// Calendar systems with leap years
pub trait HasLeapYears {
    /// [`true`] if a the given year is a leap year.
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
    /// Create a `CommonDate`
    pub fn new(year: i32, month: u8, day: u8) -> CommonDate {
        CommonDate { year, month, day }
    }
}

/// Calendar systems in which a date can be represented by a year, month and day
pub trait ToFromCommonDate<T: FromPrimitive>: Sized + EffectiveBound {
    /// Convert calendar date to a year, month and day
    fn to_common_date(self) -> CommonDate;
    /// Convert a year, month and day into a calendar date without checking validity
    ///
    /// In almost all cases, [`try_from_common_date`](ToFromCommonDate::try_from_common_date) is preferred.
    fn from_common_date_unchecked(d: CommonDate) -> Self;
    /// Returns error if the year, month or day is invalid
    fn valid_month_day(d: CommonDate) -> Result<(), CalendarError>;
    /// Start of the year as a numeric year, month and day
    fn year_start_date(year: i32) -> CommonDate {
        //LISTING 2.18 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to be generalized across many calendar systems
        CommonDate::new(year, 1, 1)
    }
    /// End of the year as a numeric year, month and day
    fn year_end_date(year: i32) -> CommonDate;

    /// [`true`] if the year, month and day is within the supported range of time.
    ///
    /// This does not check the validity of the date.
    fn in_effective_bounds(d: CommonDate) -> bool {
        let min = Self::effective_min().to_common_date();
        let max = Self::effective_max().to_common_date();
        d >= min && d <= max
    }

    /// Attempt to create a date in a specific calendar from a [`CommonDate`]
    fn try_from_common_date(d: CommonDate) -> Result<Self, CalendarError> {
        match Self::valid_month_day(d) {
            Err(e) => Err(e),
            Ok(_) => Ok(Self::from_common_date_unchecked(d)),
        }
    }

    /// Attempt to create a date in a specific calendar at the start of a specific year
    ///
    /// This may return an error if the year is 0, and the implementor does not support
    /// year 0.
    fn try_year_start(year: i32) -> Result<Self, CalendarError> {
        //Might be invalid for calendars without year 0
        let d = Self::year_start_date(year);
        debug_assert!(Self::in_effective_bounds(d));
        Self::try_from_common_date(d)
    }

    /// Attempt to create a date in a specific calendar at the end of a specific year
    ///
    /// This may return an error if the year is 0, and the implementor does not support
    /// year 0.
    fn try_year_end(year: i32) -> Result<Self, CalendarError> {
        //Might be invalid for calendars without year 0
        let d = Self::year_end_date(year);
        debug_assert!(Self::in_effective_bounds(d));
        Self::try_from_common_date(d)
    }

    fn day(self) -> u8 {
        self.to_common_date().day
    }

    /// Attempt to return the month
    ///
    /// In some calendars, certain dates are not associated with a month, which is why
    /// this function returns an [`Option`].
    ///
    /// Callers using a calendar which guarantees that every date has a month
    /// should use [`GuaranteedMonth::month`].
    fn try_month(self) -> Option<T> {
        T::from_u8(self.to_common_date().month)
    }

    fn year(self) -> i32 {
        self.to_common_date().year
    }
}

/// Calendar systems in which dates which are guaranteed to have a month
pub trait GuaranteedMonth<T: FromPrimitive + ToPrimitive>: ToFromCommonDate<T> {
    fn month(self) -> T {
        self.try_month().expect("Month is guaranteed")
    }

    /// Attempt to a date in a specific calendar system
    fn try_new(year: i32, month: T, day: u8) -> Result<Self, CalendarError> {
        let m = month.to_u8().expect("Month is correct type");
        Self::try_from_common_date(CommonDate::new(year, m, day))
    }
}

/// Calendar systems which have intercalary days
pub trait HasIntercalaryDays<T: FromPrimitive + ToPrimitive> {
    fn complementary(self) -> Option<T>;
    fn complementary_count(year: i32) -> u8;
}

/// Calendar systems which are perennial
pub trait Perennial<S, T>: ToFromCommonDate<S>
where
    S: FromPrimitive + ToPrimitive,
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

/// Calendar systems in which a year can be divided into quarters
///
/// The quarters may not have exactly the same number of days.
pub trait Quarter {
    /// Calculate the quarter associated with a particular date.
    ///
    /// This must be with the range [1..4] inclusive.
    fn quarter(self) -> NonZero<u8>;
}

/// Calendar systems in which a week of year can be calculated for a date
pub trait CommonWeekOfYear<T: FromPrimitive>: ToFromCommonDate<T> + ToFixed {
    /// Calculate the week of year for a particular date.
    fn week_of_year(self) -> u8 {
        let today = self.to_fixed();
        let start = Self::try_year_start(self.year())
            .expect("Year known to be valid")
            .to_fixed();
        let diff = today.get_day_i() - start.get_day_i();
        (diff.div_euclid(7) + 1) as u8
    }

    /// Find the nth occurence of a given day of the week
    fn nth_kday(self, nz: NonZero<i16>, k: Weekday) -> Fixed {
        //LISTING 2.33 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Arguments swapped from the original, generalized to arbitrary calendar systems
        let n = nz.get();
        let result = if n > 0 {
            k.before(self.to_fixed()).get_day_i() + (7 * n as i64)
        } else {
            k.after(self.to_fixed()).get_day_i() + (7 * n as i64)
        };
        Fixed::cast_new(result)
    }

    //TODO: first_kday (listing 2.34)
    //TODO: last_kday (listing 2.34)
}

/// Represents a numeric year and day of year.
///
/// This is not specific to any particular calendar system.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct OrdinalDate {
    pub year: i32,
    pub day_of_year: u16,
}

/// Calendar systems in which a date can be represented by a year and day of year
pub trait ToFromOrdinalDate: Sized {
    /// Check if the year and day of year is valid for a particular calendar system
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError>;
    /// Calculate the year and day of year from a [`Fixed`].
    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate;
    /// Calculate the year and day of year from a calendar date.
    fn to_ordinal(self) -> OrdinalDate;
    /// Convert a year and day of year into a calendar date without checking validity
    ///
    /// In almost all cases, [`try_from_ordinal`](ToFromOrdinalDate::try_from_ordinal) is preferred.
    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self;
    /// Attempt to create a date in a specific calendar from an [`OrdinalDate`]
    fn try_from_ordinal(ord: OrdinalDate) -> Result<Self, CalendarError> {
        match Self::valid_ordinal(ord) {
            Err(e) => Err(e),
            Ok(_) => Ok(Self::from_ordinal_unchecked(ord)),
        }
    }
}

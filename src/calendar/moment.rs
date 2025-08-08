use crate::calendar::CommonDate;
use crate::calendar::CommonWeekOfYear;
use crate::calendar::GuaranteedMonth;
use crate::calendar::HasIntercalaryDays;
use crate::calendar::HasLeapYears;
use crate::calendar::OrdinalDate;
use crate::calendar::Perennial;
use crate::calendar::Quarter;
use crate::calendar::ToFromCommonDate;
use crate::calendar::ToFromOrdinalDate;
use crate::clock::ClockTime;
use crate::clock::TimeOfDay;
use crate::day_count::BoundedDayCount;
use crate::day_count::EffectiveBound;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::CalendarError;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use std::num::NonZero;

/// Represents an instant in time using calendar system T
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct CalendarMoment<T> {
    date: T,
    time: ClockTime,
}

impl<T> CalendarMoment<T> {
    /// Create a CalendarMoment with the given date and time
    pub fn new(date: T, t: TimeOfDay) -> Self {
        Self {
            date: date,
            time: t.to_clock(),
        }
    }

    pub fn date(self) -> T {
        self.date
    }

    pub fn time_of_day(self) -> ClockTime {
        self.time
    }
}

impl<T: FromFixed> FromFixed for CalendarMoment<T> {
    fn from_fixed(fixed_date: Fixed) -> Self {
        Self::new(T::from_fixed(fixed_date), TimeOfDay::from_fixed(fixed_date))
    }
}

impl<T: ToFixed> ToFixed for CalendarMoment<T> {
    fn to_fixed(self) -> Fixed {
        let fd = self.date.to_fixed();
        let ft = TimeOfDay::try_from_clock(self.time).expect("Guaranteed valid");
        Fixed::new(fd.to_day().get() + ft.get())
    }
}

impl<T: Epoch> Epoch for CalendarMoment<T> {
    fn epoch() -> Fixed {
        T::epoch()
    }
}

impl<T: EffectiveBound> EffectiveBound for CalendarMoment<T> {
    fn effective_min() -> Self {
        Self::new(T::effective_min(), TimeOfDay::midnight())
    }
    fn effective_max() -> Self {
        Self::new(T::effective_max(), TimeOfDay::midnight())
    }
}

impl<T: HasLeapYears> HasLeapYears for CalendarMoment<T> {
    fn is_leap(year: i32) -> bool {
        T::is_leap(year)
    }
}

impl<T, U> ToFromCommonDate<T> for CalendarMoment<U>
where
    T: FromPrimitive,
    U: ToFromCommonDate<T> + EffectiveBound,
{
    fn to_common_date(self) -> CommonDate {
        self.date.to_common_date()
    }

    fn from_common_date_unchecked(d: CommonDate) -> Self {
        Self::new(U::from_common_date_unchecked(d), TimeOfDay::midnight())
    }

    fn valid_month_day(d: CommonDate) -> Result<(), CalendarError> {
        U::valid_month_day(d)
    }

    fn year_start_date(year: i32) -> CommonDate {
        U::year_start_date(year)
    }

    fn year_end_date(year: i32) -> CommonDate {
        U::year_end_date(year)
    }

    fn in_effective_bounds(d: CommonDate) -> bool {
        U::in_effective_bounds(d)
    }

    fn try_from_common_date(d: CommonDate) -> Result<Self, CalendarError> {
        let d = U::try_from_common_date(d)?;
        Ok(Self::new(d, TimeOfDay::midnight()))
    }

    fn try_year_start(year: i32) -> Result<Self, CalendarError> {
        let d = U::try_year_start(year)?;
        Ok(Self::new(d, TimeOfDay::midnight()))
    }

    fn try_year_end(year: i32) -> Result<Self, CalendarError> {
        let d = U::try_year_end(year)?;
        Ok(Self::new(d, TimeOfDay::midnight()))
    }

    fn day(self) -> u8 {
        self.date.day()
    }

    fn try_month(self) -> Option<T> {
        self.date.try_month()
    }

    fn year(self) -> i32 {
        self.date.year()
    }
}

impl<T, U> GuaranteedMonth<T> for CalendarMoment<U>
where
    T: ToPrimitive + FromPrimitive,
    U: GuaranteedMonth<T>,
{
    fn month(self) -> T {
        self.date.month()
    }

    fn try_new(year: i32, month: T, day: u8) -> Result<Self, CalendarError> {
        let d = U::try_new(year, month, day)?;
        Ok(Self::new(d, TimeOfDay::midnight()))
    }
}

impl<T, U> HasIntercalaryDays<T> for CalendarMoment<U>
where
    T: ToPrimitive + FromPrimitive,
    U: HasIntercalaryDays<T>,
{
    fn complementary(self) -> Option<T> {
        self.date.complementary()
    }

    fn complementary_count(p_year: i32) -> u8 {
        U::complementary_count(p_year)
    }
}

impl<S, T, U> Perennial<S, T> for CalendarMoment<U>
where
    S: ToPrimitive + FromPrimitive,
    T: ToPrimitive + FromPrimitive,
    U: Perennial<S, T>,
{
    fn weekday(self) -> Option<T> {
        self.date.weekday()
    }

    fn days_per_week() -> u8 {
        U::days_per_week()
    }

    fn weeks_per_month() -> u8 {
        U::weeks_per_month()
    }

    fn try_week_of_year(self) -> Option<u8> {
        self.date.try_week_of_year()
    }
}

impl<T: Quarter> Quarter for CalendarMoment<T> {
    fn quarter(self) -> NonZero<u8> {
        self.date.quarter()
    }
}

impl<T, U> CommonWeekOfYear<T> for CalendarMoment<U>
where
    T: FromPrimitive,
    U: CommonWeekOfYear<T> + ToFixed,
{
    fn week_of_year(self) -> u8 {
        self.date.week_of_year()
    }

    fn nth_kday(self, nz: NonZero<i16>, k: Weekday) -> Fixed {
        self.date.nth_kday(nz, k)
    }
}

impl<T: ToFromOrdinalDate> ToFromOrdinalDate for CalendarMoment<T> {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        T::valid_ordinal(ord)
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        T::ordinal_from_fixed(fixed_date)
    }

    fn to_ordinal(self) -> OrdinalDate {
        self.date.to_ordinal()
    }

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        Self::new(T::from_ordinal_unchecked(ord), TimeOfDay::midnight())
    }
}

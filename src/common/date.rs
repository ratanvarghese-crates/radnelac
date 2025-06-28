use crate::common::bound::EffectiveBound;
use crate::common::error::CalendarError;
use crate::day_count::ToFixed;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use std::num::NonZero;

pub trait PerennialWithComplementaryDay<T, U>
where
    T: FromPrimitive + ToPrimitive,
    U: FromPrimitive + ToPrimitive,
{
    fn complementary(self) -> Option<T>;
    fn weekday(self) -> Option<U>;
    fn complementary_count(year: i32) -> u8;
    fn days_per_week() -> u8;
    fn weeks_per_month() -> u8;
}

pub trait HasLeapYears {
    fn is_leap(year: i32) -> bool;
}

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

pub trait ToFromCommonDate: Sized + EffectiveBound {
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

    fn year_start(year: i16) -> Self {
        let d = Self::year_start_date(year as i32);
        debug_assert!(Self::in_effective_bounds(d));
        Self::try_from_common_date(d).expect("Known to be in range")
    }

    fn year_end(year: i16) -> Self {
        let d = Self::year_end_date(year as i32);
        debug_assert!(Self::in_effective_bounds(d));
        Self::try_from_common_date(d).expect("Known to be in range")
    }
}

pub trait CommonYear: ToFromCommonDate {
    fn year(self) -> i32 {
        self.to_common_date().year
    }
}

pub trait TryMonth<T: FromPrimitive>: ToFromCommonDate {
    fn try_month(self) -> Option<T> {
        T::from_u8(self.to_common_date().month)
    }
}

pub trait GuaranteedMonth<T: FromPrimitive>: ToFromCommonDate {
    fn month(self) -> T {
        T::from_u8(self.to_common_date().month).expect("Will not allow setting invalid value.")
    }
}

impl<T: FromPrimitive + ToPrimitive, U: GuaranteedMonth<T>> TryMonth<T> for U {
    fn try_month(self) -> Option<T> {
        Some(self.month())
    }
}

pub trait CommonDay: ToFromCommonDate {
    fn day(self) -> u8 {
        self.to_common_date().day
    }
}

pub trait Quarter {
    fn quarter(self) -> NonZero<u8>;
}

pub trait CommonWeekOfYear: ToFromCommonDate + ToFixed + CommonYear {
    fn week_of_year(self) -> u8 {
        let today = self.to_fixed();
        let start = Self::try_from_common_date(Self::year_start_date(self.year()))
            .expect("New year should be valid for any date")
            .to_fixed();
        let diff = today.get_day_i() - start.get_day_i();
        (diff.div_euclid(7) + 1) as u8
    }
}

pub trait ComplementaryWeekOfYear<S, T, U>:
    CommonDay + TryMonth<S> + PerennialWithComplementaryDay<T, U>
where
    S: ToPrimitive + FromPrimitive,
    T: ToPrimitive + FromPrimitive,
    U: ToPrimitive + FromPrimitive,
{
    fn try_week_of_year(self) -> Option<u8> {
        match (self.complementary(), self.try_month()) {
            (None, Some(month)) => {
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct OrdinalDate {
    pub year: i32,
    pub day_of_year: u16,
}

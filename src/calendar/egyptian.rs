// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::CommonWeekOfYear;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::CalendarMoment;
use crate::calendar::HasIntercalaryDays;
use crate::calendar::OrdinalDate;
use crate::calendar::ToFromOrdinalDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::JulianDay;
use crate::day_count::ToFixed;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;
use std::num::NonZero;

const NABONASSAR_ERA_JD: i32 = 1448638;
const NON_MONTH: u8 = 13;

/// Represents a month in the Egyptian Calendar
///
/// Note that the epagomenal days at the end of the Egyptian calendar year have no
/// month and thus are not represented by ArmenianMonth. When representing an
/// arbitrary day in the Egyptian calendar, use an `Option<EgyptianMonth>` for the
/// the month field.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum EgyptianMonth {
    Thoth = 1,
    Phaophi,
    Athyr,
    Choiak,
    Tybi,
    Mechir,
    Phamenoth,
    Pharmuthi,
    Pachon,
    Payni,
    Epiphi,
    Mesori,
}

/// Represents an epagomenal day at the end of the Egyptian calendar year
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum EgyptianDaysUponTheYear {
    BirthOfOsiris = 1,
    BirthOfHorus,
    BirthOfSeth,
    BirthOfIsis,
    BirthOfNephthys,
}

/// Represents a date in the Egyptian calendar
///
/// ## Year 0
///
/// Year 0 is supported for this calendar.
///
/// ## Further reading
/// + [Wikipedia](https://en.wikipedia.org/wiki/Egyptian_calendar)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Egyptian(CommonDate);

impl ToFromOrdinalDate for Egyptian {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        if ord.day_of_year < 1 || ord.day_of_year > 365 {
            Err(CalendarError::InvalidDayOfYear)
        } else {
            Ok(())
        }
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        let days = fixed_date.get_day_i() - Egyptian::epoch().get_day_i();
        let year = (days.div_euclid(365) + 1) as i32;
        let doy = (days.modulus(365) + 1) as u16;
        OrdinalDate {
            year: year,
            day_of_year: doy,
        }
    }

    fn to_ordinal(self) -> OrdinalDate {
        let month = self.0.month as i64;
        let day = self.0.day as i64;
        let offset = ((30 * (month - 1)) + day) as u16;
        OrdinalDate {
            year: self.0.year,
            day_of_year: offset,
        }
    }

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        let month = (ord.day_of_year - 1).div_euclid(30) + 1;
        let day = ord.day_of_year - (30 * (month - 1));
        Egyptian(CommonDate::new(ord.year, month as u8, day as u8))
    }
}

impl CalculatedBounds for Egyptian {}

impl Epoch for Egyptian {
    fn epoch() -> Fixed {
        JulianDay::new(NABONASSAR_ERA_JD as f64).to_fixed()
    }
}

impl FromFixed for Egyptian {
    fn from_fixed(date: Fixed) -> Egyptian {
        let days = date.get_day_i() - Egyptian::epoch().get_day_i();
        let year = days.div_euclid(365) + 1;
        let month = days.modulus(365).div_euclid(30) + 1;
        let day = days - (365 * (year - 1)) - (30 * (month - 1)) + 1;
        Egyptian(CommonDate {
            year: year as i32,
            month: month as u8,
            day: day as u8,
        })
    }
}

impl ToFixed for Egyptian {
    fn to_fixed(self) -> Fixed {
        let year = self.0.year as i64;
        let month = self.0.month as i64;
        let day = self.0.day as i64;
        let offset = (365 * (year - 1)) + (30 * (month - 1)) + day - 1;
        Fixed::cast_new(Egyptian::epoch().get_day_i() + offset)
    }
}

impl ToFromCommonDate<EgyptianMonth> for Egyptian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        if date.month < 1 || date.month > NON_MONTH {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.month < NON_MONTH && date.day > 30 {
            Err(CalendarError::InvalidDay)
        } else if date.month == NON_MONTH && date.day > 5 {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        CommonDate::new(year, NON_MONTH, 5)
    }
}

impl HasIntercalaryDays<EgyptianDaysUponTheYear> for Egyptian {
    fn complementary(self) -> Option<EgyptianDaysUponTheYear> {
        if self.0.month == NON_MONTH {
            EgyptianDaysUponTheYear::from_u8(self.0.day)
        } else {
            None
        }
    }

    fn complementary_count(_year: i32) -> u8 {
        5
    }
}

impl Quarter for Egyptian {
    fn quarter(self) -> NonZero<u8> {
        let m = self.to_common_date().month as u8;
        if m == NON_MONTH {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new(((m - 1) / 3) + 1).expect("(m - 1) / 3 > -1")
        }
    }
}

impl CommonWeekOfYear<EgyptianMonth> for Egyptian {}

/// Represents a date *and time* in the Egyptian Calendar
pub type EgyptianMoment = CalendarMoment<Egyptian>;

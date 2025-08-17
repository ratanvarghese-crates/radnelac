// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::CommonWeekOfYear;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::AllowYearZero;
use crate::calendar::CalendarMoment;
use crate::calendar::HasEpagemonae;
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

//LISTING 1.46 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
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
    //LISTING ?? SECTION 1.11 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
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
/// ## Introduction
///
/// The Egyptian calendar was used in ancient Egypt.
///
/// ## Basic Structure
///
/// Years are always 365 days - there are no leap years. Years are divided into 12 months
/// of 30 days each, with an extra 5 epagomenal days.
///
/// ## Epoch
///
/// This implementation of the Egyptian calendar uses the Nabonassar Era from the *Almagest*
/// written by Claudius Ptolomy. The first year of the Nabonassar Era began on 26 Februrary
/// 747 BC of the Julian calendar.
///
/// The *Almagest* was written in Greek, and Nabonassar was a Babylonian king instead of an
/// Egyptian. The actual calendar dates in Ancient Egypt used regnal years.
///
/// ## Representation and Examples
///
/// The months are represented in this crate as [`EgyptianMonth`].
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_1_1 = CommonDate::new(1462, 1, 1);
/// let a_1_1 = Egyptian::try_from_common_date(c_1_1).unwrap();
/// assert_eq!(a_1_1.try_month().unwrap(), EgyptianMonth::Thoth);
/// let c_12_30 = CommonDate::new(1462, 12, 30);
/// let a_12_30 = Egyptian::try_from_common_date(c_12_30).unwrap();
/// assert_eq!(a_12_30.try_month().unwrap(), EgyptianMonth::Mesori);
/// ```
///
/// When converting to and from a [`CommonDate`](crate::calendar::CommonDate), the epagomenal days
/// are treated as a 13th month.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c = CommonDate::new(1462, 13, 5);
/// let a = Egyptian::try_from_common_date(c).unwrap();
/// assert!(a.try_month().is_none());
/// assert!(a.epagomenae().is_some());
/// ```
///
/// The start of the Nabonassar Era can be read programatically.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let e = Egyptian::epoch();
/// let j = Julian::from_fixed(e);
/// let a = Egyptian::from_fixed(e);
/// assert_eq!(j.year(), -747);
/// assert_eq!(j.month(), JulianMonth::February);
/// assert_eq!(j.day(), 26);
/// assert_eq!(a.year(), 1);
/// assert_eq!(a.try_month().unwrap(), EgyptianMonth::Thoth);
/// assert_eq!(a.day(), 1);
/// ```
///
/// ## Further reading
/// + Wikipedia
///   + [Egyptian Calendar](https://en.wikipedia.org/wiki/Egyptian_calendar)
///   + [Nabonassar](https://en.wikipedia.org/wiki/Nabonassar)
///   + [Egyptian chronology](https://en.wikipedia.org/wiki/Egyptian_chronology)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Egyptian(CommonDate);

impl AllowYearZero for Egyptian {}

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
        //LISTING 1.49 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
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
        //LISTING 1.47 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
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
        debug_assert!(Self::valid_ymd(date).is_ok());
        Self(date)
    }

    fn valid_ymd(date: CommonDate) -> Result<(), CalendarError> {
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

    fn month_length(_year: i32, _month: EgyptianMonth) -> u8 {
        30
    }
}

impl HasEpagemonae<EgyptianDaysUponTheYear> for Egyptian {
    fn epagomenae(self) -> Option<EgyptianDaysUponTheYear> {
        if self.0.month == NON_MONTH {
            EgyptianDaysUponTheYear::from_u8(self.0.day)
        } else {
            None
        }
    }

    fn epagomenae_count(_year: i32) -> u8 {
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

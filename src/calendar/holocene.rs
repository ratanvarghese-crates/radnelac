// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::gregorian::Gregorian;
use crate::calendar::gregorian::GregorianMonth;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::CommonWeekOfYear;
use crate::calendar::prelude::GuaranteedMonth;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::AllowYearZero;
use crate::calendar::CalendarMoment;
use crate::calendar::OrdinalDate;
use crate::calendar::ToFromOrdinalDate;
use crate::common::error::CalendarError;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use std::num::NonZero;

const HOLOCENE_YEAR_OFFSET: i32 = -10000;

/// Represents a month in the Holocene calendar
pub type HoloceneMonth = GregorianMonth;

/// Represents a date in the Holocene calendar
///
/// ## Introduction
///
/// The Holocene calendar was proposed by Cesare Emiliani. It is identical to the proleptic
/// Gregorian calendar, but with an extra 10000 years added to each date. Thus 2016 in the
/// Gregorian calendar is 12016 in the Holocene calendar.
///
/// ## Epoch
///
/// Years are numbered based on a very rough estimate of the invention of agriculture.
/// The first year of the Holocene calendar starts 10000 years before the first year of the
/// proleptic Gregorian calendar.
///
/// This epoch is called the Human Era.
///
/// ## Representation and Examples
///
/// ### Months
///
/// The months are represented in this crate as [`HoloceneMonth`].
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_1_1 = CommonDate::new(12025, 1, 1);
/// let a_1_1 = Holocene::try_from_common_date(c_1_1).unwrap();
/// assert_eq!(a_1_1.month(), HoloceneMonth::January);
/// ```
///
/// ### Conversion from Gregorian
///
/// For dates from other systems, it might be necessary to convert from the Gregorian system.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let g = Gregorian::try_new(1752, JulianMonth::September, 14).unwrap();
/// let h = g.convert::<Holocene>();
/// assert_eq!(h, Holocene::try_new(11752, GregorianMonth::September, 14).unwrap());
/// ```
///
/// ## Inconsistencies with Other Implementations
///
/// Since this crate uses a proleptic Gregorian calendar with a year 0, some of the
/// Gregorian conversions for dates before 1 Common Era may differ from other implementations.
///
/// For example, Wikipedia claims that 1 Human Era corresponds to "10000 BC" in the Gregorian
/// calendar and "-9999" in ISO-8601. However since this crate uses a proleptic Gregorian
/// calendar, "-9999" (or 9999 Before Common Era) is the Gregorian year corresponding to 1
/// Human Era as per the functions in this crate.
///
/// ## Further reading
/// + [Wikipedia](https://en.wikipedia.org/wiki/Holocene_calendar)
/// + [Kurzgesagt](https://www.youtube.com/watch?v=czgOWmtGVGs)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Holocene(CommonDate);

impl AllowYearZero for Holocene {}

impl ToFromOrdinalDate for Holocene {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        Gregorian::valid_ordinal(ord)
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        let g_ord = Gregorian::ordinal_from_fixed(fixed_date);
        OrdinalDate {
            year: (g_ord.year - HOLOCENE_YEAR_OFFSET),
            day_of_year: g_ord.day_of_year,
        }
    }

    fn to_ordinal(self) -> OrdinalDate {
        let g = Gregorian::try_from_common_date(self.to_common_date())
            .expect("Same month/day validity");
        g.to_ordinal()
    }

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        let e = Gregorian::from_ordinal_unchecked(ord);
        Holocene::try_from_common_date(e.to_common_date()).expect("Same month/day validity")
    }
}

impl HasLeapYears for Holocene {
    fn is_leap(h_year: i32) -> bool {
        Gregorian::is_leap(h_year) //10000 is divisible by 400, so it's ok
    }
}

impl CalculatedBounds for Holocene {}

impl Epoch for Holocene {
    fn epoch() -> Fixed {
        Gregorian::try_year_start(HOLOCENE_YEAR_OFFSET + 1)
            .expect("Year known to be valid")
            .to_fixed()
    }
}

impl FromFixed for Holocene {
    fn from_fixed(date: Fixed) -> Holocene {
        let result = Gregorian::from_fixed(date).to_common_date();
        Holocene(CommonDate::new(
            result.year - (HOLOCENE_YEAR_OFFSET as i32),
            result.month,
            result.day,
        ))
    }
}

impl ToFixed for Holocene {
    fn to_fixed(self) -> Fixed {
        let g = Gregorian::try_from_common_date(CommonDate::new(
            self.0.year + (HOLOCENE_YEAR_OFFSET as i32),
            self.0.month,
            self.0.day,
        ))
        .expect("Same month/day rules");
        g.to_fixed()
    }
}

impl ToFromCommonDate<HoloceneMonth> for Holocene {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_ymd(date).is_ok());
        Self(date)
    }

    fn valid_ymd(date: CommonDate) -> Result<(), CalendarError> {
        Gregorian::valid_ymd(date)
    }

    fn year_end_date(year: i32) -> CommonDate {
        Gregorian::year_end_date(year)
    }

    fn month_length(year: i32, month: HoloceneMonth) -> u8 {
        Gregorian::month_length(year, month)
    }
}

impl Quarter for Holocene {
    fn quarter(self) -> NonZero<u8> {
        NonZero::new(((self.to_common_date().month - 1) / 3) + 1).expect("(m-1)/3 > -1")
    }
}

impl GuaranteedMonth<HoloceneMonth> for Holocene {}
impl CommonWeekOfYear<HoloceneMonth> for Holocene {}

/// Represents a date *and time* in the Holocen Calendar
pub type HoloceneMoment = CalendarMoment<Holocene>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h_epoch() {
        let dh = CommonDate {
            year: 1,
            month: 1,
            day: 1,
        };
        let dg = CommonDate {
            year: -9999,
            month: 1,
            day: 1,
        };
        let fh = Holocene::try_from_common_date(dh).unwrap().to_fixed();
        let fg = Gregorian::try_from_common_date(dg).unwrap().to_fixed();
        assert_eq!(fh, fg);
        assert_eq!(fh, Holocene::epoch());
    }

    #[test]
    fn g_epoch() {
        let dh = CommonDate {
            year: 10001,
            month: 1,
            day: 1,
        };
        let dg = CommonDate {
            year: 1,
            month: 1,
            day: 1,
        };
        let fh = Holocene::try_from_common_date(dh).unwrap().to_fixed();
        let fg = Gregorian::try_from_common_date(dg).unwrap().to_fixed();
        assert_eq!(fh, fg);
        assert_eq!(fh, Gregorian::epoch());
    }

    #[test]
    fn date_of_proposal() {
        let dh = CommonDate {
            year: 11993,
            month: 12,
            day: 30,
        };
        let dg = CommonDate {
            year: 1993,
            month: 12,
            day: 30,
        };
        let fh = Holocene::try_from_common_date(dh).unwrap().to_fixed();
        let fg = Gregorian::try_from_common_date(dg).unwrap().to_fixed();
        assert_eq!(fh, fg);
    }
}

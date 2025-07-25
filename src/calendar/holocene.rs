//https://en.wikipedia.org/wiki/Holocene_calendar

use crate::calendar::gregorian::Gregorian;
use crate::calendar::gregorian::GregorianMonth;
use crate::calendar::CommonDate;
use crate::calendar::CommonDay;
use crate::calendar::CommonYear;
use crate::calendar::GuaranteedMonth;
use crate::calendar::HasLeapYears;
use crate::calendar::Quarter;
use crate::calendar::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::calendar::CommonWeekOfYear;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use std::num::NonZero;

const HOLOCENE_YEAR_OFFSET: i16 = -10000;

/// Represents a month in the Holocene calendar
pub type HoloceneMonth = GregorianMonth;

/// Represents a date in the Holocene calendar
///
/// The Holocene calendar was proposed by Cesare Emiliani.
/// It is identical to the proleptic Gregorian calendar, but
/// with an extra 10000 years added to each date. Thus 2016
/// in the Gregorian calendar is 12016 in the Holocene calendar.
///
/// Further reading:
/// + [Wikipedia](https://en.wikipedia.org/wiki/Holocene_calendar)
/// + [Kurzgesagt](https://www.youtube.com/watch?v=czgOWmtGVGs)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Holocene(CommonDate);

impl HasLeapYears for Holocene {
    fn is_leap(h_year: i32) -> bool {
        Gregorian::is_leap(h_year) //10000 is divisible by 400, so it's ok
    }
}

impl CalculatedBounds for Holocene {}

impl Epoch for Holocene {
    fn epoch() -> Fixed {
        Gregorian::year_start(HOLOCENE_YEAR_OFFSET + 1).to_fixed()
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

impl ToFromCommonDate for Holocene {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        Gregorian::valid_month_day(date)
    }

    fn year_end_date(year: i32) -> CommonDate {
        Gregorian::year_end_date(year)
    }
}

impl Quarter for Holocene {
    fn quarter(self) -> NonZero<u8> {
        NonZero::new(((self.to_common_date().month - 1) / 3) + 1).expect("(m-1)/3 > -1")
    }
}

impl CommonYear for Holocene {}
impl GuaranteedMonth<HoloceneMonth> for Holocene {}
impl CommonDay for Holocene {}
impl CommonWeekOfYear for Holocene {}

#[cfg(test)]
mod tests {
    use super::*;

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

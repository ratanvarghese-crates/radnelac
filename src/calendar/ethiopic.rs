use crate::calendar::coptic::Coptic;
use crate::calendar::julian::Julian;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::CommonWeekOfYear;
use crate::calendar::prelude::GuaranteedMonth;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;
use std::num::NonZero;

const ETHIOPIC_EPOCH_JULIAN: CommonDate = CommonDate {
    year: 8,
    month: 8,
    day: 29,
};

/// Represents a month in the Ethiopic Calendar
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum EthiopicMonth {
    Maskaram = 1,
    Teqemt,
    Hedar,
    Takhsas,
    Ter,
    Yakatit,
    Magabit,
    Miyazya,
    Genbot,
    Sane,
    Hamle,
    Nahase,
    Paguemen,
}

impl EthiopicMonth {
    pub fn length(self, leap: bool) -> u8 {
        match self {
            EthiopicMonth::Paguemen => {
                if leap {
                    6
                } else {
                    5
                }
            }
            _ => 30,
        }
    }
}

/// Represents a date in the Ethiopic calendar
///
/// ## Year 0
///
/// Year 0 is supported for this calendar.
///
/// ## Further reading
/// + [Wikipedia](https://en.wikipedia.org/wiki/Ethiopic_calendar)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Ethiopic(CommonDate);

impl HasLeapYears for Ethiopic {
    fn is_leap(year: i32) -> bool {
        year.modulus(4) == 3
    }
}

impl CalculatedBounds for Ethiopic {}

impl Epoch for Ethiopic {
    fn epoch() -> Fixed {
        Julian::try_from_common_date(ETHIOPIC_EPOCH_JULIAN)
            .expect("Epoch known to be in range.")
            .to_fixed()
    }
}

impl FromFixed for Ethiopic {
    fn from_fixed(date: Fixed) -> Ethiopic {
        let f = Fixed::new(date.get() + Coptic::epoch().get() - Ethiopic::epoch().get());
        Ethiopic::try_from_common_date(Coptic::from_fixed(f).to_common_date())
            .expect("Same month/day validity")
    }
}

impl ToFixed for Ethiopic {
    fn to_fixed(self) -> Fixed {
        let e =
            Coptic::try_from_common_date(self.to_common_date()).expect("Same month/day validity");
        Fixed::new(Ethiopic::epoch().get() + e.to_fixed().get() - Coptic::epoch().get())
    }
}

impl ToFromCommonDate<EthiopicMonth> for Ethiopic {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = EthiopicMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > month_opt.unwrap().length(Ethiopic::is_leap(date.year)) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        Coptic::year_end_date(year)
    }
}

impl Quarter for Ethiopic {
    fn quarter(self) -> NonZero<u8> {
        if self.month() == EthiopicMonth::Paguemen {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new((((self.month() as u8) - 1) / 3) + 1).expect("(m-1)/3 > -1")
        }
    }
}

impl GuaranteedMonth<EthiopicMonth> for Ethiopic {}
impl CommonWeekOfYear<EthiopicMonth> for Ethiopic {}

use crate::calendar::egyptian::Egyptian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::CommonDay;
use crate::common::date::CommonYear;
use crate::common::date::Quarter;
use crate::common::date::ToFromCommonDate;
use crate::common::date::TryMonth;
use crate::common::error::CalendarError;
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
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

//https://en.wikipedia.org/wiki/Armenian_calendar
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Armenian(CommonDate);

impl Armenian {
    pub fn day_name(self) -> Option<ArmenianDaysOfMonth> {
        if self.0.year == 13 {
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
}

impl Quarter for Armenian {
    fn quarter(self) -> NonZero<u8> {
        let m = self.to_common_date().month as u8;
        if m == 13 {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new(((m - 1) / 3) + 1).expect("(m - 1) / 3 > -1")
        }
    }
}

impl CommonYear for Armenian {}
impl TryMonth<ArmenianMonth> for Armenian {}
impl CommonDay for Armenian {}

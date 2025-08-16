// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::julian::Julian;
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

//TODO: Coptic weekdays

//LISTING 4.1 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
const COPTIC_EPOCH_JULIAN: CommonDate = CommonDate {
    year: 284,
    month: 8,
    day: 29,
};

/// Represents a month in the Coptic Calendar
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum CopticMonth {
    //LISTING ?? SECTION 4.1 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
    Thoout = 1,
    Paope,
    Athor,
    Koiak,
    Tobe,
    Meshir,
    Paremotep,
    Parmoute,
    Pashons,
    Paone,
    Epep,
    Mesore,
    Epagomene,
}

/// Represents a date in the Coptic calendar
///
/// ## Introduction
///
/// The Coptic calendar (also called the Alexandrian calendar or the Calendar of Martyrs)
/// is used by the Coptic Orthodox Church and the Coptic Catholic Church. Historically it
/// was also used for fiscal purposes in Egypt.
///
/// ## Basic Structure
///
/// Years are divided into 13 months. The first 12 months have 30 days each. The final month
/// has 5 days in a common year and 6 days in a leap year.
///
/// There is 1 leap year every four years. This leap year occurs *before* the Julian leap year.
/// In other words, if a given year is divisible by 4, the year *before* was a leap year.
///
/// ## Epoch
///
/// Years are numbered from the start of the reign of the Roman Emperor Diocletian.
///
/// ## Representation and Examples
///
/// The months are represented in this crate as [`CopticMonth`].
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_1_1 = CommonDate::new(1741, 1, 1);
/// let a_1_1 = Coptic::try_from_common_date(c_1_1).unwrap();
/// assert_eq!(a_1_1.month(), CopticMonth::Thoout);
/// let c_12_30 = CommonDate::new(1741, 12, 30);
/// let a_12_30 = Coptic::try_from_common_date(c_12_30).unwrap();
/// assert_eq!(a_12_30.month(), CopticMonth::Mesore);
/// ```
///
/// ## Further reading
/// + Wikipedia
///   + [Coptic Calendar](https://en.wikipedia.org/wiki/Coptic_calendar)
///   + [Era of the Martyrs](https://en.wikipedia.org/wiki/Era_of_the_Martyrs)
/// + [Coptic Orthodox Church](https://copticorthodox.church/en/coptic-church/coptic-history/)
/// + [*The Coptic Christian Heritage* by Lois M. Farag](https://www.google.ca/books/edition/The_Coptic_Christian_Heritage/dYK3AQAAQBAJ)
/// + [*A Handbook for Travellers in Lower and_Upper Egypt*](https://www.google.ca/books/edition/A_Handbook_for_Travellers_in_Lower_and_U/CnhJYhBzMmgC?hl=en&gbpv=1)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coptic(CommonDate);

impl AllowYearZero for Coptic {}

impl ToFromOrdinalDate for Coptic {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        let correction = if Coptic::is_leap(ord.year) { 1 } else { 0 };
        if ord.day_of_year > 0 && ord.day_of_year <= (365 + correction) {
            Ok(())
        } else {
            Err(CalendarError::InvalidDayOfYear)
        }
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        //LISTING 4.4 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Missing the day term and parts of the month term
        let date = fixed_date.get_day_i();
        let epoch = Coptic::epoch().get_day_i();
        let year = (4 * (date - epoch) + 1463).div_euclid(1461) as i32;
        let year_start = Coptic::to_fixed(Coptic(CommonDate::new(year, 1, 1)));
        let doy = ((date - year_start.get_day_i()) + 1) as u16;
        OrdinalDate {
            year: year,
            day_of_year: doy,
        }
    }

    fn to_ordinal(self) -> OrdinalDate {
        //LISTING 4.3 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //This is just the terms containing monthand day
        OrdinalDate {
            year: self.0.year,
            day_of_year: (30 * ((self.0.month as u16) - 1) + (self.0.day as u16)),
        }
    }

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        //LISTING 4.4 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Only the month and day terms, modified to use ordinal day count instead of count from epoch
        let month = ((ord.day_of_year - 1).div_euclid(30) + 1) as u8;
        let month_start = Coptic(CommonDate::new(ord.year, month, 1)).to_ordinal();
        let day = (ord.day_of_year - month_start.day_of_year + 1) as u8;
        Coptic(CommonDate::new(ord.year, month, day))
    }
}

impl HasLeapYears for Coptic {
    //LISTING 4.2 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
    fn is_leap(year: i32) -> bool {
        year.modulus(4) == 3
    }
}

impl CalculatedBounds for Coptic {}

impl Epoch for Coptic {
    fn epoch() -> Fixed {
        Julian::try_from_common_date(COPTIC_EPOCH_JULIAN)
            .expect("Epoch known to be in range.")
            .to_fixed()
    }
}

impl FromFixed for Coptic {
    fn from_fixed(fixed_date: Fixed) -> Coptic {
        //LISTING 4.4 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Split compared to original
        let ord = Self::ordinal_from_fixed(fixed_date);
        Self::from_ordinal_unchecked(ord)
    }
}

impl ToFixed for Coptic {
    fn to_fixed(self) -> Fixed {
        //LISTING 4.3 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Split compared to original: the terms containing month and day are in to_ordinal
        let year = self.0.year as i64;
        let epoch = Coptic::epoch().get_day_i();
        let doy = self.to_ordinal().day_of_year as i64;
        Fixed::cast_new(epoch - 1 + (365 * (year - 1)) + year.div_euclid(4) + doy)
    }
}

impl ToFromCommonDate<CopticMonth> for Coptic {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_ymd(date).is_ok());
        Self(date)
    }

    fn valid_ymd(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = CopticMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > Self::month_length(date.year, month_opt.unwrap()) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        let m = CopticMonth::Epagomene;
        CommonDate::new(year, m as u8, Self::month_length(year, m))
    }

    fn month_length(year: i32, month: CopticMonth) -> u8 {
        //LISTING ?? SECTION 4.1 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        match (month, Coptic::is_leap(year)) {
            (CopticMonth::Epagomene, false) => 5,
            (CopticMonth::Epagomene, true) => 6,
            (_, _) => 30,
        }
    }
}

impl Quarter for Coptic {
    fn quarter(self) -> NonZero<u8> {
        if self.month() == CopticMonth::Epagomene {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new((((self.month() as u8) - 1) / 3) + 1).expect("(m-1)/3 > -1")
        }
    }
}

impl GuaranteedMonth<CopticMonth> for Coptic {}
impl CommonWeekOfYear<CopticMonth> for Coptic {}

/// Represents a date *and time* in the Coptic Calendar
pub type CopticMoment = CalendarMoment<Coptic>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::julian::JulianMonth;
    use crate::calendar::Gregorian;
    use proptest::prop_assume;

    use proptest::proptest;

    #[test]
    fn handbook() {
        //https://www.google.ca/books/edition/A_Handbook_for_Travellers_in_Lower_and_U/CnhJYhBzMmgC?hl=en&gbpv=1
        let c = Coptic::try_from_common_date(CommonDate::new(1604, 1, 1)).unwrap();
        let g = c.convert::<Gregorian>();
        assert_eq!(g.to_common_date(), CommonDate::new(1887, 9, 11));
    }

    proptest! {
        #[test]
        fn julian_leap_ad(x in 1..(i16::MAX/4)) {
            let jy: i32 = (x * 4) as i32;
            prop_assume!(Julian::is_leap(jy));
            let j = Julian::try_from_common_date(CommonDate::new(jy, 1, 1)).unwrap();
            let c = j.convert::<Coptic>();
            assert!(!Coptic::is_leap(c.year()));
            assert!(Coptic::is_leap(c.year() - 1));
        }

        #[test]
        fn christmas(y in i16::MIN..i16::MAX) {
            //LISTING 4.9 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
            let c = Coptic::try_from_common_date(CommonDate::new(y as i32, CopticMonth::Koiak as u8, 29))?;
            let j = c.convert::<Julian>();
            assert_eq!(j.month(), JulianMonth::December);
            assert!(j.day() == 25 || j.day() == 26);
        }

        #[test]
        fn feast_of_neyrouz(y in i16::MIN..i16::MAX) {
            // https://en.wikipedia.org/wiki/Coptic_calendar
            let c = Coptic::try_from_common_date(CommonDate::new(y as i32, CopticMonth::Thoout as u8, 1))?;
            let j = c.convert::<Julian>();
            assert_eq!(j.month(), JulianMonth::August);
            assert!(j.day() == 29 || j.day() == 30);
        }
    }
}

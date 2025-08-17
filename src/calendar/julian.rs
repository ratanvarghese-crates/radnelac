// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::gregorian::GregorianMonth;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::CommonWeekOfYear;
use crate::calendar::prelude::GuaranteedMonth;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::CalendarMoment;
use crate::calendar::Gregorian;
use crate::calendar::OrdinalDate;
use crate::calendar::ToFromOrdinalDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::RataDie;
use crate::day_count::ToFixed;
use std::num::NonZero;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

/// Represents a month in the Julian calendar
pub type JulianMonth = GregorianMonth;

//LISTING 3.2 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
//Instead of explicitly converting from Gregorian, just use the known Rata Die value.
const JULIAN_EPOCH_RD: i32 = -1;

/// Represents a date in the proleptic Julian calendar
///
/// ## Introduction
///
/// The Julian calendar is used by the Eastern Orthodox Church. Historically, it was used
/// by the Roman Empire, many later European states, and the colonies of those states.
///
/// The calendar is named after Julius Caesar who decreed that the calendar be used in the
/// Roman Empire, replacing the calendar used in the late Roman Republic. Caesar may have
/// been assisted by Sosigenes of Alexandria, according to Pliny. He may have also been
/// assisted by Marcus Flavius, according to Macrobius.
///
/// Over the past 500 years, the Julian calendar has been almost entirely replaced by the
/// Gregorian calendar.
///
/// ### Proleptic Modification
///
/// During the initial adoption of the Julian calendar in 45 Before Christ (BC), leap years
/// were every 3 years instead of every 4 years. According to Macrobius, this error was
/// introduced by Roman priests and had to be corrected by Augustus in 8 Anno Domini (AD).
/// (See the "Epoch" section for more details about BC, AD and AUC epoch labels).
///
/// According to Wikipedia:
/// > The proleptic Julian calendar is produced by extending the Julian calendar backwards
/// > to dates preceding AD 8 when the quadrennial leap year stabilized.
///
/// This crate implements a proleptic Julian calendar, and so does **not** change the leap year
/// rules for dates before 8 AD.
///
/// ### Year 0
///
/// Year 0 is **not** supported for this implementation of the Julian calendar.
/// The year before 1 is -1.
///
/// ## Basic Structure
///
/// Years are divided into 12 months. Every month has either 30 or 31 days except for the
/// second month, February. February has 28 days in a common year and 29 days in a leap year.
///
/// Leap years occur on every positive year divisible by 4, and every negative year before
/// a year divisible by 4.
///
/// (See [`Roman`](crate::calendar::Roman) for Roman names of days).
///
/// ## Epoch
///
/// Years are numbered based on an estimate of the date of birth of Jesus Christ. The estimate
/// was devised by Dionysius Exiguus 525 years after the birth supposedly happened.
///
/// The first year of the Julian calendar is called 1 Anno Domini (abbreviated "AD"), and the
/// year before that is called 1 Before Christ (abbreviated "BC").
///
/// ### Alternative Epochs
///
/// Before 525 AD (and for centuries after 525 AD) there were other epochs used with the Julian
/// calendar. One such epoch is "Ab urbe condita" (abbreviated "AUC"), based on the date of the
/// founding of Rome - see [`Roman`](crate::calendar::Roman) for more details.
///
/// Another method of identifying years was to name the consuls who held office that year. Regnal
/// years were also used in Roman Egypt and the Byzantine Empire.
///
/// ## Representation and Examples
///
/// The months are represented in this crate as [`JulianMonth`].
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_1_1 = CommonDate::new(2025, 1, 1);
/// let a_1_1 = Julian::try_from_common_date(c_1_1).unwrap();
/// assert_eq!(a_1_1.month(), JulianMonth::January);
/// ```
///
/// ### Conversion to Gregorian
///
/// For historical dates, it is often necessary to convert to the Gregorian system.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let j = Julian::try_new(1752, JulianMonth::September, 3).unwrap();
/// let g = j.convert::<Gregorian>();
/// assert_eq!(g, Gregorian::try_new(1752, GregorianMonth::September, 14).unwrap());
/// ```
///
/// ## Inconsistencies with Other Implementations
///
/// Other systems may use non-proleptic Julian calendars. They might also allow year 0 for the
/// Julian calendar.
///
/// ## Further reading
/// + Wikipedia
///   + [Julian calendar](https://en.wikipedia.org/wiki/Julian_calendar)
///   + [Proleptic Julian calendar](https://en.m.wikipedia.org/wiki/Proleptic_Julian_calendar)
///   + [Ab urbe condita](https://en.m.wikipedia.org/wiki/Ab_urbe_condita)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Julian(CommonDate);

impl Julian {
    pub fn nz_year(self) -> NonZero<i32> {
        NonZero::new(self.0.year).expect("Will not be assigned zero")
    }

    pub fn prior_elapsed_days(year: i32) -> i64 {
        //LISTING 3.3 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //These are the terms which do not rely on the day or month
        let y = if year < 0 { year + 1 } else { year } as i64;
        let offset_e = Julian::epoch().get_day_i() - 1;
        let offset_y = 365 * (y - 1);
        let offset_leap = (y - 1).div_euclid(4);
        offset_e + offset_y + offset_leap
    }
}

impl ToFromOrdinalDate for Julian {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        let correction = if Julian::is_leap(ord.year) { 1 } else { 0 };
        if ord.day_of_year > 0 && ord.day_of_year <= (365 + correction) {
            Ok(())
        } else {
            Err(CalendarError::InvalidDayOfYear)
        }
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        //LISTING 3.4 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //These are the calculations except for correction, month and day
        let date = fixed_date.get_day_i();
        let epoch = Julian::epoch().get_day_i();
        let approx = ((4 * (date - epoch)) + 1464).div_euclid(1461);
        let year = if approx <= 0 { approx - 1 } else { approx } as i32;
        let year_start = Julian(CommonDate::new(year, 1, 1)).to_fixed().get_day_i();
        let prior_days = (date - year_start) as u16;
        OrdinalDate {
            year: year,
            day_of_year: prior_days + 1,
        }
    }

    fn to_ordinal(self) -> OrdinalDate {
        //LISTING 3.3 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //These are the terms which rely on the day or month
        let year = self.0.year;
        let month = self.0.month as i64;
        let day = self.0.day as i64;
        let offset_m = ((367 * month) - 362).div_euclid(12);
        let offset_x = if month <= 2 {
            0
        } else if Julian::is_leap(year) {
            -1
        } else {
            -2
        };
        let offset_d = day;

        OrdinalDate {
            year: year,
            day_of_year: (offset_m + offset_x + offset_d) as u16,
        }
    }

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        //LISTING 3.4 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //These are the calculations for correction, month and day
        let year = ord.year;
        let prior_days = ord.day_of_year - 1;
        let march1 = Julian(CommonDate::new(year, 3, 1)).to_ordinal(); //Modification
        let correction = if ord < march1 {
            0
        } else if Julian::is_leap(year) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373).div_euclid(367) as u8;
        let month_start = Julian(CommonDate::new(year, month, 1)).to_ordinal();
        let day = ((ord.day_of_year - month_start.day_of_year) as u8) + 1;
        debug_assert!(day > 0);
        Julian(CommonDate { year, month, day })
    }
}

impl HasLeapYears for Julian {
    fn is_leap(j_year: i32) -> bool {
        //LISTING 3.1 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let m4 = j_year.modulus(4);
        if j_year > 0 {
            m4 == 0
        } else {
            m4 == 3
        }
    }
}

impl CalculatedBounds for Julian {}

impl Epoch for Julian {
    fn epoch() -> Fixed {
        RataDie::new(JULIAN_EPOCH_RD as f64).to_fixed()
    }
}

impl FromFixed for Julian {
    fn from_fixed(fixed_date: Fixed) -> Julian {
        //LISTING 3.4 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Split compared to original
        let ord = Self::ordinal_from_fixed(fixed_date);
        Self::from_ordinal_unchecked(ord)
    }
}

impl ToFixed for Julian {
    fn to_fixed(self) -> Fixed {
        //LISTING 3.3 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Split compared to original
        let offset_prior = Julian::prior_elapsed_days(self.0.year);
        let ord = self.to_ordinal();
        Fixed::cast_new(offset_prior + (ord.day_of_year as i64))
    }
}

impl ToFromCommonDate<JulianMonth> for Julian {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_ymd(date).is_ok());
        Self(date)
    }

    fn valid_ymd(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = JulianMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > Julian::month_length(date.year, month_opt.unwrap()) {
            Err(CalendarError::InvalidDay)
        } else if date.year == 0 {
            Err(CalendarError::InvalidYear)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        let m = JulianMonth::December;
        CommonDate::new(year, m as u8, Julian::month_length(year, m))
    }

    fn month_length(year: i32, month: JulianMonth) -> u8 {
        //LISTING ?? SECTION 2.1 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //TODO: use listing 2.1 here?
        match month {
            JulianMonth::February => {
                if Julian::is_leap(year) {
                    29
                } else {
                    28
                }
            }
            _ => Gregorian::month_length(year, month),
        }
    }
}

impl Quarter for Julian {
    fn quarter(self) -> NonZero<u8> {
        NonZero::new(((self.to_common_date().month - 1) / 3) + 1).expect("(m-1)/3 > -1")
    }
}

impl GuaranteedMonth<JulianMonth> for Julian {}
impl CommonWeekOfYear<JulianMonth> for Julian {}

/// Represents a date *and time* in the Julian Calendar
pub type JulianMoment = CalendarMoment<Julian>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::gregorian::Gregorian;
    use proptest::proptest;

    #[test]
    fn julian_gregorian_conversion() {
        let gap_list = [
            // Official dates of adopting the Gregorian calendar
            // Governments would declare that certain days would be skipped
            // The table below lists Julian dates and the Gregorian dates of the next day.
            // https://en.wikipedia.org/wiki/Adoption_of_the_Gregorian_calendar
            // https://en.wikipedia.org/wiki/List_of_adoption_dates_of_the_Gregorian_calendar_by_country
            (CommonDate::new(1582, 10, 4), CommonDate::new(1582, 10, 15)), //Papal States, Spain, Portugal
            (CommonDate::new(1582, 12, 9), CommonDate::new(1582, 12, 20)), //France
            (CommonDate::new(1582, 12, 14), CommonDate::new(1582, 12, 25)), //"Flanders" (Belgium), Netherlands
            (CommonDate::new(1582, 12, 20), CommonDate::new(1582, 12, 31)), //"Southern Netherlands" (Belgium), Luxembourg
            (CommonDate::new(1582, 12, 31), CommonDate::new(1583, 1, 11)),  //"Aachen" (Germany)
            (CommonDate::new(1583, 1, 1), CommonDate::new(1583, 1, 12)), //"Holland" (Netherlands)
            (CommonDate::new(1583, 2, 10), CommonDate::new(1583, 2, 21)), //"Salzburg" (Austria), "Liege" (Belgium)
            (CommonDate::new(1583, 2, 13), CommonDate::new(1583, 2, 24)), //"Kaufbeuren" (Germany)
            (CommonDate::new(1583, 2, 14), CommonDate::new(1583, 2, 25)), //"Ellwangen" (Germany)
            (CommonDate::new(1583, 3, 1), CommonDate::new(1583, 3, 12)), //"Groningen" (Netherlands)
            (CommonDate::new(1583, 10, 4), CommonDate::new(1583, 10, 15)), //"Tyrol" (Austria)
            (CommonDate::new(1583, 10, 5), CommonDate::new(1583, 10, 16)), //"Bavaria" (Germany)
            (CommonDate::new(1583, 10, 13), CommonDate::new(1583, 10, 24)), //"Austrian Upper Alsace" (France)
            (CommonDate::new(1583, 10, 20), CommonDate::new(1583, 10, 31)), //"Lower Austria" (Austria)
            (CommonDate::new(1583, 11, 2), CommonDate::new(1583, 11, 13)),  //"Cologne" (Germany)
            (CommonDate::new(1583, 11, 11), CommonDate::new(1583, 11, 22)), //"Mainz" (Germany)
            (CommonDate::new(1632, 12, 14), CommonDate::new(1632, 12, 25)), //"Hildesheim" (Germany)
            (CommonDate::new(1700, 2, 18), CommonDate::new(1700, 3, 1)), //"Denmark-Norway" (Denmark, Norway)
            (CommonDate::new(1753, 2, 17), CommonDate::new(1753, 3, 1)), //Sweden (partial?)
            (CommonDate::new(1752, 9, 2), CommonDate::new(1752, 9, 14)), //British Empire (United Kingdom, Ireland, Canada, United States)
            (CommonDate::new(1753, 2, 17), CommonDate::new(1753, 3, 1)), //Sweden
            (CommonDate::new(1912, 11, 14), CommonDate::new(1912, 11, 28)), //Albania
            (CommonDate::new(1916, 3, 31), CommonDate::new(1916, 4, 14)), //Bulgaria
            (CommonDate::new(1918, 1, 31), CommonDate::new(1918, 2, 14)), //Soviet Union (Russia, etc.)
            (CommonDate::new(1918, 2, 15), CommonDate::new(1918, 3, 1)),  //Estonia, Ukraine
            (CommonDate::new(1918, 4, 17), CommonDate::new(1918, 5, 1)), //"Transcaucasian Democratic Federative Republic"
            (CommonDate::new(1919, 1, 14), CommonDate::new(1919, 1, 28)), //Yugoslavia
            (CommonDate::new(1919, 3, 31), CommonDate::new(1919, 4, 14)), //Romania
            (CommonDate::new(1923, 2, 15), CommonDate::new(1923, 3, 1)), //Greece
        ];

        for pair in gap_list {
            let dj = Julian::try_from_common_date(pair.0).unwrap().to_fixed();
            let dg = Gregorian::try_from_common_date(pair.1).unwrap().to_fixed();
            assert_eq!(dj.get_day_i() + 1, dg.get_day_i());
        }
    }

    #[test]
    fn cross_epoch() {
        let new_years_eve = Julian::try_year_end(-1).unwrap().to_fixed();
        let new_years_day = Julian::try_year_start(1).unwrap().to_fixed();
        assert_eq!(new_years_day.get_day_i(), new_years_eve.get_day_i() + 1);
        assert!(Julian::try_year_start(0).is_err());
        assert!(Julian::try_year_end(0).is_err());
    }

    proptest! {
        #[test]
        fn invalid_year_0(month in 1..12, day in 1..28) {
            let c = CommonDate::new(0, month as u8, day as u8);
            assert!(Julian::try_from_common_date(c).is_err())
        }
    }
}

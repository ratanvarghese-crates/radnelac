use crate::calendar::gregorian::Gregorian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::CommonDay;
use crate::common::date::CommonYear;
use crate::common::date::HasLeapYears;
use crate::common::date::PerennialWithComplementaryDay;
use crate::common::date::Quarter;
use crate::common::date::ToFromCommonDate;
use crate::common::date::TryMonth;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::date::ComplementaryWeekOfYear;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;
use std::num::NonZero;

const FRENCH_EPOCH_GREGORIAN: CommonDate = CommonDate {
    year: 1792,
    month: 9,
    day: 22,
};
const NON_MONTH: u8 = 13;

/// Represents a month in the French Revolutionary Calendar
///
/// Note that the Sansculottides at the end of the French Revolutionary calendar
/// year have no month and thus are not represented by FrenchRevMonth. When representing
/// an arbitrary day in the French Revolutionary calendar, use an `Option<FrenchRevMonth>`
/// for the the month field.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum FrenchRevMonth {
    Vendemiaire = 1,
    Brumaire,
    Frimaire,
    Nivose,
    Pluviose,
    Ventose,
    Germinal,
    Floreal,
    Prairial,
    Messidor,
    Thermidor,
    Fructidor,
}

/// Represents a weekday in the French Revolutionary Calendar
///
/// The calendar reforms during the French Revolution included the creation of
/// a ten-day week. The name of each day is based on the numeric position in the week.
///
/// Note that the Sansculottides at the end of the French Revolutionary calendar
/// year do not have a weekday.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum FrenchRevWeekday {
    Primidi = 1,
    Duodi,
    Tridi,
    Quartidi,
    Quintidi,
    Sextidi,
    Septidi,
    Octidi,
    Nonidi,
    Decadi,
}

/// Represents an epagomenal day at the end of the French Revolutionary calendar year
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Sansculottide {
    Vertu = 1,
    Genie,
    Travail,
    Opinion,
    Recompense,
    Revolution,
}

/// Represents a date in the algorithmic approximation of the French Revolutionary calendar
/// (ie French Republican calendar)
///
/// The calendar actually implemented during the French First Republic relied on astronomical
/// observations to determine whether a given year was a leap year. **FrenchRevArith does not
/// read astronomical data nor approximate such data - instead it relies on algorithmic
/// rules to determine the start of new years, similar to those used by the Gregorian calendar**.
///
/// The leap year rule is determined by the parameter L.
/// * L = false: According to this rule, any year which is a multiple of 4 is a leap year
///    unless it is a multiple of 100. Any year which is a multiple of 100 is not a leap year
///    unless it is a multiple of 400. Any year which is a multiple of 400 is a leap year
///    unless it is a multiple of 4000. For example, years 4, 8, and 12 are leap years.
/// * L = true: This approximation is exactly the same as the one used where L = false, except
///    that an offset of 1 is added to the year before starting the calculation. For example,
///    years 3, 7 and 11 are leap years.
///
/// The approximation where L = false was proposed by Gilbert Romme, who directed the creation
/// of the calendar. It is commonly used by other software approximating the French Revolutionary
/// calendar. However, it was never used by any French government -
/// the calendar actually used during the French First Republic used astronomical observations
/// to determine leap years, and contradicted Romme's approximations. The official leap years
/// during the Revolution were years 3, 7, and 11 whereas the leap years produced by Romme's
/// approximation are years 4, 8, and 12.
///
/// The approximation where L = true ensures that leap years are consistent with the French
/// government for the years where the Revolutionary calendar was officially used. This is a
/// rather crude approximation which is not astronomically accurate outside those particular
/// years.
///
/// The value of L should be determined by the caller's use case:
/// * for consistency with other software using Romme's approximation: L = false
/// * for consistency with Romme's wishes: L = false
/// * for consistency with historical dates during the French First Republic: L = true
/// * for consistency with historical dates during the Paris Commune: L = true
/// * for consistency with how the calendar was "originally intended" to work for
///   time periods not mentioned above: **not supported**
///
/// The final use case in the list above is not currently supported by this library.
/// Implementing that feature requires calculating the date of the autumnal equinox
/// at the Paris Observatory. If a future version of this library implements such
/// astronomical calculations, those calculations will not be provided by FrenchRevArith.
/// Instead, such calculations shall be provided by a new struct with a new name.
///
/// Further reading
/// + [Wikipedia](https://en.wikipedia.org/wiki/French_Republican_calendar)
/// + [Guanzhong "quantum" Chen](https://quantum5.ca/2022/03/09/art-of-time-keeping-part-4-french-republican-calendar/)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FrenchRevArith<const L: bool>(CommonDate);

impl<const L: bool> FrenchRevArith<L> {
    /// Returns L
    pub fn is_adjusted(self) -> bool {
        L
    }
}

impl<const L: bool> PerennialWithComplementaryDay<Sansculottide, FrenchRevWeekday>
    for FrenchRevArith<L>
{
    fn complementary(self) -> Option<Sansculottide> {
        if self.0.month == NON_MONTH {
            Sansculottide::from_u8(self.0.day)
        } else {
            None
        }
    }

    fn weekday(self) -> Option<FrenchRevWeekday> {
        if self.0.month == NON_MONTH {
            None
        } else {
            FrenchRevWeekday::from_i64((self.0.day as i64).adjusted_remainder(10))
        }
    }

    fn complementary_count(f_year: i32) -> u8 {
        if FrenchRevArith::<L>::is_leap(f_year) {
            6
        } else {
            5
        }
    }

    fn days_per_week() -> u8 {
        10
    }

    fn weeks_per_month() -> u8 {
        3
    }
}

impl<const L: bool> HasLeapYears for FrenchRevArith<L> {
    fn is_leap(year: i32) -> bool {
        let f_year = if L { year + 1 } else { year };
        let m4 = f_year.modulus(4);
        let m400 = f_year.modulus(400);
        let m4000 = f_year.modulus(4000);
        m4 == 0 && (m400 != 100 && m400 != 200 && m400 != 300) && m4000 != 0
    }
}

impl<const L: bool> CalculatedBounds for FrenchRevArith<L> {}

impl<const L: bool> Epoch for FrenchRevArith<L> {
    fn epoch() -> Fixed {
        Gregorian::try_from_common_date(FRENCH_EPOCH_GREGORIAN)
            .expect("Epoch known to be valid")
            .to_fixed()
    }
}

impl<const L: bool> FromFixed for FrenchRevArith<L> {
    fn from_fixed(fixed_date: Fixed) -> FrenchRevArith<L> {
        let date = fixed_date.get_day_i();
        let epoch = Self::epoch().get_day_i();
        let approx = ((4000 * (date - epoch + 2)).div_euclid(1460969) + 1) as i32;
        let approx_start = Self(CommonDate::new(approx, 1, 1)).to_fixed().get_day_i();
        let year = if date < approx_start {
            approx - 1
        } else {
            approx
        };
        let year_start = Self(CommonDate::new(year, 1, 1)).to_fixed().get_day_i();
        let month = (1 + (date - year_start).div_euclid(30)) as u8;
        let month_start = Self(CommonDate::new(year, month, 1)).to_fixed().get_day_i();
        let day = (1 + date - month_start) as u8;

        FrenchRevArith(CommonDate::new(year, month, day))
    }
}

impl<const L: bool> ToFixed for FrenchRevArith<L> {
    fn to_fixed(self) -> Fixed {
        let year = self.0.year as i64;
        let month = self.0.month as i64;
        let day = self.0.day as i64;
        let y_adj = if L { 1 } else { 0 };

        let offset_e = Self::epoch().get_day_i() - 1;
        let offset_y = 365 * (year - 1);
        let offset_leap = (year + y_adj - 1).div_euclid(4) - (year + y_adj - 1).div_euclid(100)
            + (year + y_adj - 1).div_euclid(400)
            - (year + y_adj - 1).div_euclid(4000);
        let offset_m = 30 * (month - 1);
        let offset_d = day;
        Fixed::cast_new(offset_e + offset_y + offset_leap + offset_m + offset_d)
    }
}

impl<const L: bool> ToFromCommonDate for FrenchRevArith<L> {
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
        } else if date.month == NON_MONTH
            && date.day > FrenchRevArith::<L>::complementary_count(date.year)
        {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        CommonDate::new(
            year,
            NON_MONTH,
            FrenchRevArith::<L>::complementary_count(year),
        )
    }
}

impl<const L: bool> Quarter for FrenchRevArith<L> {
    fn quarter(self) -> NonZero<u8> {
        let m = self.to_common_date().month;
        if m == NON_MONTH {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new(((m - 1) / 3) + 1).expect("(m-1)/3 > -1")
        }
    }
}

impl<const L: bool> CommonYear for FrenchRevArith<L> {}
impl<const L: bool> TryMonth<FrenchRevMonth> for FrenchRevArith<L> {}
impl<const L: bool> CommonDay for FrenchRevArith<L> {}

impl<const L: bool> ComplementaryWeekOfYear<FrenchRevMonth, Sansculottide, FrenchRevWeekday>
    for FrenchRevArith<L>
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;

    #[test]
    fn leaps() {
        assert!(FrenchRevArith::<true>::is_leap(3));
        assert!(FrenchRevArith::<true>::is_leap(7));
        assert!(FrenchRevArith::<true>::is_leap(11));
        assert!(FrenchRevArith::<false>::is_leap(4));
        assert!(FrenchRevArith::<false>::is_leap(8));
        assert!(FrenchRevArith::<false>::is_leap(12));
    }

    #[test]
    fn revolutionary_events() {
        // https://en.wikipedia.org/wiki/Glossary_of_the_French_Revolution#Events_commonly_known_by_their_Revolutionary_dates
        // 13 Vendémiaire and 18 Brumaire can be mangled when L = false
        let event_list = [
            (
                CommonDate::new(2, FrenchRevMonth::Prairial as u8, 22),
                CommonDate::new(2, FrenchRevMonth::Prairial as u8, 22),
                CommonDate::new(1794, 6, 10),
            ),
            (
                CommonDate::new(2, FrenchRevMonth::Thermidor as u8, 9),
                CommonDate::new(2, FrenchRevMonth::Thermidor as u8, 9),
                CommonDate::new(1794, 7, 27),
            ),
            (
                CommonDate::new(4, FrenchRevMonth::Vendemiaire as u8, 13),
                CommonDate::new(4, FrenchRevMonth::Vendemiaire as u8, 13 + 1), //Supposed to be 13
                CommonDate::new(1795, 10, 5),
            ),
            (
                CommonDate::new(5, FrenchRevMonth::Fructidor as u8, 18),
                CommonDate::new(5, FrenchRevMonth::Fructidor as u8, 18),
                CommonDate::new(1797, 9, 4),
            ),
            (
                CommonDate::new(6, FrenchRevMonth::Floreal as u8, 22),
                CommonDate::new(6, FrenchRevMonth::Floreal as u8, 22),
                CommonDate::new(1798, 5, 11),
            ),
            (
                CommonDate::new(7, FrenchRevMonth::Prairial as u8, 30),
                CommonDate::new(7, FrenchRevMonth::Prairial as u8, 30),
                CommonDate::new(1799, 6, 18),
            ),
            (
                CommonDate::new(8, FrenchRevMonth::Brumaire as u8, 18),
                CommonDate::new(8, FrenchRevMonth::Brumaire as u8, 18 + 1), //Supposed to be 18
                CommonDate::new(1799, 11, 9),
            ),
            // Paris Commune
            (
                CommonDate::new(79, FrenchRevMonth::Floreal as u8, 16),
                CommonDate::new(79, FrenchRevMonth::Floreal as u8, 16),
                CommonDate::new(1871, 5, 6),
            ),
        ];
        for pair in event_list {
            let df0 = FrenchRevArith::<true>::try_from_common_date(pair.0)
                .unwrap()
                .to_fixed();
            let df1 = FrenchRevArith::<false>::try_from_common_date(pair.1)
                .unwrap()
                .to_fixed();
            let dg = Gregorian::try_from_common_date(pair.2).unwrap().to_fixed();
            assert_eq!(df0, dg);
            assert_eq!(df1, dg);
        }
    }

    proptest! {
        #[test]
        fn align_to_gregorian(year in 0..100) {
            // https://en.wikipedia.org/wiki/French_Republican_calendar
            // > Autumn:
            // >     Vendémiaire (...), starting 22, 23, or 24 September
            // >     Brumaire (...), starting 22, 23, or 24 October
            // >     Frimaire (...), starting 21, 22, or 23 November
            // > Winter:
            // >     Nivôse (...), starting 21, 22, or 23 December
            // >     Pluviôse (...), starting 20, 21, or 22 January
            // >     Ventôse (...), starting 19, 20, or 21 February
            // > Spring:
            // >     Germinal (...), starting 21 or 22 March
            // >     Floréal (...), starting 20 or 21 April
            // >     Prairial (...), starting 20 or 21 May
            // > Summer:
            // >     Messidor (...), starting 19 or 20 June
            // >     Thermidor (...), starting 19 or 20 July; ...
            // >     Fructidor (...), starting 18 or 19 August
            // Not clear how long this property is supposed to hold, given
            // the differing leap year rule. There can be off by one errors
            // if L is false.
            let d_list = [
                ( CommonDate{ year, month: 1, day: 1 }, 9, 22, 24),
                ( CommonDate{ year, month: 2, day: 1 }, 10, 22, 24),
                ( CommonDate{ year, month: 3, day: 1 }, 11, 21, 23),
                ( CommonDate{ year, month: 4, day: 1 }, 12, 21, 23),
                ( CommonDate{ year, month: 5, day: 1 }, 1, 20, 22),
                ( CommonDate{ year, month: 6, day: 1 }, 2, 19, 21),
                ( CommonDate{ year, month: 7, day: 1 }, 3, 21, 22),
                ( CommonDate{ year, month: 8, day: 1 }, 4, 20, 21),
                ( CommonDate{ year, month: 9, day: 1 }, 5, 20, 21),
                ( CommonDate{ year, month: 10, day: 1 }, 6, 19, 20),
                ( CommonDate{ year, month: 11, day: 1 }, 7, 19, 20),
                ( CommonDate{ year, month: 12, day: 1 }, 8, 18, 19),
            ];
            for item in d_list {
                let r0 = FrenchRevArith::<true>::try_from_common_date(item.0).unwrap();
                let f0 = r0.to_fixed();
                let r1 = FrenchRevArith::<false>::try_from_common_date(item.0).unwrap();
                let f1 = r1.to_fixed();
                let g = Gregorian::from_fixed(f0);
                let gc = g.to_common_date();
                assert_eq!(gc.month, item.1);
                assert!(item.2 <= gc.day && item.3 >= gc.day);
                assert!((f1.get_day_i() - f0.get_day_i()).abs() < 2);
            }
        }
    }
}

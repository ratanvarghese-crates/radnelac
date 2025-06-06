use crate::calendar::gregorian::Gregorian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const FRENCH_EPOCH_GREGORIAN: CommonDate = CommonDate {
    year: 1792,
    month: 9,
    day: 22,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum Sansculottide {
    Vertu = 1,
    Genie,
    Travail,
    Opinion,
    Recompense,
    Revolution,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FrenchRevArith<const L: bool>(CommonDate);

impl<const L: bool> FrenchRevArith<L> {
    pub fn is_adjusted(self) -> bool {
        L
    }

    pub fn year(self) -> i32 {
        self.0.year
    }

    pub fn month(self) -> Option<FrenchRevMonth> {
        if self.0.month == 13 {
            None
        } else {
            FrenchRevMonth::from_u8(self.0.month)
        }
    }

    pub fn day(self) -> u8 {
        self.0.day
    }

    pub fn sansculottide(self) -> Option<Sansculottide> {
        if self.0.month == 13 {
            Sansculottide::from_u8(self.0.day)
        } else {
            None
        }
    }

    pub fn weekday(self) -> Option<FrenchRevWeekday> {
        if self.0.month == 13 {
            None
        } else {
            FrenchRevWeekday::from_i64((self.0.day as i64).adjusted_remainder(10))
        }
    }

    pub fn is_leap(year: i32) -> bool {
        let f_year = if L { year + 1 } else { year };
        let m4 = f_year.modulus(4);
        let m400 = f_year.modulus(400);
        let m4000 = f_year.modulus(4000);
        m4 == 0 && (m400 != 100 && m400 != 200 && m400 != 300) && m4000 != 0
    }

    pub fn sansculottide_count(f_year: i32) -> u8 {
        if FrenchRevArith::<L>::is_leap(f_year) {
            6
        } else {
            5
        }
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
        if date.month < 1 || date.month > 13 {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.month < 13 && date.day > 30 {
            Err(CalendarError::InvalidDay)
        } else if date.month == 13 && date.day > FrenchRevArith::<L>::sansculottide_count(date.year)
        {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::RataDie;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
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

        #[test]
        fn sansculottide_xor_weekday(t in FIXED_MIN..FIXED_MAX) {
            let t0 = RataDie::new(t).to_fixed().to_day();
            let r0 = FrenchRevArith::<true>::from_fixed(t0);
            let w0 = r0.weekday();
            let s0 = r0.sansculottide();
            assert_ne!(w0.is_some(), s0.is_some());
            let r1 = FrenchRevArith::<false>::from_fixed(t0);
            let w1 = r1.weekday();
            let s1 = r1.sansculottide();
            assert_ne!(w1.is_some(), s1.is_some());
        }
    }
}

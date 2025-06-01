// http://positivists.org/calendar.html
// https://gallica.bnf.fr/ark:/12148/bpt6k21868f/f42.planchecontact
// https://books.google.ca/books?id=S_BRAAAAMAAJ&printsec=frontcover&source=gbs_ge_summary_r&cad=0#v=onepage&q&f=false

// Calendier Positiviste Page 52-53
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
use crate::day_cycle::Weekday;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const POSITIVIST_YEAR_OFFSET: i16 = 1789 - 1;

// Calendier Positiviste Page 19
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum PositivistMonth {
    Moses = 1,
    Homer,
    Aristotle,
    Archimedes,
    Caesar,
    SaintPaul,
    Charlemagne,
    Dante,
    Gutenburg,
    Shakespeare,
    Descartes,
    Frederick,
    Bichat,
}

// Calendier Positiviste Page 8
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum PositivistComplementaryDay {
    FestivalOfTheDead = 1,
    FestivalOfHolyWomen,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Positivist(CommonDate);

impl Positivist {
    pub fn year(self) -> i32 {
        self.0.year
    }

    pub fn month(self) -> Option<PositivistMonth> {
        if self.0.month == 14 {
            None
        } else {
            PositivistMonth::from_u8(self.0.month)
        }
    }

    pub fn day(self) -> u8 {
        self.0.day
    }

    // Calendier Positiviste Page 8
    pub fn complementary(self) -> Option<PositivistComplementaryDay> {
        if self.0.month == 14 {
            PositivistComplementaryDay::from_u8(self.0.day)
        } else {
            None
        }
    }

    // Calendier Positiviste Page 23-30
    pub fn weekday(self) -> Option<Weekday> {
        if self.0.month == 14 {
            None
        } else {
            Weekday::from_i64((self.0.day as i64).modulus(7))
        }
    }

    // Not sure about the source for this...
    pub fn is_leap(p_year: i32) -> bool {
        Gregorian::is_leap((POSITIVIST_YEAR_OFFSET as i32) + p_year)
    }

    pub fn complementary_count(p_year: i32) -> u8 {
        if Positivist::is_leap(p_year) {
            2
        } else {
            1
        }
    }
}

impl CalculatedBounds for Positivist {}

impl Epoch for Positivist {
    fn epoch() -> Fixed {
        Gregorian::new_year(POSITIVIST_YEAR_OFFSET)
    }
}

impl FromFixed for Positivist {
    fn from_fixed(date: Fixed) -> Positivist {
        let ord = Gregorian::ordinal_from_fixed(date);
        let year = ord.year - (POSITIVIST_YEAR_OFFSET as i32);
        let month = (((ord.day_of_year - 1) as i64).div_euclid(28) + 1) as u8;
        let day = (ord.day_of_year as i64).adjusted_remainder(28) as u8;
        debug_assert!(day > 0 && day < 29);
        Positivist(CommonDate::new(year, month, day))
    }
}

impl ToFixed for Positivist {
    fn to_fixed(self) -> Fixed {
        let y = self.0.year + (POSITIVIST_YEAR_OFFSET as i32);
        let offset_y = Gregorian::try_from_common_date(CommonDate::new(y, 1, 1))
            .expect("month 1, day 1 is always valid for Gregorian")
            .to_fixed()
            .get_day_i()
            - 1;
        let offset_m = ((self.0.month as i64) - 1) * 28;
        Fixed::cast_new(offset_y + offset_m + (self.0.day as i64))
    }
}

impl ToFromCommonDate for Positivist {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        if date.month < 1 || date.month > 14 {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.month < 14 && date.day > 28 {
            Err(CalendarError::InvalidDay)
        } else if date.month == 14 && date.day > Positivist::complementary_count(date.year) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::gregorian::GregorianMonth;
    use crate::common::bound::EffectiveBound;
    use crate::day_count::RataDie;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use proptest::proptest;
    const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

    #[test]
    fn bounds_actually_work() {
        assert!(
            Positivist::from_fixed(Fixed::effective_min())
                < Positivist::from_fixed(Fixed::cast_new(0))
        );
        assert!(
            Positivist::from_fixed(Fixed::effective_max())
                > Positivist::from_fixed(Fixed::cast_new(0))
        );
    }

    #[test]
    fn example_from_text() {
        //The Positivist Calendar, page 37
        let dg = Gregorian::try_from_common_date(CommonDate::new(1855, 1, 1)).unwrap();
        let dp = Positivist::try_from_common_date(CommonDate::new(67, 1, 1)).unwrap();
        let fg = dg.to_fixed();
        let fp = dp.to_fixed();
        assert_eq!(fg, fp);
    }

    proptest! {
        #[test]
        fn valid_day(t0 in FIXED_MIN..FIXED_MAX) {
            let t = Fixed::new(t0);
            let e1 = Positivist::from_fixed(t);
            assert!(Positivist::valid_month_day(e1.to_common_date()).is_ok());
        }

        #[test]
        fn complementary_xor_weekday(t in FIXED_MIN..FIXED_MAX) {
            let t0 = RataDie::new(t).to_fixed().to_day();
            let r0 = Positivist::from_fixed(t0);
            let w0 = r0.weekday();
            let s0 = r0.complementary();
            assert_ne!(w0.is_some(), s0.is_some());
        }

        #[test]
        fn start_with_gregorian(year in -MAX_YEARS..MAX_YEARS) {
            let d = CommonDate::new(year, 1, 1);
            let p = Positivist::try_from_common_date(d).unwrap();
            let g = Gregorian::from_fixed(p.to_fixed());
            assert_eq!(g.year(), year + 1788);
            assert_eq!(g.month(), GregorianMonth::January);
            assert_eq!(g.day(), 1);
            assert_eq!(p.weekday().unwrap(), Weekday::Monday);
        }

        #[test]
        fn end_with_gregorian(year in -MAX_YEARS..MAX_YEARS) {
            let d = CommonDate::new(year, 14, 1);
            let p = Positivist::try_from_common_date(d).unwrap();
            let g = Gregorian::from_fixed(p.to_fixed());

            assert_eq!(p.complementary().unwrap(), PositivistComplementaryDay::FestivalOfTheDead);
            assert_eq!(g.year(), year + 1788);
            assert_eq!(g.month(), GregorianMonth::December);
            if Gregorian::is_leap(g.year()) {
                assert_eq!(g.day(), 30);

                let d = CommonDate::new(year, 14, 2);
                let p = Positivist::try_from_common_date(d).unwrap();
                let g = Gregorian::from_fixed(p.to_fixed());
                assert_eq!(p.complementary().unwrap(), PositivistComplementaryDay::FestivalOfHolyWomen);
                assert_eq!(g.year(), year + 1788);
                assert_eq!(g.month(), GregorianMonth::December);
                assert_eq!(g.day(), 31);
            } else {
                assert_eq!(g.day(), 31);
            }

        }

        #[test]
        fn invalid_common(year in -MAX_YEARS..MAX_YEARS, month in 15..u8::MAX, day in 29..u8::MAX) {
            let d_list = [
                CommonDate{ year, month, day },
                CommonDate{ year, month: 1, day},
                CommonDate{ year, month, day: 1 },
                CommonDate{ year, month: 1, day: 0},
                CommonDate{ year, month: 0, day: 1 }
            ];
            for d in d_list {
                assert!(Positivist::try_from_common_date(d).is_err());
            }
        }
    }
}

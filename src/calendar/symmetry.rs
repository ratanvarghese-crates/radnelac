// "Basic Symmetry454 and Symmetry010 Calendar Arithmetic" by Dr. Irvin L. Bromberg
// https://kalendis.free.nf/Symmetry454-Arithmetic.pdf
// > Symmetry454 and Symmetry010 calendar arithmetic is very simple, but there is a tendency
// > for those who are programming their first implementation of these calendars to immediately
// > cut corners that may suffice for a limited range of dates, or to skip thorough validation
// > of their implementation.
// > Please don’t deviate from the arithmetic outlined herein. Please “stick to the script”.
// > Don’t try to invent your own arithmetic using novel expressions. There is no reason to do
// > so, because this arithmetic is in the public domain, royalty free. The algorithm steps
// > documented herein were carefully designed for efficiency, simplicity, and clarity of
// > program code, and were thoroughly validated. Cutting corners will most likely result in
// > harder-to-read programs that are more difficult to maintain and troubleshoot. In all
// > probability a novel expression intended to “simplify” the arithmetic documented herein
// > will actually prove to function erroneously under specific circumstances. It is just not
// > worth wasting the time on the trouble that will make for you.

// Approximate northward equinox
use crate::calendar::gregorian::Gregorian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::CommonDay;
use crate::common::date::CommonYear;
use crate::common::date::GuaranteedMonth;
use crate::common::date::HasLeapYears;
use crate::common::date::Quarter;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use std::num::NonZero;

use crate::common::math::TermNum;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

#[allow(non_snake_case)]
struct SymmetryParams {
    C: i64,
    L: i64,
    K: i64,
}

const NORTHWARD_EQUINOX_PARAMS: SymmetryParams = SymmetryParams {
    C: 293,
    L: 52,
    K: 146,
};

const NORTH_SOLSTICE_PARAMS: SymmetryParams = SymmetryParams {
    C: 389,
    L: 69,
    K: 194,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum SymmetryMonth {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    Irvember,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Symmetry<const T: bool, const U: bool>(CommonDate);

pub type Symmetry454 = Symmetry<true, true>;
pub type Symmetry010 = Symmetry<false, true>;
pub type Symmetry454Solstice = Symmetry<true, false>;
pub type Symmetry010Solstice = Symmetry<false, false>;

impl<const T: bool, const U: bool> Symmetry<T, U> {
    fn params() -> SymmetryParams {
        if U {
            NORTHWARD_EQUINOX_PARAMS
        } else {
            NORTH_SOLSTICE_PARAMS
        }
    }

    pub fn mode(self) -> (bool, bool) {
        (T, U)
    }

    pub fn new_year_day_unchecked(sym_year: i32, sym_epoch: i64) -> i64 {
        let p = Self::params();
        #[allow(non_snake_case)]
        let E = (sym_year - 1) as i64;
        sym_epoch + (364 * E) + (7 * ((p.L * E) + p.K).div_euclid(p.C))
    }

    fn days_before_month_454(sym_month: i64) -> u16 {
        ((28 * (sym_month - 1)) + 7 * sym_month.div_euclid(3)) as u16
    }

    fn days_before_month_010(sym_month: i64) -> u16 {
        ((30 * (sym_month - 1)) + sym_month.div_euclid(3)) as u16
    }

    fn days_before_month(sym_month: u8) -> u16 {
        if T {
            Self::days_before_month_454(sym_month as i64)
        } else {
            Self::days_before_month_010(sym_month as i64)
        }
    }

    fn day_of_year(sym_month: u8, sym_day: u8) -> u16 {
        Self::days_before_month(sym_month) + (sym_day as u16)
    }

    fn year_from_fixed(fixed: i64, epoch: i64) -> (i32, i64) {
        // Very tempting to cut "corners here" to avoid floating point.
        // But the notice at the top of the file reminds us to "stick to the script"
        let fixed_date = fixed as f64;
        let sym_epoch = epoch as f64;
        let cycle_mean_year = if U {
            365.0 + (71.0 / 293.0)
        } else {
            365.0 + (94.0 / 389.0)
        };
        let sym_year = ((fixed_date - sym_epoch) / cycle_mean_year).ceil() as i32;
        let start_of_year = Self::new_year_day_unchecked(sym_year, epoch);
        if start_of_year < fixed {
            if (fixed - start_of_year) >= 364 {
                let start_of_next_year = Self::new_year_day_unchecked(sym_year + 1, epoch);
                if fixed >= start_of_next_year {
                    (sym_year + 1, start_of_next_year)
                } else {
                    (sym_year, start_of_year)
                }
            } else {
                (sym_year, start_of_year)
            }
        } else if start_of_year > fixed {
            (
                sym_year - 1,
                Self::new_year_day_unchecked(sym_year - 1, epoch),
            )
        } else {
            (sym_year, start_of_year)
        }
    }

    pub fn days_in_month(month: SymmetryMonth) -> u8 {
        let sym_month = month as i64;
        if sym_month == 13 {
            7
        } else if T {
            (28 + (7 * (sym_month.modulus(3).div_euclid(2)))) as u8
        } else {
            (30 + sym_month.modulus(3).div_euclid(2)) as u8
        }
    }
}

impl<const T: bool, const U: bool> HasLeapYears for Symmetry<T, U> {
    fn is_leap(sym_year: i32) -> bool {
        let p = Self::params();
        let sym_year = sym_year as i64;
        ((p.L * sym_year) + p.K).modulus(p.C) < p.L
    }
}

impl<const T: bool, const U: bool> CalculatedBounds for Symmetry<T, U> {}

impl<const T: bool, const U: bool> Epoch for Symmetry<T, U> {
    fn epoch() -> Fixed {
        Gregorian::epoch()
    }
}

impl<const T: bool, const U: bool> FromFixed for Symmetry<T, U> {
    fn from_fixed(fixed_date: Fixed) -> Symmetry<T, U> {
        let date = fixed_date.get_day_i();
        let (sym_year, start_of_year) = Self::year_from_fixed(date, Self::epoch().get_day_i());
        let day_of_year = (date - start_of_year + 1) as u16;
        // Thank Ferris for div_ceil
        let week_of_year = day_of_year.div_ceil(7);
        debug_assert!(week_of_year > 0 && week_of_year < 54);
        let quarter = (4 * week_of_year).div_ceil(53);
        debug_assert!(quarter > 0 && quarter < 5);
        let day_of_quarter = day_of_year - (91 * (quarter - 1));
        let week_of_quarter = day_of_quarter.div_ceil(7);
        let month_of_quarter = if T {
            (2 * week_of_quarter).div_ceil(9)
        } else {
            (2 * day_of_quarter).div_ceil(61)
        };
        let sym_month = (3 * (quarter - 1) + month_of_quarter) as u8;
        // Skipping optionals
        let sym_day = (day_of_year - Self::days_before_month(sym_month)) as u8;
        Self(CommonDate::new(sym_year, sym_month, sym_day))
    }
}

impl<const T: bool, const U: bool> ToFixed for Symmetry<T, U> {
    fn to_fixed(self) -> Fixed {
        let new_year_day = Self::new_year_day_unchecked(self.0.year, Self::epoch().get_day_i());
        let day_of_year = Self::day_of_year(self.0.month, self.0.day) as i64;
        Fixed::cast_new(new_year_day + day_of_year - 1)
    }
}

impl<const T: bool, const U: bool> ToFromCommonDate for Symmetry<T, U> {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = SymmetryMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > Self::days_in_month(month_opt.unwrap()) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }
}

impl<const T: bool, const U: bool> Quarter for Symmetry<T, U> {
    fn quarter(self) -> NonZero<u8> {
        let m = self.to_common_date().month;
        if m == 13 {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new(((m - 1) / 3) + 1).expect("(m-1)/3 > -1")
        }
    }
}

impl<const T: bool, const U: bool> CommonYear for Symmetry<T, U> {}
impl<const T: bool, const U: bool> GuaranteedMonth<SymmetryMonth> for Symmetry<T, U> {}
impl<const T: bool, const U: bool> CommonDay for Symmetry<T, U> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::RataDie;
    use crate::day_count::FIXED_MAX;
    use crate::day_cycle::Weekday;
    use proptest::proptest;
    const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

    #[test]
    fn is_leap_example() {
        assert!(Symmetry454::is_leap(2009));
        assert!(Symmetry454::is_leap(2015));
        assert!(Symmetry010::is_leap(2009));
        assert!(Symmetry010::is_leap(2015));

        assert!(!Symmetry454Solstice::is_leap(2009));
        assert!(!Symmetry010Solstice::is_leap(2009));
        assert!(Symmetry454Solstice::is_leap(2010));
        assert!(Symmetry010Solstice::is_leap(2016));
    }

    #[test]
    fn new_year_day_example() {
        assert_eq!(Symmetry454::new_year_day_unchecked(2010, 1), 733776);
        assert_eq!(Symmetry010::new_year_day_unchecked(2010, 1), 733776);
        assert_eq!(Symmetry454Solstice::new_year_day_unchecked(2010, 1), 733769);
        assert_eq!(Symmetry010Solstice::new_year_day_unchecked(2010, 1), 733769);
    }

    #[test]
    fn days_before_month() {
        assert_eq!(Symmetry454::days_before_month(1), 0);
        assert_eq!(Symmetry454::days_before_month(13), 364);
        assert_eq!(Symmetry454::days_before_month(6), 154);
        assert_eq!(Symmetry010::days_before_month(1), 0);
        assert_eq!(Symmetry010::days_before_month(13), 364);
        assert_eq!(Symmetry010::days_before_month(6), 152);

        assert_eq!(Symmetry454Solstice::days_before_month(1), 0);
        assert_eq!(Symmetry454Solstice::days_before_month(13), 364);
        assert_eq!(Symmetry454Solstice::days_before_month(6), 154);
        assert_eq!(Symmetry010Solstice::days_before_month(1), 0);
        assert_eq!(Symmetry010Solstice::days_before_month(13), 364);
        assert_eq!(Symmetry010Solstice::days_before_month(6), 152);
    }

    #[test]
    fn day_of_year() {
        assert_eq!(Symmetry454::day_of_year(6, 17), 171);
        assert_eq!(Symmetry010::day_of_year(6, 17), 169);
        assert_eq!(Symmetry454Solstice::day_of_year(6, 17), 171);
        assert_eq!(Symmetry010Solstice::day_of_year(6, 17), 169);
    }

    #[test]
    fn to_fixed() {
        assert_eq!(
            Symmetry454::try_from_common_date(CommonDate::new(2009, 4, 5))
                .unwrap()
                .to_fixed()
                .get_day_i(),
            733500
        );
        assert_eq!(
            Symmetry010::try_from_common_date(CommonDate::new(2009, 4, 5))
                .unwrap()
                .to_fixed()
                .get_day_i(),
            733500
        );
        assert_eq!(
            Symmetry454Solstice::try_from_common_date(CommonDate::new(2009, 4, 5))
                .unwrap()
                .to_fixed()
                .get_day_i(),
            733500
        );
        assert_eq!(
            Symmetry010Solstice::try_from_common_date(CommonDate::new(2009, 4, 5))
                .unwrap()
                .to_fixed()
                .get_day_i(),
            733500
        );
    }

    #[test]
    fn year_from_fixed() {
        assert_eq!(Symmetry454::year_from_fixed(733649, 1), (2009, 733405));
        assert_eq!(Symmetry010::year_from_fixed(733649, 1), (2009, 733405));
        assert_eq!(Symmetry454::year_from_fixed(733406, 1), (2009, 733405));
        assert_eq!(Symmetry010::year_from_fixed(733406, 1), (2009, 733405));
        assert_eq!(Symmetry454::year_from_fixed(733774, 1).0, 2009);
        assert_eq!(Symmetry010::year_from_fixed(733774, 1).0, 2009);
    }

    #[test]
    fn example_data() {
        let d_list = [
            (
                -44444 as i32,
                CommonDate::new(-121, 4, 27),
                CommonDate::new(-121, 4, 27),
                CommonDate::new(-121, 4, 27),
                CommonDate::new(-121, 4, 27),
            ),
            (
                -33333 as i32,
                CommonDate::new(-91, 9, 22),
                CommonDate::new(-91, 9, 24),
                CommonDate::new(-91, 9, 22),
                CommonDate::new(-91, 9, 24),
            ),
            (
                44444 as i32,
                CommonDate::new(122, 9, 8),
                CommonDate::new(122, 9, 10),
                CommonDate::new(122, 9, 8),
                CommonDate::new(122, 9, 10),
            ),
            (
                648491 as i32,
                CommonDate::new(1776, 7, 4),
                CommonDate::new(1776, 7, 4),
                CommonDate::new(1776, 7, 4),
                CommonDate::new(1776, 7, 4),
            ),
            (
                681724 as i32,
                CommonDate::new(1867, 7, 1),
                CommonDate::new(1867, 7, 1),
                CommonDate::new(1867, 7, 1),
                CommonDate::new(1867, 7, 1),
            ),
            (
                711058 as i32,
                CommonDate::new(1947, 10, 26),
                CommonDate::new(1947, 10, 26),
                CommonDate::new(1947, 10, 26),
                CommonDate::new(1947, 10, 26),
            ),
            (
                728515 as i32,
                CommonDate::new(1995, 8, 11),
                CommonDate::new(1995, 8, 9),
                CommonDate::new(1995, 8, 11),
                CommonDate::new(1995, 8, 9),
            ),
            (
                730179 as i32,
                CommonDate::new(2000, 2, 30),
                CommonDate::new(2000, 2, 28),
                CommonDate::new(2000, 2, 30),
                CommonDate::new(2000, 2, 28),
            ),
            (
                731703 as i32,
                CommonDate::new(2004, 5, 7),
                CommonDate::new(2004, 5, 5),
                CommonDate::new(2004, 5, 7),
                CommonDate::new(2004, 5, 5),
            ),
            (
                731946 as i32,
                CommonDate::new(2004, 13, 5),
                CommonDate::new(2004, 13, 5),
                CommonDate::new(2005, 1, 5),
                CommonDate::new(2005, 1, 5),
            ),
            (
                737475 as i32,
                CommonDate::new(2020, 2, 25),
                CommonDate::new(2020, 2, 23),
                CommonDate::new(2020, 2, 25),
                CommonDate::new(2020, 2, 23),
            ),
            (
                811236 as i32,
                CommonDate::new(2222, 2, 6),
                CommonDate::new(2222, 2, 4),
                CommonDate::new(2222, 2, 6),
                CommonDate::new(2222, 2, 4),
            ),
            (
                1217048 as i32,
                CommonDate::new(3333, 2, 35),
                CommonDate::new(3333, 3, 2),
                CommonDate::new(3333, 2, 35),
                CommonDate::new(3333, 3, 2),
            ),
        ];
        for item in d_list {
            let rd = RataDie::new(item.0 as f64);
            let s454q = Symmetry454::try_from_common_date(item.1).unwrap();
            let s010q = Symmetry010::try_from_common_date(item.2).unwrap();
            let s454s = Symmetry454Solstice::try_from_common_date(item.3).unwrap();
            let s010s = Symmetry010Solstice::try_from_common_date(item.4).unwrap();
            assert_eq!(rd.to_fixed(), s454q.to_fixed());
            assert_eq!(rd.to_fixed(), s010q.to_fixed());
            assert_eq!(rd.to_fixed(), s454s.to_fixed());
            assert_eq!(rd.to_fixed(), s010s.to_fixed());
        }
    }

    proptest! {
        #[test]
        fn month_start_on_monday_454(year in -MAX_YEARS..MAX_YEARS, month in 1..12) {
            let c = CommonDate::new(year as i32, month as u8, 1);
            let d = Symmetry454::try_from_common_date(c).unwrap();
            assert_eq!(d.convert::<Weekday>(), Weekday::Monday);
            let d = Symmetry454Solstice::try_from_common_date(c).unwrap();
            assert_eq!(d.convert::<Weekday>(), Weekday::Monday);
        }

        #[test]
        fn month_end_on_sunday_454(year in -MAX_YEARS..MAX_YEARS, month in 1..12) {
            let m = SymmetryMonth::from_i32(month).unwrap();
            let c = CommonDate::new(year as i32, month as u8, Symmetry454::days_in_month(m));
            let d = Symmetry454::try_from_common_date(c).unwrap();
            assert_eq!(d.convert::<Weekday>(), Weekday::Sunday);
            let d = Symmetry454Solstice::try_from_common_date(c).unwrap();
            assert_eq!(d.convert::<Weekday>(), Weekday::Sunday);
        }

        #[test]
        fn no_friday_13_454(year in -MAX_YEARS..MAX_YEARS, month in 1..12) {
            let c = CommonDate::new(year as i32, month as u8, 13);
            let d = Symmetry454::try_from_common_date(c).unwrap();
            assert_ne!(d.convert::<Weekday>(), Weekday::Friday);
            let d = Symmetry454Solstice::try_from_common_date(c).unwrap();
            assert_ne!(d.convert::<Weekday>(), Weekday::Friday);
        }

        #[test]
        fn year_start_on_monday_010(year in -MAX_YEARS..MAX_YEARS) {
            let c = CommonDate::new(year as i32, 1, 1);
            let d = Symmetry010::try_from_common_date(c).unwrap();
            assert_eq!(d.convert::<Weekday>(), Weekday::Monday);
            let d = Symmetry010Solstice::try_from_common_date(c).unwrap();
            assert_eq!(d.convert::<Weekday>(), Weekday::Monday);
        }

        #[test]
        fn no_friday_13_010(year in -MAX_YEARS..MAX_YEARS, month in 1..12) {
            let c = CommonDate::new(year as i32, month as u8, 13);
            let d = Symmetry010::try_from_common_date(c).unwrap();
            assert_ne!(d.convert::<Weekday>(), Weekday::Friday);
            let d = Symmetry010Solstice::try_from_common_date(c).unwrap();
            assert_ne!(d.convert::<Weekday>(), Weekday::Friday);
        }
    }
}

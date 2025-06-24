use proptest::proptest;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::CotsworthComplementaryDay;
use radnelac::calendar::FrenchRevArith;
use radnelac::calendar::FrenchRevWeekday;
use radnelac::calendar::Positivist;
use radnelac::calendar::PositivistComplementaryDay;
use radnelac::calendar::Sansculottide;
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::TranquilityComplementaryDay;
use radnelac::calendar::TranquilityMoment;
use radnelac::bound::BoundedDayCount;
use radnelac::date::CommonDate;
use radnelac::date::ToFromCommonDate;
use radnelac::day_count::FromFixed;
use radnelac::day_count::RataDie;
use radnelac::day_count::ToFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use radnelac::day_cycle::Weekday;

use radnelac::date::PerennialWithComplementaryDay;

const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

fn complementary_xor_weekday<T, U, V: PerennialWithComplementaryDay<T, U> + FromFixed>(t: f64) {
    let t0 = RataDie::new(t).to_fixed().to_day();
    let r0 = V::from_fixed(t0);
    let w0 = r0.weekday();
    let s0 = r0.complementary();
    assert_ne!(w0.is_some(), s0.is_some());
    assert_ne!(w0.is_none(), s0.is_none());
}

fn perennial<
    T,
    U: std::cmp::PartialEq + std::fmt::Debug,
    V: PerennialWithComplementaryDay<T, U> + ToFromCommonDate,
>(
    y0: i32,
    y1: i32,
    month: u8,
    day: u8,
) {
    let d0 = V::try_from_common_date(CommonDate::new(y0, month, day)).unwrap();
    let d1 = V::try_from_common_date(CommonDate::new(y1, month, day)).unwrap();
    assert_eq!(d0.weekday(), d1.weekday());
}

fn simple_perennial<T: ToFromCommonDate + ToFixed>(y0: i32, y1: i32, month: u8, day: u8) {
    let d0 = T::try_from_common_date(CommonDate::new(y0, month, day)).unwrap();
    let d1 = T::try_from_common_date(CommonDate::new(y1, month, day)).unwrap();
    let f0 = d0.to_fixed();
    let f1 = d1.to_fixed();
    assert_eq!(Weekday::from_fixed(f0), Weekday::from_fixed(f1));
}

proptest! {
    #[test]
    fn cotsworth_complementary_xor_weekday(t in FIXED_MIN..FIXED_MAX) {
        complementary_xor_weekday::<CotsworthComplementaryDay, Weekday, Cotsworth>(t);
    }

    #[test]
    fn cotsworth_perennial(y0 in -MAX_YEARS..MAX_YEARS, y1 in -MAX_YEARS..MAX_YEARS, month in 1..13, day in 1..28) {
        perennial::<CotsworthComplementaryDay, Weekday, Cotsworth>(y0, y1, month as u8, day as u8);
    }

    #[test]
    fn french_rev_arith_complementary_xor_weekday(t in FIXED_MIN..FIXED_MAX) {
        complementary_xor_weekday::<Sansculottide, FrenchRevWeekday, FrenchRevArith<true>>(t);
        complementary_xor_weekday::<Sansculottide, FrenchRevWeekday, FrenchRevArith<false>>(t);
    }

    #[test]
    fn french_rev_arith_perennial(y0 in -MAX_YEARS..MAX_YEARS, y1 in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        perennial::<Sansculottide, FrenchRevWeekday, FrenchRevArith<true>>(y0, y1, month as u8, day as u8);
        perennial::<Sansculottide, FrenchRevWeekday, FrenchRevArith<false>>(y0, y1, month as u8, day as u8);
    }

    #[test]
    fn positivist_complementary_xor_weekday(t in FIXED_MIN..FIXED_MAX) {
        complementary_xor_weekday::<PositivistComplementaryDay, Weekday, Positivist>(t);
    }

    #[test]
    fn positivist_perennial(y0 in -MAX_YEARS..MAX_YEARS, y1 in -MAX_YEARS..MAX_YEARS, month in 1..13, day in 1..28) {
        perennial::<PositivistComplementaryDay, Weekday, Positivist>(y0, y1, month as u8, day as u8);
    }

    #[test]
    fn tranquility_complementary_xor_weekday(t in FIXED_MIN..FIXED_MAX) {
        complementary_xor_weekday::<TranquilityComplementaryDay, Weekday, TranquilityMoment>(t);
    }

    #[test]
    fn tranquility_perennial(y0 in -MAX_YEARS..MAX_YEARS, y1 in -MAX_YEARS..MAX_YEARS, month in 1..13, day in 1..28) {
        perennial::<TranquilityComplementaryDay, Weekday, TranquilityMoment>(y0, y1, month as u8, day as u8);
    }

    #[test]
    fn symmetry_perennial(y0 in -MAX_YEARS..MAX_YEARS, y1 in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..28) {
        simple_perennial::<Symmetry010>(y0, y1, month as u8, day as u8);
        simple_perennial::<Symmetry454>(y0, y1, month as u8, day as u8);
        simple_perennial::<Symmetry010Solstice>(y0, y1, month as u8, day as u8);
        simple_perennial::<Symmetry454Solstice>(y0, y1, month as u8, day as u8);
    }
}

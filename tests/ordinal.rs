use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use proptest::prop_assume;
use proptest::proptest;
use radnelac::calendar::Gregorian;
use radnelac::calendar::GregorianMonth;
use radnelac::calendar::HasLeapYears;
use radnelac::calendar::OrdinalDate;
use radnelac::calendar::ToFromCommonDate;
use radnelac::calendar::ToFromOrdinalDate;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::day_count::FIXED_MAX;
use std::fmt::Debug;

const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

fn year_start<
    S: FromPrimitive + ToPrimitive,
    T: ToFromCommonDate<S> + ToFromOrdinalDate + Debug,
>(
    year: i32,
    year_len: u16,
) {
    let d0 = T::year_start(year);
    let d1 = T::year_end(year);
    let ord0 = OrdinalDate {
        year: year,
        day_of_year: 1,
    };
    let ord1 = OrdinalDate {
        year: year,
        day_of_year: year_len,
    };
    assert_eq!(d0.to_ordinal(), ord0);
    assert_eq!(d1.to_ordinal(), ord1);
}

proptest! {
    #[test]
    fn valid_gregorian(year: i32, day in 1..365) {
        let ord = OrdinalDate{ year: year, day_of_year: day as u16 };
        Gregorian::valid_ordinal(ord).unwrap();
    }

    #[test]
    fn invalid_gregorian(year: i32, day in 367..u16::MAX) {
        let ord0 = OrdinalDate{ year: year, day_of_year: 0 };
        let ord1 = OrdinalDate{ year: year, day_of_year: day as u16 };
        let ord2 = OrdinalDate{ year: year, day_of_year: 366 };
        assert!(Gregorian::valid_ordinal(ord0).is_err());
        assert!(Gregorian::valid_ordinal(ord1).is_err());
        assert_eq!(Gregorian::valid_ordinal(ord2).is_err(), !Gregorian::is_leap(year));
    }

    #[test]
    fn year_start_gregorian(year in -MAX_YEARS..MAX_YEARS) {
        let len = if Gregorian::is_leap(year) { 366 } else { 365 };
        year_start::<GregorianMonth, Gregorian>(year, len);
    }

    #[test]
    fn valid_tranquility(year: i32, day in 1..365) {
        prop_assume!(year != 0);
        let ord = OrdinalDate{ year: year, day_of_year: day as u16 };
        TranquilityMoment::valid_ordinal(ord).unwrap();
    }

    #[test]
    fn invalid_tranquility(year: i32, day in 367..u16::MAX) {
        let ord0 = OrdinalDate{ year: year, day_of_year: 0 };
        let ord1 = OrdinalDate{ year: year, day_of_year: day as u16 };
        let ord2 = OrdinalDate{ year: year, day_of_year: 366 };
        assert!(TranquilityMoment::valid_ordinal(ord0).is_err());
        assert!(TranquilityMoment::valid_ordinal(ord1).is_err());
        assert_eq!(TranquilityMoment::valid_ordinal(ord2).is_err(), !TranquilityMoment::is_leap(year));
    }

    #[test]
    fn year_start_tranquility(year in -MAX_YEARS..MAX_YEARS) {
        let len = if TranquilityMoment::is_leap(year) { 366 } else { 365 };
        year_start::<TranquilityMonth, TranquilityMoment>(year, len);
    }
}

use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use proptest::prop_assume;
use proptest::proptest;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::CotsworthMonth;
use radnelac::calendar::Egyptian;
use radnelac::calendar::EgyptianMonth;
use radnelac::calendar::Gregorian;
use radnelac::calendar::GregorianMonth;
use radnelac::calendar::HasLeapYears;
use radnelac::calendar::OrdinalDate;
use radnelac::calendar::Positivist;
use radnelac::calendar::PositivistMonth;
use radnelac::calendar::ToFromCommonDate;
use radnelac::calendar::ToFromOrdinalDate;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::day_count::FIXED_MAX;
use std::fmt::Debug;

const MAX_YEARS: i32 = (FIXED_MAX / 366.0) as i32; //Deliberately smaller than other files

fn year_start<
    S: FromPrimitive + ToPrimitive,
    T: ToFromCommonDate<S> + ToFromOrdinalDate + Debug,
>(
    year: i32,
    year_len: u16,
) {
    let d0 = T::try_year_start(year).unwrap();
    let d1 = T::try_year_end(year).unwrap();
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
    fn valid_cotsworth(year: i32, day in 1..365) {
        let ord = OrdinalDate{ year: year, day_of_year: day as u16 };
        Cotsworth::valid_ordinal(ord).unwrap();
    }

    #[test]
    fn invalid_cotsworth(year: i32, day in 367..u16::MAX) {
        let ord0 = OrdinalDate{ year: year, day_of_year: 0 };
        let ord1 = OrdinalDate{ year: year, day_of_year: day as u16 };
        let ord2 = OrdinalDate{ year: year, day_of_year: 366 };
        assert!(Cotsworth::valid_ordinal(ord0).is_err());
        assert!(Cotsworth::valid_ordinal(ord1).is_err());
        assert_eq!(Cotsworth::valid_ordinal(ord2).is_err(), !Cotsworth::is_leap(year));
    }

    #[test]
    fn year_start_cotsworth(year in -MAX_YEARS..MAX_YEARS) {
        let len = if Cotsworth::is_leap(year) { 366 } else { 365 };
        year_start::<CotsworthMonth, Cotsworth>(year, len);
    }

    #[test]
    fn valid_egyptian(year: i32, day in 1..365) {
        let ord = OrdinalDate{ year: year, day_of_year: day as u16 };
        Egyptian::valid_ordinal(ord).unwrap();
    }

    #[test]
    fn invalid_egyptian(year: i32, day in 366..u16::MAX) {
        let ord0 = OrdinalDate{ year: year, day_of_year: 0 };
        let ord1 = OrdinalDate{ year: year, day_of_year: day as u16 };
        assert!(Egyptian::valid_ordinal(ord0).is_err());
        assert!(Egyptian::valid_ordinal(ord1).is_err());
    }

    #[test]
    fn year_start_egyptian(year in -MAX_YEARS..MAX_YEARS) {
        year_start::<EgyptianMonth, Egyptian>(year, 365);
    }

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
    fn valid_positivist(year: i32, day in 1..365) {
        let ord = OrdinalDate{ year: year, day_of_year: day as u16 };
        Positivist::valid_ordinal(ord).unwrap();
    }

    #[test]
    fn invalid_positivist(year: i32, day in 367..u16::MAX) {
        let ord0 = OrdinalDate{ year: year, day_of_year: 0 };
        let ord1 = OrdinalDate{ year: year, day_of_year: day as u16 };
        let ord2 = OrdinalDate{ year: year, day_of_year: 366 };
        assert!(Positivist::valid_ordinal(ord0).is_err());
        assert!(Positivist::valid_ordinal(ord1).is_err());
        assert_eq!(Positivist::valid_ordinal(ord2).is_err(), !Positivist::is_leap(year));
    }

    #[test]
    fn year_start_positivist(year in -MAX_YEARS..MAX_YEARS) {
        let len = if Positivist::is_leap(year) { 366 } else { 365 };
        year_start::<PositivistMonth, Positivist>(year, len);
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

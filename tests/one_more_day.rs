use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use proptest::prop_assume;
use proptest::proptest;
use radnelac::calendar::Armenian;
use radnelac::calendar::ArmenianMonth;
use radnelac::calendar::CommonDate;
use radnelac::calendar::Coptic;
use radnelac::calendar::CopticMonth;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::CotsworthMonth;
use radnelac::calendar::Egyptian;
use radnelac::calendar::EgyptianMonth;
use radnelac::calendar::Ethiopic;
use radnelac::calendar::EthiopicMonth;
use radnelac::calendar::FrenchRevArith;
use radnelac::calendar::FrenchRevMonth;
use radnelac::calendar::Gregorian;
use radnelac::calendar::GregorianMonth;
use radnelac::calendar::HasIntercalaryDays;
use radnelac::calendar::Holocene;
use radnelac::calendar::HoloceneMonth;
use radnelac::calendar::Julian;
use radnelac::calendar::JulianMonth;
use radnelac::calendar::Positivist;
use radnelac::calendar::PositivistMonth;
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::SymmetryMonth;
use radnelac::calendar::ToFromCommonDate;
use radnelac::calendar::ToFromOrdinalDate;
use radnelac::calendar::TranquilityComplementaryDay;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::day_count::BoundedDayCount;
use radnelac::day_count::Epoch;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use std::fmt::Debug;

fn one_more_day<S: FromPrimitive + ToPrimitive, T: ToFromCommonDate<S> + FromFixed + Debug>(
    t: f64,
) {
    let f0 = Fixed::new(t);
    let f1 = Fixed::new(t + 1.0);
    let d0 = T::from_fixed(f0).to_common_date();
    let d1 = T::from_fixed(f1).to_common_date();
    if d0.year != d1.year {
        assert_eq!(d1.year, d0.year + 1);
        assert_eq!(d1.month, 1);
        assert_eq!(d1.day, 1);
        assert_eq!(d0, T::year_end_date(d0.year));
        assert_eq!(d1, T::year_start_date(d1.year));
    } else if d0.month != d1.month {
        assert_eq!(d1.year, d0.year);
        assert_eq!(d1.month, d0.month + 1);
        assert_eq!(d1.day, 1);
    } else if d0.day != d1.day {
        assert_eq!(d1.year, d0.year);
        assert_eq!(d1.month, d0.month);
        assert_eq!(d1.day, d0.day + 1);
    } else {
        panic!("Added one day but the dates are equal");
    }
}

fn one_more_day_ordinal<T: FromFixed + ToFromOrdinalDate>(t: f64) {
    let f0 = Fixed::new(t);
    let f1 = Fixed::new(t + 1.0);
    let ord0 = T::ordinal_from_fixed(f0);
    let ord1 = T::ordinal_from_fixed(f1);
    if ord0.year == ord1.year {
        assert_eq!(ord1.year, ord0.year);
        assert_eq!(ord1.day_of_year, ord0.day_of_year + 1);
    } else {
        assert_eq!(ord1.year, ord0.year + 1);
        assert_eq!(ord1.day_of_year, 1);
    }
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<ArmenianMonth, Armenian>(t);
    }

    #[test]
    fn armenian_ordinal(t in FIXED_MIN..FIXED_MAX) {
        one_more_day_ordinal::<Armenian>(t);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<CopticMonth, Coptic>(t);
    }

    #[test]
    fn coptic_ordinal(t in FIXED_MIN..FIXED_MAX) {
        one_more_day_ordinal::<Coptic>(t);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<CotsworthMonth, Cotsworth>(t);
    }

    #[test]
    fn cotsworth_ordinal(t in FIXED_MIN..FIXED_MAX) {
        one_more_day_ordinal::<Cotsworth>(t);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<EgyptianMonth, Egyptian>(t);
        one_more_day_ordinal::<Egyptian>(t);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<EthiopicMonth, Ethiopic>(t);
        one_more_day_ordinal::<Ethiopic>(t);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<FrenchRevMonth, FrenchRevArith<true>>(t);
        one_more_day::<FrenchRevMonth, FrenchRevArith<false>>(t);
        one_more_day_ordinal::<FrenchRevArith<true>>(t);
        one_more_day_ordinal::<FrenchRevArith<false>>(t);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<GregorianMonth, Gregorian>(t);
        one_more_day_ordinal::<Gregorian>(t);
    }

    #[test]
    fn gregorian_ordinal(t in FIXED_MIN..FIXED_MAX) {
        one_more_day_ordinal::<Gregorian>(t);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<HoloceneMonth, Holocene>(t);
    }

    #[test]
    fn julian_ad(t in FIXED_MIN..-7.0) {
        //Avoiding year 0
        one_more_day::<JulianMonth, Julian>(t);
    }

    #[test]
    fn julian_bc(t in 7.0..FIXED_MAX) {
        //Avoiding year 0
        one_more_day::<JulianMonth, Julian>(t);
    }

    #[test]
    fn julian_ad_ordinal(t in FIXED_MIN..-7.0) {
        //Avoiding year 0
        one_more_day_ordinal::<Julian>(t);
    }

    #[test]
    fn julian_bc_ordinal(t in 7.0..FIXED_MAX) {
        //Avoiding year 0
        one_more_day_ordinal::<Julian>(t);
    }


    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<PositivistMonth, Positivist>(t);
    }

    #[test]
    fn positivist_ordinal(t in FIXED_MIN..FIXED_MAX) {
        one_more_day_ordinal::<Positivist>(t);
    }

    #[test]
    fn symmetry(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<SymmetryMonth, Symmetry454>(t);
        one_more_day::<SymmetryMonth, Symmetry010>(t);
        one_more_day::<SymmetryMonth, Symmetry454Solstice>(t);
        one_more_day::<SymmetryMonth, Symmetry010Solstice>(t);
    }

    #[test]
    fn symmetry_ordinal(t in FIXED_MIN..FIXED_MAX) {
        one_more_day_ordinal::<Symmetry454>(t);
        one_more_day_ordinal::<Symmetry010>(t);
        one_more_day_ordinal::<Symmetry454Solstice>(t);
        one_more_day_ordinal::<Symmetry010Solstice>(t);
    }

    #[test]
    fn tranquility(dt in FIXED_MIN..FIXED_MAX) {
        let t = TranquilityMoment::epoch().get() + dt;
        let tq0 = TranquilityMoment::from_fixed(Fixed::new(t));
        let tq1 = TranquilityMoment::from_fixed(Fixed::new(t + 1.0));
        match (tq0.complementary(), tq1.complementary()) {
            (None, None) => one_more_day::<TranquilityMonth, TranquilityMoment>(t),
            (None, Some(TranquilityComplementaryDay::AldrinDay)) => {
                assert_eq!(tq0.to_common_date(), CommonDate::new(tq0.year(), 8, 27))
            },
            (None, _) => {
                assert_eq!(tq0.to_common_date(), CommonDate::new(tq0.year(), 13, 28))
            },
            (Some(TranquilityComplementaryDay::AldrinDay), None) => {
                assert_eq!(tq1.to_common_date(), CommonDate::new(tq1.year(), 8, 28))
            },
            (_, None) => {
                assert_eq!(tq1.to_common_date(), CommonDate::new(tq1.year(), 1, 1))
            },
            (_, _) => panic!("Impossible")
        }
    }

    #[test]
    fn tranquility_ordinal(dt in FIXED_MIN..FIXED_MAX) {
        prop_assume!(dt < -1.0 || dt > 1.0);
        let t = TranquilityMoment::epoch().get() + dt;
        one_more_day_ordinal::<TranquilityMoment>(t);
    }
}

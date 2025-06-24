use num_traits::FromPrimitive;
use proptest::prop_assume;
use proptest::proptest;
use radnelac::calendar::Armenian;
use radnelac::calendar::ArmenianMonth;
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
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::date::CommonDate;
use radnelac::date::HasLeapYears;
use radnelac::date::TryMonth;
use radnelac::day_count::FIXED_MAX;

const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

fn common_date_roundtrip<T: FromPrimitive, U: TryMonth<T>, const V: bool>(d: CommonDate) {
    let e0 = U::try_from_common_date(d).unwrap();
    let m = e0.try_month();
    if V {
        assert!(m.is_some());
    } else {
        assert!(m.is_none());
    }
    assert_eq!(e0.to_common_date(), d);
}

fn month_is_some<T: FromPrimitive, U: TryMonth<T>>(year: i32, month: u8, day: u8) {
    let d = CommonDate {
        year: year,
        month: month as u8,
        day: day as u8,
    };
    common_date_roundtrip::<T, U, true>(d);
}

fn month_is_none<T: FromPrimitive, U: TryMonth<T>>(year: i32, month: u8, day: u8) {
    let d = CommonDate {
        year: year,
        month: month as u8,
        day: day as u8,
    };
    common_date_roundtrip::<T, U, false>(d);
}

proptest! {
    #[test]
    fn armenian_month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        month_is_some::<ArmenianMonth, Armenian>(year, month as u8, day as u8);
    }

    #[test]
    fn armenian_month_is_none(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
        month_is_none::<ArmenianMonth, Armenian>(year, 13, day as u8);
    }

    #[test]
    fn coptic_month_normal(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        month_is_some::<CopticMonth, Coptic>(year, month as u8, day as u8);
    }

    #[test]
    fn coptic_month_month_13(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
        month_is_some::<CopticMonth, Coptic>(year, 13, day as u8);
        if Coptic::is_leap(year) {
            month_is_some::<CopticMonth, Coptic>(year, 13, 6);
        }
    }

    #[test]
    fn cotsworth_month_normal(year in -MAX_YEARS..MAX_YEARS, month in 1..13, day in 1..28) {
        month_is_some::<CotsworthMonth, Cotsworth>(year, month as u8, day as u8);
    }

    #[test]
    fn cotsworth_month_edge_case(year in -MAX_YEARS..MAX_YEARS) {
        month_is_some::<CotsworthMonth, Cotsworth>(year, 13, 29);
        if Cotsworth::is_leap(year) {
            month_is_some::<CotsworthMonth, Cotsworth>(year, 6, 29);
        }
    }

    #[test]
    fn egyptian_month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        month_is_some::<EgyptianMonth, Egyptian>(year, month as u8, day as u8);
    }

    #[test]
    fn egyptian_month_is_none(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
        month_is_none::<EgyptianMonth, Egyptian>(year, 13, day as u8);
    }

    #[test]
    fn ethiopic_month_normal(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        month_is_some::<EthiopicMonth, Ethiopic>(year, month as u8, day as u8);
    }

    #[test]
    fn ethiopic_month_month_13(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
        month_is_some::<EthiopicMonth, Ethiopic>(year, 13, day as u8);
        if Ethiopic::is_leap(year) {
            month_is_some::<EthiopicMonth, Ethiopic>(year, 13, 6);
        }
    }

    #[test]
    fn french_rev_month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        month_is_some::<FrenchRevMonth, FrenchRevArith<true>>(year, month as u8, day as u8);
        month_is_some::<FrenchRevMonth, FrenchRevArith<false>>(year, month as u8, day as u8);
    }

    #[test]
    fn french_rev_month_is_none(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
        month_is_none::<FrenchRevMonth, FrenchRevArith<true>>(year, 13, day as u8);
        month_is_none::<FrenchRevMonth, FrenchRevArith<false>>(year, 13, day as u8);
        if FrenchRevArith::<true>::is_leap(year) {
            month_is_none::<FrenchRevMonth, FrenchRevArith<true>>(year, 13, 6);
        }
        if FrenchRevArith::<false>::is_leap(year) {
            month_is_none::<FrenchRevMonth, FrenchRevArith<false>>(year, 13, 6);
        }
    }

    #[test]
    fn gregorian_month_start(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..27) {
        month_is_some::<GregorianMonth, Gregorian>(year, month as u8, day as u8);
    }

    #[test]
    fn gregorian_month_end(year in -MAX_YEARS..MAX_YEARS, month in 1..12) {
        let leap = Gregorian::is_leap(year);
        let min: u8 = 28;
        let max: u8 = GregorianMonth::from_u8(month as u8).unwrap().length(leap) + 1;
        for d in min..max {
            month_is_some::<GregorianMonth, Gregorian>(year, month as u8, d as u8);
        }
    }

    #[test]
    fn holocene_month_start(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..27) {
        month_is_some::<HoloceneMonth, Holocene>(year, month as u8, day as u8);
    }

    #[test]
    fn holocene_month_end(year in -MAX_YEARS..MAX_YEARS, month in 1..12) {
        let leap = Holocene::is_leap(year);
        let min: u8 = 28;
        let max: u8 = HoloceneMonth::from_u8(month as u8).unwrap().length(leap) + 1;
        for d in min..max {
            month_is_some::<HoloceneMonth, Holocene>(year, month as u8, d as u8);
        }
    }

    #[test]
    fn julian_month_start(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..27) {
        prop_assume!(year != 0);
        month_is_some::<JulianMonth, Julian>(year, month as u8, day as u8);
    }

    #[test]
    fn julian_month_end(year in -MAX_YEARS..MAX_YEARS, month in 1..12) {
        prop_assume!(year != 0);
        let leap = Julian::is_leap(year);
        let min: u8 = 28;
        let max: u8 = JulianMonth::from_u8(month as u8).unwrap().length(leap) + 1;
        for d in min..max {
            month_is_some::<JulianMonth, Julian>(year, month as u8, d as u8);
        }
    }

    #[test]
    fn positivist_month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..13, day in 1..28) {
        month_is_some::<PositivistMonth, Positivist>(year, month as u8, day as u8);
    }

    #[test]
    fn positivist_month_is_none(year in -MAX_YEARS..MAX_YEARS) {
        month_is_none::<PositivistMonth, Positivist>(year, 14, 1);
        if Positivist::is_leap(year) {
            month_is_none::<PositivistMonth, Positivist>(year, 14, 2);
        }
    }

    #[test]
    fn symmetry_normal_month_start(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..27) {
        month_is_some::<SymmetryMonth, Symmetry010>(year, month as u8, day as u8);
        month_is_some::<SymmetryMonth, Symmetry454>(year, month as u8, day as u8);
        month_is_some::<SymmetryMonth, Symmetry010Solstice>(year, month as u8, day as u8);
        month_is_some::<SymmetryMonth, Symmetry454Solstice>(year, month as u8, day as u8);
    }

    #[test]
    fn symmetry_normal_month_end(year in -MAX_YEARS..MAX_YEARS, month in 1..12) {
        let min: u8 = 28;
        let max: u8 = Symmetry010::days_in_month(SymmetryMonth::from_u8(month as u8).unwrap()) + 1;
        for d in min..max {
            month_is_some::<SymmetryMonth, Symmetry010>(year, month as u8, d as u8);
        }
        let max: u8 = Symmetry454::days_in_month(SymmetryMonth::from_u8(month as u8).unwrap()) + 1;
        for d in min..max {
            month_is_some::<SymmetryMonth, Symmetry454>(year, month as u8, d as u8);
        }
        let max: u8 = Symmetry010Solstice::days_in_month(SymmetryMonth::from_u8(month as u8).unwrap()) + 1;
        for d in min..max {
            month_is_some::<SymmetryMonth, Symmetry010Solstice>(year, month as u8, d as u8);
        }
        let max: u8 = Symmetry454Solstice::days_in_month(SymmetryMonth::from_u8(month as u8).unwrap()) + 1;
        for d in min..max {
            month_is_some::<SymmetryMonth, Symmetry454Solstice>(year, month as u8, d as u8);
        }
    }

    #[test]
    fn symmetry_equinox_irvember(year in -MAX_YEARS..MAX_YEARS, day in 1..7) {
        //TODO: implement arbitrary for these dates
        //prop_assume!(Symmetry010::is_leap(year) || Symmetry454::is_leap(year));
        if Symmetry010::is_leap(year) {
            month_is_some::<SymmetryMonth, Symmetry010>(year, 13, day as u8);
        }
        if Symmetry454::is_leap(year) {
            month_is_some::<SymmetryMonth, Symmetry454>(year, 13, day as u8);
        }
    }

    #[test]
    fn symmetry_solstice_irvember(year in -MAX_YEARS..MAX_YEARS, day in 1..7) {
        //TODO: implement arbitrary for these dates
        //prop_assume!(Symmetry010Solstice::is_leap(year) || Symmetry454Solstice::is_leap(year));
        if Symmetry010Solstice::is_leap(year) {
            month_is_some::<SymmetryMonth, Symmetry010Solstice>(year, 13, day as u8);
        }
        if Symmetry454Solstice::is_leap(year) {
            month_is_some::<SymmetryMonth, Symmetry454Solstice>(year, 13, day as u8);
        }
    }

    #[test]
    fn tranquility_month_is_some(year in -MAX_YEARS..MAX_YEARS, month in 1..13, day in 1..28) {
        prop_assume!(year != 0);
        month_is_some::<TranquilityMonth, TranquilityMoment>(year, month as u8, day as u8);
    }

    #[test]
    fn tranquility_month_is_none(year in -MAX_YEARS..MAX_YEARS) {
        prop_assume!(year != 0 && year != -1);
        month_is_none::<TranquilityMonth, TranquilityMoment>(year, 0, 1);
        if TranquilityMoment::is_leap(year) {
            month_is_none::<TranquilityMonth, TranquilityMoment>(year, 0, 2);
        }
    }
}

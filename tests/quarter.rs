use proptest::prop_assume;
use proptest::proptest;
use radnelac::calendar::Armenian;
use radnelac::calendar::Coptic;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::Egyptian;
use radnelac::calendar::Ethiopic;
use radnelac::calendar::FrenchRevArith;
use radnelac::calendar::Gregorian;
use radnelac::calendar::Holocene;
use radnelac::calendar::Julian;
use radnelac::calendar::Positivist;
use radnelac::calendar::Roman;
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::ISO;
use radnelac::common::bound::BoundedDayCount;
use radnelac::common::date::CommonDate;
use radnelac::common::date::Quarter;
use radnelac::common::date::ToFromCommonDate;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;

const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

fn quarter_tomorrow<T: Quarter + FromFixed>(f: f64) {
    let t0 = Fixed::new(f);
    let t1 = Fixed::new(t0.get() + 1.0);
    let q0 = T::from_fixed(t0).quarter().get();
    let q1 = T::from_fixed(t1).quarter().get();
    assert!(q0 < 5);
    assert!(q1 < 5);
    if q0 < 4 {
        assert!(q1 == q0 || q1 == (q0 + 1), "q0 = {}; q1 = {}", q0, q1);
    } else {
        assert!(q1 == q0 || q1 == 1, "q0 = {}; q1 = {}", q0, q1);
    }
}

fn quarter_boundary<T: Quarter + FromFixed + ToFromCommonDate>(year: i32, month: u8, q0: u8) {
    let c = CommonDate::new(year, month, 1);
    let q1 = T::try_from_common_date(c).unwrap().quarter().get();
    assert_eq!(
        q1, q0,
        "year = {}; month = {}; q1 = {}; q0 = {};",
        year, month, q1, q0
    );
}

fn quarter_boundary_m12<T: Quarter + FromFixed + ToFromCommonDate>(year: i32) {
    quarter_boundary::<T>(year, 1, 1);
    quarter_boundary::<T>(year, 3, 1);
    quarter_boundary::<T>(year, 4, 2);
    quarter_boundary::<T>(year, 6, 2);
    quarter_boundary::<T>(year, 7, 3);
    quarter_boundary::<T>(year, 9, 3);
    quarter_boundary::<T>(year, 10, 4);
    quarter_boundary::<T>(year, 12, 4);
}

fn quarter_boundary_m13<T: Quarter + FromFixed + ToFromCommonDate>(year: i32) {
    quarter_boundary_m12::<T>(year);
    quarter_boundary::<T>(year, 13, 4);
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Armenian>(t);
    }

    #[test]
    fn armenian_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<Armenian>(y);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Coptic>(t);
    }

    #[test]
    fn coptic_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<Coptic>(y);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Cotsworth>(t);
    }

    #[test]
    fn cotsworth_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<Cotsworth>(y);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Egyptian>(t);
    }

    #[test]
    fn egyptian_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<Egyptian>(y);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Ethiopic>(t);
    }

    #[test]
    fn ethiopic_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<Ethiopic>(y);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<FrenchRevArith<true>>(t);
        quarter_tomorrow::<FrenchRevArith<false>>(t);
    }

    #[test]
    fn french_rev_arith_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<FrenchRevArith<true>>(y);
        quarter_boundary_m13::<FrenchRevArith<false>>(y);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Gregorian>(t);
    }

    #[test]
    fn gregorian_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m12::<Gregorian>(y);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Holocene>(t);
    }

    #[test]
    fn holocene_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m12::<Holocene>(y);
    }

    #[test]
    fn iso(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<ISO>(t);
    }

    #[test]
    fn julian(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Julian>(t);
    }

    #[test]
    fn julian_boundary(y in -MAX_YEARS..MAX_YEARS) {
        prop_assume!(y != 0);
        quarter_boundary_m12::<Julian>(y);
    }

    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Positivist>(t);
    }

    #[test]
    fn positivist_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<Positivist>(y);
    }

    #[test]
    fn roman(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Roman>(t);
    }

    #[test]
    fn symmetry(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Symmetry454>(t);
        quarter_tomorrow::<Symmetry010>(t);
        quarter_tomorrow::<Symmetry454Solstice>(t);
        quarter_tomorrow::<Symmetry010Solstice>(t);
    }

    #[test]
    fn symmetry_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m12::<Symmetry454>(y);
        quarter_boundary_m12::<Symmetry010>(y);
        quarter_boundary_m12::<Symmetry454Solstice>(y);
        quarter_boundary_m12::<Symmetry010Solstice>(y);
    }

    #[test]
    fn tranquility(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<TranquilityMoment>(t);
    }

    #[test]
    fn tranquility_boundary(y in -MAX_YEARS..MAX_YEARS) {
        prop_assume!(y != 0 && y != -1);
        quarter_boundary_m13::<TranquilityMoment>(y);
    }
}

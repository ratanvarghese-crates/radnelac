// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
use radnelac::calendar::Holocene;
use radnelac::calendar::HoloceneMonth;
use radnelac::calendar::Julian;
use radnelac::calendar::JulianMonth;
use radnelac::calendar::Positivist;
use radnelac::calendar::PositivistMonth;
use radnelac::calendar::Quarter;
use radnelac::calendar::Roman;
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::SymmetryMonth;
use radnelac::calendar::ToFromCommonDate;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::calendar::ISO;
use radnelac::day_count::BoundedDayCount;
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

fn quarter_boundary<
    S: FromPrimitive + ToPrimitive,
    T: Quarter + FromFixed + ToFromCommonDate<S>,
>(
    year: i32,
    month: u8,
    q0: u8,
) {
    let c = CommonDate::new(year, month, 1);
    let q1 = T::try_from_common_date(c).unwrap().quarter().get();
    assert_eq!(
        q1, q0,
        "year = {}; month = {}; q1 = {}; q0 = {};",
        year, month, q1, q0
    );
}

fn quarter_boundary_m12<
    S: FromPrimitive + ToPrimitive,
    T: Quarter + FromFixed + ToFromCommonDate<S>,
>(
    year: i32,
) {
    quarter_boundary::<S, T>(year, 1, 1);
    quarter_boundary::<S, T>(year, 3, 1);
    quarter_boundary::<S, T>(year, 4, 2);
    quarter_boundary::<S, T>(year, 6, 2);
    quarter_boundary::<S, T>(year, 7, 3);
    quarter_boundary::<S, T>(year, 9, 3);
    quarter_boundary::<S, T>(year, 10, 4);
    quarter_boundary::<S, T>(year, 12, 4);
}

fn quarter_boundary_m13<
    S: FromPrimitive + ToPrimitive,
    T: Quarter + FromFixed + ToFromCommonDate<S>,
>(
    year: i32,
) {
    quarter_boundary_m12::<S, T>(year);
    quarter_boundary::<S, T>(year, 13, 4);
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Armenian>(t);
    }

    #[test]
    fn armenian_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<ArmenianMonth, Armenian>(y);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Coptic>(t);
    }

    #[test]
    fn coptic_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<CopticMonth, Coptic>(y);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Cotsworth>(t);
    }

    #[test]
    fn cotsworth_boundary(year in -MAX_YEARS..MAX_YEARS) {
        // https://archive.org/details/rationalalmanact00cotsuoft/page/n1/mode/2up
        // Not the true quarter boundaries because they don't lie on month boundaries
        quarter_boundary::<CotsworthMonth, Cotsworth>(year, 1, 1);
        quarter_boundary::<CotsworthMonth, Cotsworth>(year, 4, 1);
        quarter_boundary::<CotsworthMonth, Cotsworth>(year, 5, 2);
        quarter_boundary::<CotsworthMonth, Cotsworth>(year, 7, 2);
        quarter_boundary::<CotsworthMonth, Cotsworth>(year, 8, 3);
        quarter_boundary::<CotsworthMonth, Cotsworth>(year, 10, 3);
        quarter_boundary::<CotsworthMonth, Cotsworth>(year, 11, 4);
        quarter_boundary::<CotsworthMonth, Cotsworth>(year, 13, 4);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Egyptian>(t);
    }

    #[test]
    fn egyptian_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<EgyptianMonth, Egyptian>(y);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Ethiopic>(t);
    }

    #[test]
    fn ethiopic_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<EthiopicMonth, Ethiopic>(y);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<FrenchRevArith<true>>(t);
        quarter_tomorrow::<FrenchRevArith<false>>(t);
    }

    #[test]
    fn french_rev_arith_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<FrenchRevMonth, FrenchRevArith<true>>(y);
        quarter_boundary_m13::<FrenchRevMonth, FrenchRevArith<false>>(y);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Gregorian>(t);
    }

    #[test]
    fn gregorian_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m12::<GregorianMonth, Gregorian>(y);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Holocene>(t);
    }

    #[test]
    fn holocene_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m12::<HoloceneMonth, Holocene>(y);
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
        quarter_boundary_m12::<JulianMonth, Julian>(y);
    }

    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<Positivist>(t);
    }

    #[test]
    fn positivist_boundary(y in -MAX_YEARS..MAX_YEARS) {
        quarter_boundary_m13::<PositivistMonth, Positivist>(y);
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
        quarter_boundary_m12::<SymmetryMonth, Symmetry454>(y);
        quarter_boundary_m12::<SymmetryMonth, Symmetry010>(y);
        quarter_boundary_m12::<SymmetryMonth, Symmetry454Solstice>(y);
        quarter_boundary_m12::<SymmetryMonth, Symmetry010Solstice>(y);
    }

    #[test]
    fn tranquility(t in FIXED_MIN..FIXED_MAX) {
        quarter_tomorrow::<TranquilityMoment>(t);
    }

    #[test]
    fn tranquility_boundary(y in -MAX_YEARS..MAX_YEARS) {
        prop_assume!(y != 0 && y != -1);
        quarter_boundary_m13::<TranquilityMonth, TranquilityMoment>(y);
    }
}

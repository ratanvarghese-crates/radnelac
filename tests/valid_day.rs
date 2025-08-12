// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
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
use radnelac::calendar::ToFromCommonDate;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::day_count::BoundedDayCount;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use std::fmt::Debug;

fn valid_day<S: FromPrimitive + ToPrimitive, T: FromFixed + ToFromCommonDate<S> + Debug>(t: f64) {
    let f = Fixed::new(t);
    let d = T::from_fixed(f);
    assert!(T::valid_ymd(d.to_common_date()).is_ok());
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<ArmenianMonth, Armenian>(t);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<CopticMonth, Coptic>(t);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<CotsworthMonth, Cotsworth>(t);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<EgyptianMonth, Egyptian>(t);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<EthiopicMonth, Ethiopic>(t);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<FrenchRevMonth, FrenchRevArith<true>>(t);
        valid_day::<FrenchRevMonth, FrenchRevArith<false>>(t);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<GregorianMonth, Gregorian>(t);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<HoloceneMonth, Holocene>(t);
    }

    #[test]
    fn julian(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<JulianMonth, Julian>(t);
    }

    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<PositivistMonth, Positivist>(t);
    }

    #[test]
    fn symmetry(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<SymmetryMonth, Symmetry010>(t);
        valid_day::<SymmetryMonth, Symmetry454>(t);
        valid_day::<SymmetryMonth, Symmetry010Solstice>(t);
        valid_day::<SymmetryMonth, Symmetry454Solstice>(t);
    }

    #[test]
    fn tranquility(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<TranquilityMonth, TranquilityMoment>(t);
    }
}

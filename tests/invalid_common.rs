// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
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
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::SymmetryMonth;
use radnelac::calendar::ToFromCommonDate;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use std::fmt::Debug;

const MAX_YEARS: i32 = ((FIXED_MAX / 365.25) - 10000.0) as i32;
const MIN_YEARS: i32 = ((FIXED_MIN / 365.25) - 10000.0) as i32;

fn invalid_common_internal<S: FromPrimitive + ToPrimitive, T: ToFromCommonDate<S> + Debug>(
    year: i32,
    month: u8,
    day: u8,
    allow_m0: bool,
) {
    let d_list = [
        CommonDate::new(year, month, day),
        CommonDate::new(year, 1, day),
        CommonDate::new(year, month, 1),
        CommonDate::new(year, 1, 0),
    ];
    for d in d_list {
        assert!(T::try_from_common_date(d).is_err());
    }
    if !allow_m0 {
        assert!(T::try_from_common_date(CommonDate::new(year, 0, 1)).is_err());
    }
}

fn invalid_common<S: ToPrimitive + FromPrimitive, T: ToFromCommonDate<S> + Debug>(
    year: i32,
    month: u8,
    day: u8,
) {
    invalid_common_internal::<S, T>(year, month, day, false);
}

proptest! {
    #[test]
    fn armenian(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
        invalid_common::<ArmenianMonth, Armenian>(year, month, day);
    }

    #[test]
    fn coptic(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
        invalid_common::<CopticMonth, Coptic>(year, month, day);
    }

    #[test]
    fn cotsworth(year in MIN_YEARS..MAX_YEARS, month in 15..u8::MAX, day in 30..u8::MAX) {
        invalid_common::<CotsworthMonth, Cotsworth>(year, month, day);
    }

    #[test]
    fn egyptian(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
        invalid_common::<EgyptianMonth, Egyptian>(year, month, day);
    }

    #[test]
    fn ethiopic(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
        invalid_common::<EthiopicMonth, Ethiopic>(year, month, day);
    }

    #[test]
    fn french_rev_arith(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<FrenchRevMonth, FrenchRevArith<true>>(year, month, day);
        invalid_common::<FrenchRevMonth, FrenchRevArith<false>>(year, month, day);
    }

    #[test]
    fn gregorian(year in MIN_YEARS..MAX_YEARS, month in 13..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<GregorianMonth, Gregorian>(year, month, day);
    }

    #[test]
    fn holocene(year in MIN_YEARS..MAX_YEARS, month in 13..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<HoloceneMonth, Holocene>(year, month, day);
    }

    #[test]
    fn julian(year in MIN_YEARS..MAX_YEARS, month in 13..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<JulianMonth, Julian>(year, month, day);
    }

    #[test]
    fn positivist(year in -MAX_YEARS..MAX_YEARS, month in 15..u8::MAX, day in 29..u8::MAX) {
        invalid_common::<PositivistMonth, Positivist>(year, month, day);
    }

    #[test]
    fn symmetry(year in -MAX_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<SymmetryMonth, Symmetry010>(year, month, day);
        invalid_common::<SymmetryMonth, Symmetry454>(year, month, day);
        invalid_common::<SymmetryMonth, Symmetry010Solstice>(year, month, day);
        invalid_common::<SymmetryMonth, Symmetry454Solstice>(year, month, day);
    }

    #[test]
    fn tranquility(year in -MAX_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 32..u8::MAX) {
        invalid_common_internal::<TranquilityMonth, TranquilityMoment>(year, month, day, true);
    }
}

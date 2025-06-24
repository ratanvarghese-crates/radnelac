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
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::TranquilityMoment;
use radnelac::date::CommonDate;
use radnelac::date::ToFromCommonDate;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use std::fmt::Debug;

const MAX_YEARS: i32 = ((FIXED_MAX / 365.25) - 10000.0) as i32;
const MIN_YEARS: i32 = ((FIXED_MIN / 365.25) - 10000.0) as i32;

fn invalid_common_internal<T: ToFromCommonDate + Debug>(
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

fn invalid_common<T: ToFromCommonDate + Debug>(year: i32, month: u8, day: u8) {
    invalid_common_internal::<T>(year, month, day, false);
}

proptest! {
    #[test]
    fn armenian(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
        invalid_common::<Armenian>(year, month, day);
    }

    #[test]
    fn coptic(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
        invalid_common::<Coptic>(year, month, day);
    }

    #[test]
    fn cotsworth(year in MIN_YEARS..MAX_YEARS, month in 15..u8::MAX, day in 30..u8::MAX) {
        invalid_common::<Cotsworth>(year, month, day);
    }

    #[test]
    fn egyptian(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
        invalid_common::<Egyptian>(year, month, day);
    }

    #[test]
    fn ethiopic(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
        invalid_common::<Ethiopic>(year, month, day);
    }

    #[test]
    fn french_rev_arith(year in MIN_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<FrenchRevArith<true>>(year, month, day);
        invalid_common::<FrenchRevArith<false>>(year, month, day);
    }

    #[test]
    fn gregorian(year in MIN_YEARS..MAX_YEARS, month in 13..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<Gregorian>(year, month, day);
    }

    #[test]
    fn holocene(year in MIN_YEARS..MAX_YEARS, month in 13..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<Holocene>(year, month, day);
    }

    #[test]
    fn julian(year in MIN_YEARS..MAX_YEARS, month in 13..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<Julian>(year, month, day);
    }

    #[test]
    fn positivist(year in -MAX_YEARS..MAX_YEARS, month in 15..u8::MAX, day in 29..u8::MAX) {
        invalid_common::<Positivist>(year, month, day);
    }

    #[test]
    fn symmetry(year in -MAX_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 32..u8::MAX) {
        invalid_common::<Symmetry010>(year, month, day);
        invalid_common::<Symmetry454>(year, month, day);
        invalid_common::<Symmetry010Solstice>(year, month, day);
        invalid_common::<Symmetry454Solstice>(year, month, day);
    }

    #[test]
    fn tranquility(year in -MAX_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 32..u8::MAX) {
        invalid_common_internal::<TranquilityMoment>(year, month, day, true);
    }
}

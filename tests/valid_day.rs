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
use radnelac::day_count::BoundedDayCount;
use radnelac::calendar::ToFromCommonDate;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use std::fmt::Debug;

fn valid_day<T: FromFixed + ToFromCommonDate + Debug>(t: f64) {
    let f = Fixed::new(t);
    let d = T::from_fixed(f);
    assert!(T::valid_month_day(d.to_common_date()).is_ok());
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Armenian>(t);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Coptic>(t);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Cotsworth>(t);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Egyptian>(t);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Ethiopic>(t);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<FrenchRevArith<true>>(t);
        valid_day::<FrenchRevArith<false>>(t);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Gregorian>(t);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Holocene>(t);
    }

    #[test]
    fn julian(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Julian>(t);
    }

    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Positivist>(t);
    }

    #[test]
    fn symmetry(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<Symmetry010>(t);
        valid_day::<Symmetry454>(t);
        valid_day::<Symmetry010Solstice>(t);
        valid_day::<Symmetry454Solstice>(t);
    }

    #[test]
    fn tranquility(t in FIXED_MIN..FIXED_MAX) {
        valid_day::<TranquilityMoment>(t);
    }
}

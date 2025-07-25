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
use radnelac::day_count::BoundedDayCount;
use radnelac::calendar::ToFromCommonDate;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use std::fmt::Debug;

fn consistent_order_basic<T: FromFixed + PartialOrd + Debug>(t0: f64, t1: f64) {
    let f0 = Fixed::new(t0).to_day();
    let f1 = Fixed::new(t1).to_day();
    let d0 = T::from_fixed(f0);
    let d1 = T::from_fixed(f1);
    assert_eq!(f0 < f1, d0 < d1);
    assert_eq!(f0 <= f1, d0 <= d1);
    assert_eq!(f0 == f1, d0 == d1);
    assert_eq!(f0 >= f1, d0 >= d1);
    assert_eq!(f0 > f1, d0 > d1);
}

fn consistent_order<T: FromFixed + ToFromCommonDate + PartialOrd + Debug>(t0: f64, t1: f64) {
    consistent_order_basic::<T>(t0, t1);
    let f0 = Fixed::new(t0).to_day();
    let f1 = Fixed::new(t1).to_day();
    let c0 = T::from_fixed(f0).to_common_date();
    let c1 = T::from_fixed(f1).to_common_date();
    assert_eq!(f0 < f1, c0 < c1);
    assert_eq!(f0 <= f1, c0 <= c1);
    assert_eq!(f0 == f1, c0 == c1);
    assert_eq!(f0 >= f1, c0 >= c1);
    assert_eq!(f0 > f1, c0 > c1);
}

proptest! {
    #[test]
    fn armenian(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Armenian>(t0, t1);
    }

    #[test]
    fn armenian_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Armenian>(t0, t0 + (diff as f64));
    }

    #[test]
    fn coptic(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Coptic>(t0, t1);
    }

    #[test]
    fn coptic_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Coptic>(t0, t0 + (diff as f64));
    }

    #[test]
    fn cotsworth(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Cotsworth>(t0, t1);
    }

    #[test]
    fn cotsworth_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Cotsworth>(t0, t0 + (diff as f64));
    }

    #[test]
    fn egyptian(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Egyptian>(t0, t1);
    }

    #[test]
    fn egyptian_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Egyptian>(t0, t0 + (diff as f64));
    }

    #[test]
    fn ethiopic(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Ethiopic>(t0, t1);
    }

    #[test]
    fn ethiopic_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Ethiopic>(t0, t0 + (diff as f64));
    }

    #[test]
    fn french_rev_arith(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<FrenchRevArith<true>>(t0, t1);
        consistent_order::<FrenchRevArith<false>>(t0, t1);
    }

    #[test]
    fn french_rev_arith_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<FrenchRevArith<true>>(t0, t0 + (diff as f64));
        consistent_order::<FrenchRevArith<false>>(t0, t0 + (diff as f64));
    }

    #[test]
    fn gregorian(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Gregorian>(t0, t1);
    }

    #[test]
    fn gregorian_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Gregorian>(t0, t0 + (diff as f64));
    }

    #[test]
    fn holocene(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Holocene>(t0, t1);
    }

    #[test]
    fn holocene_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Holocene>(t0, t0 + (diff as f64));
    }

    #[test]
    fn iso(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order_basic::<ISO>(t0, t1);
    }

    #[test]
    fn iso_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order_basic::<ISO>(t0, t0 + (diff as f64));
    }

    #[test]
    fn julian(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Julian>(t0, t1);
    }

    #[test]
    fn julian_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Julian>(t0, t0 + (diff as f64));
    }

    #[test]
    fn positivist(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Positivist>(t0, t1);
    }

    #[test]
    fn positivist_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Positivist>(t0, t0 + (diff as f64));
    }

    #[test]
    fn roman(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order_basic::<Roman>(t0, t1);
    }

    #[test]
    fn roman_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order_basic::<Roman>(t0, t0 + (diff as f64));
    }

    #[test]
    fn symmetry(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<Symmetry454>(t0, t1);
        consistent_order::<Symmetry010>(t0, t1);
        consistent_order::<Symmetry454Solstice>(t0, t1);
        consistent_order::<Symmetry010Solstice>(t0, t1);
    }

    #[test]
    fn symmetry_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<Symmetry454>(t0, t0 + (diff as f64));
        consistent_order::<Symmetry010>(t0, t0 + (diff as f64));
        consistent_order::<Symmetry454Solstice>(t0, t0 + (diff as f64));
        consistent_order::<Symmetry010Solstice>(t0, t0 + (diff as f64));
    }

    #[test]
    fn tranquility(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order_basic::<TranquilityMoment>(t0, t1);
    }

    #[test]
    fn tranquility_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order_basic::<TranquilityMoment>(t0, t0 + (diff as f64));
    }
}

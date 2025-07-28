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
use radnelac::calendar::Roman;
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::SymmetryMonth;
use radnelac::calendar::ToFromCommonDate;
use radnelac::calendar::ToFromOrdinalDate;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::ISO;
use radnelac::day_count::BoundedDayCount;
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

fn consistent_order<
    S: FromPrimitive + ToPrimitive,
    T: FromFixed + ToFromCommonDate<S> + PartialOrd + Debug,
>(
    t0: f64,
    t1: f64,
) {
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

fn consistent_order_ordinal<T: FromFixed + PartialEq + Debug + ToFromOrdinalDate>(
    t0: f64,
    t1: f64,
) {
    let f0 = Fixed::new(t0).to_day();
    let f1 = Fixed::new(t1).to_day();
    let ord0 = T::from_fixed(f0).to_ordinal();
    let ord1 = T::from_fixed(f1).to_ordinal();
    assert_eq!(f0 < f1, ord0 < ord1);
    assert_eq!(f0 <= f1, ord0 <= ord1);
    assert_eq!(f0 == f1, ord0 == ord1);
    assert_eq!(f0 >= f1, ord0 >= ord1);
    assert_eq!(f0 > f1, ord0 > ord1);
}

proptest! {
    #[test]
    fn armenian(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<ArmenianMonth, Armenian>(t0, t1);
        consistent_order_ordinal::<Armenian>(t0, t1);
    }

    #[test]
    fn armenian_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<ArmenianMonth, Armenian>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Armenian>(t0, t0 + (diff as f64));
    }

    #[test]
    fn coptic(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<CopticMonth, Coptic>(t0, t1);
    }

    #[test]
    fn coptic_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<CopticMonth, Coptic>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Coptic>(t0, t0 + (diff as f64));
    }

    #[test]
    fn cotsworth(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<CotsworthMonth, Cotsworth>(t0, t1);
        consistent_order_ordinal::<Cotsworth>(t0, t1);
    }

    #[test]
    fn cotsworth_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<CotsworthMonth, Cotsworth>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Cotsworth>(t0, t0 + (diff as f64));
    }

    #[test]
    fn egyptian(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<EgyptianMonth, Egyptian>(t0, t1);
        consistent_order_ordinal::<Egyptian>(t0, t1);
    }

    #[test]
    fn egyptian_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<EgyptianMonth, Egyptian>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Egyptian>(t0, t0 + (diff as f64));
    }

    #[test]
    fn ethiopic(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<EthiopicMonth, Ethiopic>(t0, t1);
        consistent_order_ordinal::<Ethiopic>(t0, t1);
    }

    #[test]
    fn ethiopic_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<EthiopicMonth, Ethiopic>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Ethiopic>(t0, t0 + (diff as f64));
    }

    #[test]
    fn french_rev_arith(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<FrenchRevMonth, FrenchRevArith<true>>(t0, t1);
        consistent_order::<FrenchRevMonth, FrenchRevArith<false>>(t0, t1);
        consistent_order_ordinal::<FrenchRevArith<true>>(t0, t1);
        consistent_order_ordinal::<FrenchRevArith<false>>(t0, t1);
    }

    #[test]
    fn french_rev_arith_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<FrenchRevMonth, FrenchRevArith<true>>(t0, t0 + (diff as f64));
        consistent_order::<FrenchRevMonth, FrenchRevArith<false>>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<FrenchRevArith<true>>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<FrenchRevArith<false>>(t0, t0 + (diff as f64));
    }

    #[test]
    fn gregorian(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<GregorianMonth, Gregorian>(t0, t1);
        consistent_order_ordinal::<Gregorian>(t0, t1);
    }

    #[test]
    fn gregorian_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<GregorianMonth, Gregorian>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Gregorian>(t0, t0 + (diff as f64));
    }

    #[test]
    fn holocene(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<HoloceneMonth, Holocene>(t0, t1);
        consistent_order_ordinal::<Holocene>(t0, t1);
    }

    #[test]
    fn holocene_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<HoloceneMonth, Holocene>(t0, t0 + (diff as f64));
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
        consistent_order::<JulianMonth, Julian>(t0, t1);
        consistent_order_ordinal::<Julian>(t0, t1);
    }

    #[test]
    fn julian_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<JulianMonth, Julian>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Julian>(t0, t0 + (diff as f64));
    }

    #[test]
    fn positivist(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order::<PositivistMonth, Positivist>(t0, t1);
        consistent_order_ordinal::<Positivist>(t0, t1);
    }

    #[test]
    fn positivist_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<PositivistMonth, Positivist>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Positivist>(t0, t0 + (diff as f64));
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
        consistent_order::<SymmetryMonth, Symmetry454>(t0, t1);
        consistent_order::<SymmetryMonth, Symmetry010>(t0, t1);
        consistent_order::<SymmetryMonth, Symmetry454Solstice>(t0, t1);
        consistent_order::<SymmetryMonth, Symmetry010Solstice>(t0, t1);
        consistent_order_ordinal::<Symmetry454>(t0, t1);
        consistent_order_ordinal::<Symmetry010>(t0, t1);
        consistent_order_ordinal::<Symmetry454Solstice>(t0, t1);
        consistent_order_ordinal::<Symmetry010Solstice>(t0, t1);
    }

    #[test]
    fn symmetry_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order::<SymmetryMonth, Symmetry454>(t0, t0 + (diff as f64));
        consistent_order::<SymmetryMonth, Symmetry010>(t0, t0 + (diff as f64));
        consistent_order::<SymmetryMonth, Symmetry454Solstice>(t0, t0 + (diff as f64));
        consistent_order::<SymmetryMonth, Symmetry010Solstice>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Symmetry454>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Symmetry010>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Symmetry454Solstice>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<Symmetry010Solstice>(t0, t0 + (diff as f64));
    }

    #[test]
    fn tranquility(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
        consistent_order_basic::<TranquilityMoment>(t0, t1);
        consistent_order_ordinal::<TranquilityMoment>(t0, t1);
    }

    #[test]
    fn tranquility_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
        consistent_order_basic::<TranquilityMoment>(t0, t0 + (diff as f64));
        consistent_order_ordinal::<TranquilityMoment>(t0, t0 + (diff as f64));
    }
}

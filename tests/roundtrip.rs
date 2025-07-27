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
use radnelac::calendar::ToFromOrdinalDate;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::ISO;
use radnelac::day_count::BoundedDayCount;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::JulianDay;
use radnelac::day_count::ModifiedJulianDay;
use radnelac::day_count::RataDie;
use radnelac::day_count::ToFixed;
use radnelac::day_count::UnixMoment;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use std::fmt::Debug;

fn roundtrip_inner<T: FromFixed + ToFixed + PartialEq + Debug>(f0: Fixed) {
    let d0 = T::from_fixed(f0);
    let f1 = d0.to_fixed();
    let d1 = T::from_fixed(f1);
    assert_eq!(d0, d1, "f0 = {:?}; f1 = {:?}", f0, f1);
    assert!(f0.same_second(f1), "d0 = {:?}; d1 = {:?}", d0, d1);
}

fn roundtrip<T: FromFixed + ToFixed + PartialEq + Debug>(t: f64) {
    let f0 = Fixed::new(t).to_day();
    roundtrip_inner::<T>(f0);
}

fn roundtrip_moment<T: FromFixed + ToFixed + PartialEq + Debug>(t: f64) {
    let f0 = Fixed::new(t);
    roundtrip_inner::<T>(f0);
}

fn roundtrip_ordinal<T: FromFixed + PartialEq + Debug + ToFromOrdinalDate>(t: f64) {
    let f = Fixed::new(t).to_day();
    let d0 = T::from_fixed(f);
    let ord = d0.to_ordinal();
    let d1 = T::try_from_ordinal(ord).unwrap();
    assert_eq!(d1, d0);
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Armenian>(t);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Coptic>(t);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Cotsworth>(t);
        roundtrip_ordinal::<Cotsworth>(t);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Egyptian>(t);
        roundtrip_ordinal::<Egyptian>(t);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Ethiopic>(t);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<FrenchRevArith<true>>(t);
        roundtrip::<FrenchRevArith<false>>(t);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Gregorian>(t);
        roundtrip_ordinal::<Gregorian>(t);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Holocene>(t);
    }

    #[test]
    fn iso(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<ISO>(t);
    }

    #[test]
    fn julian(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Julian>(t);
    }

    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Positivist>(t);
        roundtrip_ordinal::<Positivist>(t);
    }

    #[test]
    fn roman(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Roman>(t);
    }

    #[test]
    fn symmetry(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Symmetry454>(t);
        roundtrip::<Symmetry010>(t);
        roundtrip::<Symmetry454Solstice>(t);
        roundtrip::<Symmetry010Solstice>(t);
        roundtrip_ordinal::<Symmetry454>(t);
        roundtrip_ordinal::<Symmetry010>(t);
        roundtrip_ordinal::<Symmetry454Solstice>(t);
        roundtrip_ordinal::<Symmetry010Solstice>(t);
    }

    #[test]
    fn tranquility(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<TranquilityMoment>(t);
        roundtrip_ordinal::<TranquilityMoment>(t);
    }

    #[test]
    fn unix(t in FIXED_MIN..FIXED_MAX) {
        roundtrip_moment::<UnixMoment>(t);
    }

    #[test]
    fn jd(t in FIXED_MIN..FIXED_MAX) {
        roundtrip_moment::<JulianDay>(t);
    }

    #[test]
    fn mjd(t in FIXED_MIN..FIXED_MAX) {
        roundtrip_moment::<ModifiedJulianDay>(t);
    }

    #[test]
    fn rd(t in FIXED_MIN..FIXED_MAX) {
        roundtrip_moment::<RataDie>(t);
    }
}

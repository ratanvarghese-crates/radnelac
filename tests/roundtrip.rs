// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
use radnelac::day_cycle::Akan;
use radnelac::day_cycle::AkanPrefix;
use radnelac::day_cycle::AkanStem;
use radnelac::day_cycle::BoundedCycle;
use radnelac::day_cycle::Weekday;
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

fn roundtrip_cycle<const N: u8, const M: u8, T: BoundedCycle<N, M>>(x: i64) {
    let w = T::from_i64(x).unwrap();
    let y = w.to_i64().unwrap();
    assert_eq!(x, y);

    let xu = x as u64;
    let wu = T::from_u64(xu).unwrap();
    let yu = wu.to_u64().unwrap();
    assert_eq!(xu, yu);
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Armenian>(t);
        roundtrip_ordinal::<Armenian>(t);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Coptic>(t);
        roundtrip_ordinal::<Coptic>(t);
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
        roundtrip_ordinal::<Ethiopic>(t);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<FrenchRevArith<true>>(t);
        roundtrip::<FrenchRevArith<false>>(t);
        roundtrip_ordinal::<FrenchRevArith<true>>(t);
        roundtrip_ordinal::<FrenchRevArith<false>>(t);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Gregorian>(t);
        roundtrip_ordinal::<Gregorian>(t);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Holocene>(t);
        roundtrip_ordinal::<Holocene>(t);
    }

    #[test]
    fn iso(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<ISO>(t);
    }

    #[test]
    fn julian(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Julian>(t);
        roundtrip_ordinal::<Julian>(t);
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

    #[test]
    fn week(x in 0..6) {
        roundtrip_cycle::<7, 0, Weekday>(x as i64);
    }

    #[test]
    fn akan(x in 1..42) {
        roundtrip_cycle::<42, 1, Akan>(x as i64);
    }

    #[test]
    fn akan_prefix(x in 1..6) {
        roundtrip_cycle::<6, 1, AkanPrefix>(x as i64);
    }

    #[test]
    fn akan_stem(x in 1..7) {
        roundtrip_cycle::<7, 1, AkanStem>(x as i64);
    }
}

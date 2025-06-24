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
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::TranquilityMoment;
use radnelac::bound::BoundedDayCount;
use radnelac::date::PerennialWithComplementaryDay;
use radnelac::date::ToFromCommonDate;
use radnelac::day_count::Epoch;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;

fn one_more_day<T: ToFromCommonDate + FromFixed>(t: f64) {
    let f0 = Fixed::new(t);
    let f1 = Fixed::new(t + 1.0);
    let d0 = T::from_fixed(f0).to_common_date();
    let d1 = T::from_fixed(f1).to_common_date();
    if d0.year != d1.year {
        assert_eq!(d1.year, d0.year + 1);
        assert_eq!(d1.month, 1);
        assert_eq!(d1.day, 1);
    } else if d0.month != d1.month {
        assert_eq!(d1.year, d0.year);
        assert_eq!(d1.month, d0.month + 1);
        assert_eq!(d1.day, 1);
    } else if d0.day != d1.day {
        assert_eq!(d1.year, d0.year);
        assert_eq!(d1.month, d0.month);
        assert_eq!(d1.day, d0.day + 1);
    } else {
        panic!("Added one day but the dates are equal");
    }
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Armenian>(t);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Coptic>(t);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Cotsworth>(t);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Egyptian>(t);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Ethiopic>(t);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<FrenchRevArith<true>>(t);
        one_more_day::<FrenchRevArith<false>>(t);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Gregorian>(t);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Holocene>(t);
    }

    #[test]
    fn julian_ad(t in FIXED_MIN..-367.0) {
        //Avoiding year 0
        one_more_day::<Julian>(t);
    }

    #[test]
    fn julian_bc(t in 367.0..FIXED_MAX) {
        //Avoiding year 0
        one_more_day::<Julian>(t);
    }

    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Positivist>(t);
    }

    #[test]
    fn symmetry(t in FIXED_MIN..FIXED_MAX) {
        one_more_day::<Symmetry454>(t);
        one_more_day::<Symmetry010>(t);
        one_more_day::<Symmetry454Solstice>(t);
        one_more_day::<Symmetry010Solstice>(t);
    }

    #[test]
    fn tranquility_bt(dt in FIXED_MIN..-367.0) {
        //Avoiding year 0 and complementary days
        let t = TranquilityMoment::epoch().get() + dt;
        let tq0 = TranquilityMoment::from_fixed(Fixed::new(t));
        let tq1 = TranquilityMoment::from_fixed(Fixed::new(t + 1.0));
        prop_assume!(tq0.complementary().is_none());
        prop_assume!(tq1.complementary().is_none());
        one_more_day::<TranquilityMoment>(t);
    }

    #[test]
    fn tranquility_at(dt in 367.0..FIXED_MAX) {
        //Avoiding year 0 and complementary days
        let t = TranquilityMoment::epoch().get() + dt;
        let tq0 = TranquilityMoment::from_fixed(Fixed::new(t));
        let tq1 = TranquilityMoment::from_fixed(Fixed::new(t + 1.0));
        prop_assume!(tq0.complementary().is_none());
        prop_assume!(tq1.complementary().is_none());
        one_more_day::<TranquilityMoment>(t);
    }

}

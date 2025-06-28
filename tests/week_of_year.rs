use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use proptest::proptest;
use radnelac::bound::BoundedDayCount;
use radnelac::calendar::Armenian;
use radnelac::calendar::Coptic;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::CotsworthComplementaryDay;
use radnelac::calendar::CotsworthMonth;
use radnelac::calendar::Egyptian;
use radnelac::calendar::Ethiopic;
use radnelac::calendar::FrenchRevArith;
use radnelac::calendar::FrenchRevMonth;
use radnelac::calendar::FrenchRevWeekday;
use radnelac::calendar::Gregorian;
use radnelac::calendar::Holocene;
use radnelac::calendar::Julian;
use radnelac::calendar::Positivist;
use radnelac::calendar::PositivistComplementaryDay;
use radnelac::calendar::PositivistMonth;
use radnelac::calendar::Sansculottide;
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::TranquilityComplementaryDay;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::calendar::ISO;
use radnelac::date::CommonWeekOfYear;
use radnelac::date::ComplementaryWeekOfYear;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use radnelac::day_cycle::Weekday;

fn between_2_weeks(allow_same_week: bool, max: u8, w0: u8, w1: u8, w2: u8) {
    assert!(w0 >= 1 && w0 <= max);
    assert!(w1 >= 1 && w1 <= max);
    assert!(w2 >= 1 && w2 <= max);
    match (w0, w1, w2) {
        (w0, 1, 1) => assert!(w0 == max || w0 == (max - 1)),
        (w0, w1, 1) => {
            assert!(w1 == w0 || w1 == (w0 + 1));
            assert!(w1 == max || w1 == (max - 1));
        }
        (w0, w1, w2) => {
            if allow_same_week {
                assert!(w2 == w0 || w2 == (w0 + 1))
            } else {
                assert_eq!(w2, w0 + 1);
            }
            assert!(w2 == w1 || w2 == (w1 + 1));
        }
    };
}

fn within_1_week(max: u8, w0: u8, w1: u8) {
    assert!(w0 >= 1 && w0 <= max);
    assert!(w1 >= 1 && w1 <= max);
    assert!(w1 == w0 || w1 == (w0 + 1)); //Assumes weekdays at start of year
}

fn common_week_of_year<T: FromFixed + CommonWeekOfYear>(max: u8, t: f64, dt: u8) {
    let f0 = Fixed::new(t);
    let f1 = Fixed::new(t + (dt as f64));
    let f2 = Fixed::new(t + 7.0);
    let w0 = T::from_fixed(f0).week_of_year();
    let w1 = T::from_fixed(f1).week_of_year();
    let w2 = T::from_fixed(f2).week_of_year();
    between_2_weeks(false, max, w0, w1, w2);
}

fn complementary_week_of_year<S, T, U, V>(allow_same_week: bool, max: u8, t: f64, dt: u8)
where
    S: ToPrimitive + FromPrimitive,
    T: ToPrimitive + FromPrimitive,
    U: ToPrimitive + FromPrimitive,
    V: ComplementaryWeekOfYear<S, T, U> + FromFixed,
{
    let f0 = Fixed::new(t);
    let f1 = Fixed::new(t + (dt as f64));
    let f2 = Fixed::new(t + (V::days_per_week() as f64));
    let w0_opt = V::from_fixed(f0).try_week_of_year();
    let w1_opt = V::from_fixed(f1).try_week_of_year();
    let w2_opt = V::from_fixed(f2).try_week_of_year();
    match (w0_opt, w1_opt, w2_opt) {
        (Some(w0), Some(w1), Some(w2)) => between_2_weeks(allow_same_week, max, w0, w1, w2),
        (Some(w0), None, Some(w2)) => between_2_weeks(allow_same_week, max, w0, w0, w2),
        (Some(w0), Some(w1), None) => within_1_week(max, w0, w1),
        (None, Some(w1), Some(w2)) => within_1_week(max, w1, w2),
        (Some(w0), None, None) => assert!(1 <= w0 && w0 <= max),
        (None, Some(w1), None) => assert!(1 <= w1 && w1 <= max),
        (None, None, Some(w2)) => assert!(1 <= w2 && w2 <= max),
        (None, None, None) => panic!("Too many complementary days."),
    };
    if w0_opt.is_some() {
        let m0 = V::from_fixed(f0).try_month().unwrap().to_i64().unwrap();
        let wpm = V::weeks_per_month() as i64;
        assert!((w0_opt.unwrap() as i64) > ((m0 - 1) * wpm));
        assert!((w0_opt.unwrap() as i64) <= ((m0 - 0) * wpm));
    }
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<Armenian>(53, t, dt as u8);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<Coptic>(53, t, dt as u8);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        complementary_week_of_year::<CotsworthMonth, CotsworthComplementaryDay, Weekday, Cotsworth>(false, 53, t, dt as u8);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<Egyptian>(53, t, dt as u8);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<Ethiopic>(53, t, dt as u8);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX, dt in 1..9) {
        complementary_week_of_year::<FrenchRevMonth, Sansculottide, FrenchRevWeekday, FrenchRevArith<true>>(false, 36, t, dt as u8);
        complementary_week_of_year::<FrenchRevMonth, Sansculottide, FrenchRevWeekday, FrenchRevArith<false>>(false, 36, t, dt as u8);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<Gregorian>(53, t, dt as u8);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<Holocene>(53, t, dt as u8);
    }

    #[test]
    fn iso(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        let f0 = Fixed::new(t);
        let f1 = Fixed::new(t + (dt as f64));
        let f2 = Fixed::new(t + 7.0);
        let w0 = ISO::from_fixed(f0).week().get();
        let w1 = ISO::from_fixed(f1).week().get();
        let w2 = ISO::from_fixed(f2).week().get();
        between_2_weeks(false, 53, w0, w1, w2);
    }

    #[test]
    fn julian(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<Julian>(53, t, dt as u8);
    }

    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        complementary_week_of_year::<PositivistMonth, PositivistComplementaryDay, Weekday, Positivist>(false, 53, t, dt as u8);
    }

    #[test]
    fn symmetry(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<Symmetry010>(53, t, dt as u8);
        common_week_of_year::<Symmetry454>(53, t, dt as u8);
        common_week_of_year::<Symmetry010Solstice>(53, t, dt as u8);
        common_week_of_year::<Symmetry454Solstice>(53, t, dt as u8);
    }

    #[test]
    fn tranquility(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        complementary_week_of_year::<TranquilityMonth, TranquilityComplementaryDay, Weekday, TranquilityMoment>(true, 53, t, dt as u8);
    }

}

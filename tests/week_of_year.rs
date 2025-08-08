use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use proptest::proptest;
use radnelac::calendar::*;
use radnelac::day_count::BoundedDayCount;
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

fn common_week_of_year<S: ToPrimitive + FromPrimitive, T: FromFixed + CommonWeekOfYear<S>>(
    max: u8,
    t: f64,
    dt: u8,
) {
    let f0 = Fixed::new(t);
    let f1 = Fixed::new(t + (dt as f64));
    let f2 = Fixed::new(t + 7.0);
    let w0 = T::from_fixed(f0).week_of_year();
    let w1 = T::from_fixed(f1).week_of_year();
    let w2 = T::from_fixed(f2).week_of_year();
    between_2_weeks(false, max, w0, w1, w2);
}

fn complementary_week_of_year<S, U, V>(allow_same_week: bool, max: u8, t: f64, dt: u8)
where
    S: ToPrimitive + FromPrimitive,
    U: ToPrimitive + FromPrimitive,
    V: Perennial<S, U> + FromFixed,
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
        common_week_of_year::<ArmenianMonth, Armenian>(53, t, dt as u8);
        common_week_of_year::<ArmenianMonth, ArmenianMoment>(53, t, dt as u8);
    }

    #[test]
    fn coptic(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<CopticMonth, Coptic>(53, t, dt as u8);
        common_week_of_year::<CopticMonth, CopticMoment>(53, t, dt as u8);
    }

    #[test]
    fn cotsworth(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        complementary_week_of_year::<CotsworthMonth, Weekday, Cotsworth>(false, 53, t, dt as u8);
        complementary_week_of_year::<CotsworthMonth, Weekday, CotsworthMoment>(false, 53, t, dt as u8);
    }

    #[test]
    fn egyptian(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<EgyptianMonth, Egyptian>(53, t, dt as u8);
        common_week_of_year::<EgyptianMonth, EgyptianMoment>(53, t, dt as u8);
    }

    #[test]
    fn ethiopic(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<EthiopicMonth, Ethiopic>(53, t, dt as u8);
        common_week_of_year::<EthiopicMonth, EthiopicMoment>(53, t, dt as u8);
    }

    #[test]
    fn french_rev_arith(t in FIXED_MIN..FIXED_MAX, dt in 1..9) {
        complementary_week_of_year::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArith<true>>(false, 36, t, dt as u8);
        complementary_week_of_year::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArith<false>>(false, 36, t, dt as u8);
        complementary_week_of_year::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArithMoment<true>>(false, 36, t, dt as u8);
        complementary_week_of_year::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArithMoment<false>>(false, 36, t, dt as u8);
    }

    #[test]
    fn gregorian(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<GregorianMonth, Gregorian>(53, t, dt as u8);
        common_week_of_year::<GregorianMonth, GregorianMoment>(53, t, dt as u8);
    }

    #[test]
    fn holocene(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<HoloceneMonth, Holocene>(53, t, dt as u8);
        common_week_of_year::<HoloceneMonth, HoloceneMoment>(53, t, dt as u8);
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
        let wm0 = ISOMoment::from_fixed(f0).week().get();
        let wm1 = ISOMoment::from_fixed(f1).week().get();
        let wm2 = ISOMoment::from_fixed(f2).week().get();
        between_2_weeks(false, 53, wm0, wm1, wm2);

    }

    #[test]
    fn julian(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<JulianMonth, Julian>(53, t, dt as u8);
        common_week_of_year::<JulianMonth, JulianMoment>(53, t, dt as u8);
    }

    #[test]
    fn positivist(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        complementary_week_of_year::<PositivistMonth, Weekday, Positivist>(false, 53, t, dt as u8);
        complementary_week_of_year::<PositivistMonth, Weekday, PositivistMoment>(false, 53, t, dt as u8);
    }

    #[test]
    fn symmetry(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        common_week_of_year::<SymmetryMonth, Symmetry010>(53, t, dt as u8);
        common_week_of_year::<SymmetryMonth, Symmetry454>(53, t, dt as u8);
        common_week_of_year::<SymmetryMonth, Symmetry010Solstice>(53, t, dt as u8);
        common_week_of_year::<SymmetryMonth, Symmetry454Solstice>(53, t, dt as u8);
        common_week_of_year::<SymmetryMonth, Symmetry010Moment>(53, t, dt as u8);
        common_week_of_year::<SymmetryMonth, Symmetry454Moment>(53, t, dt as u8);
        common_week_of_year::<SymmetryMonth, Symmetry010SolsticeMoment>(53, t, dt as u8);
        common_week_of_year::<SymmetryMonth, Symmetry454SolsticeMoment>(53, t, dt as u8);
    }

    #[test]
    fn tranquility(t in FIXED_MIN..FIXED_MAX, dt in 1..6) {
        complementary_week_of_year::<TranquilityMonth, Weekday, Tranquility>(true, 53, t, dt as u8);
        complementary_week_of_year::<TranquilityMonth, Weekday, TranquilityMoment>(true, 53, t, dt as u8);
    }

}

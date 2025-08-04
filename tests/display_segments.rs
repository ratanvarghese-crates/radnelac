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
use radnelac::calendar::FrenchRevWeekday;
use radnelac::calendar::Gregorian;
use radnelac::calendar::GregorianMonth;
use radnelac::calendar::GuaranteedMonth;
use radnelac::calendar::HasIntercalaryDays;
use radnelac::calendar::Holocene;
use radnelac::calendar::HoloceneMonth;
use radnelac::calendar::Julian;
use radnelac::calendar::JulianMonth;
use radnelac::calendar::Perennial;
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
use radnelac::calendar::ISO;
use radnelac::day_count::BoundedDayCount;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_cycle::Weekday;

fn reasonable_blanks<U: ToString + FromFixed>(t0: f64) {
    let d = U::from_fixed(Fixed::new(t0));
    let s = d.to_string();
    assert!(s.len() > 0);
    assert!(s.find("  ").is_none());
    assert!(s.find('\t').is_none());
    assert!(s.find('\n').is_none());
}

fn compare_month<T, U>(t0: f64, t1: f64, m_idx: usize)
where
    T: FromPrimitive + ToPrimitive + PartialEq,
    U: GuaranteedMonth<T> + ToString + FromFixed,
{
    let a0 = U::from_fixed(Fixed::new(t0));
    let a1 = U::from_fixed(Fixed::new(t1));
    let s0 = a0.to_string();
    let s1 = a1.to_string();
    let v0: Vec<&str> = s0.split(' ').collect();
    let v1: Vec<&str> = s1.split(' ').collect();
    assert_eq!(a0.month() == a1.month(), v0[m_idx] == v1[m_idx]);
}

fn compare_possible_month<T, U>(t0: f64, t1: f64, m_idx: usize)
where
    T: FromPrimitive + ToPrimitive + PartialEq,
    U: ToFromCommonDate<T> + ToString + FromFixed,
{
    let a0 = U::from_fixed(Fixed::new(t0));
    let a1 = U::from_fixed(Fixed::new(t1));
    let s0 = a0.to_string();
    let s1 = a1.to_string();
    let v0: Vec<&str> = s0.split(' ').collect();
    let v1: Vec<&str> = s1.split(' ').collect();
    if a0.try_month().is_some() && a1.try_month().is_some() {
        assert_eq!(a0.try_month() == a1.try_month(), v0[m_idx] == v1[m_idx]);
    }
}

fn compare_common_weekday<U: FromFixed + ToString>(t0: f64, t1: f64, m_idx: usize) {
    let f0 = Fixed::new(t0);
    let f1 = Fixed::new(t1);
    let w0 = Weekday::from_fixed(f0);
    let w1 = Weekday::from_fixed(f1);
    let a0 = U::from_fixed(f0);
    let a1 = U::from_fixed(f1);
    let s0 = a0.to_string();
    let s1 = a1.to_string();
    let v0: Vec<&str> = s0.split(' ').collect();
    let v1: Vec<&str> = s1.split(' ').collect();
    assert_eq!(
        w0 == w1,
        v0[m_idx] == v1[m_idx],
        "{:?} {:?} {:?} {:?}",
        f0,
        f1,
        s0,
        s1
    );
}

fn compare_perennial_weekday<S, T, U>(t0: f64, t1: f64, m_idx: usize)
where
    S: ToPrimitive + FromPrimitive + PartialEq,
    T: ToPrimitive + FromPrimitive + PartialEq,
    U: Perennial<S, T> + FromFixed + ToString,
{
    let a0 = U::from_fixed(Fixed::new(t0));
    let a1 = U::from_fixed(Fixed::new(t1));
    let s0 = a0.to_string();
    let s1 = a1.to_string();
    let v0: Vec<&str> = s0.split(' ').collect();
    let v1: Vec<&str> = s1.split(' ').collect();
    if a0.weekday().is_some() && a1.weekday().is_some() {
        assert_eq!(a0.weekday() == a1.weekday(), v0[m_idx] == v1[m_idx]);
    }
}

#[cfg(feature = "display")]
proptest! {
    #[test]
    fn armenian_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Armenian>(t0);
    }

    #[test]
    fn armenian_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<ArmenianMonth, Armenian>(t0, t1, 1);
    }

    #[test]
    fn armenian_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Armenian>(t0, t1, 0);
    }

    #[test]
    fn coptic_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Coptic>(t0);
    }

    #[test]
    fn coptic_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<CopticMonth, Coptic>(t0, t1, 1);
    }

    #[test]
    fn coptic_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Armenian>(t0, t1, 0);
    }

    #[test]
    fn cotsworth_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Cotsworth>(t0);
    }

    #[test]
    fn cotsworth_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<CotsworthMonth, Cotsworth>(t0, t1, 1);
    }

    #[test]
    fn cotsworth_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_perennial_weekday::<CotsworthMonth, Weekday, Cotsworth>(t0, t1, 0);
    }

    #[test]
    fn egyptian_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Egyptian>(t0);
    }

    #[test]
    fn egyptian_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<EgyptianMonth, Egyptian>(t0, t1, 1);
    }

    #[test]
    fn egyptian_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        let e0 = Egyptian::from_fixed(Fixed::new(t0));
        let e1 = Egyptian::from_fixed(Fixed::new(t1));
        if e0.complementary().is_none() && e1.complementary().is_none() {
            compare_common_weekday::<Egyptian>(t0, t1, 0);
        }
    }

    #[test]
    fn ethiopic_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Ethiopic>(t0);
    }

    #[test]
    fn ethiopic_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<EthiopicMonth, Ethiopic>(t0, t1, 1);
    }

    #[test]
    fn ethiopic_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Ethiopic>(t0, t1, 0);
    }

    #[test]
    fn french_rev_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<FrenchRevArith<false>>(t0);
        reasonable_blanks::<FrenchRevArith<true>>(t0);
    }

    #[test]
    fn french_rev_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<FrenchRevMonth, FrenchRevArith<false>>(t0, t1, 1);
        compare_possible_month::<FrenchRevMonth, FrenchRevArith<true>>(t0, t1, 1);
    }

    #[test]
    fn french_rev_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_perennial_weekday::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArith<false>>(t0, t1, 0);
        compare_perennial_weekday::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArith<true>>(t0, t1, 0);
    }

    #[test]
    fn gregorian_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Gregorian>(t0);
    }

    #[test]
    fn gregorian_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<GregorianMonth, Gregorian>(t0, t1, 1);
    }

    #[test]
    fn gregorian_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Gregorian>(t0, t1, 0);
    }

    #[test]
    fn holocene_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Holocene>(t0);
    }

    #[test]
    fn holocene_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<HoloceneMonth, Holocene>(t0, t1, 1);
    }

    #[test]
    fn holocene_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Holocene>(t0, t1, 0);
    }

    #[test]
    fn iso_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<ISO>(t0);
    }

    #[test]
    fn julian_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Julian>(t0);
    }

    #[test]
    fn julian_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<JulianMonth, Julian>(t0, t1, 1);
    }

    #[test]
    fn julian_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Julian>(t0, t1, 0);
    }

    #[test]
    fn positivist_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Positivist>(t0);
    }

    #[test]
    fn positivist_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<PositivistMonth, Positivist>(t0, t1, 1);
    }

    #[test]
    fn positivist_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_perennial_weekday::<PositivistMonth, Weekday, Positivist>(t0, t1, 0);
    }

    #[test]
    fn symmetry_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Symmetry010>(t0);
        reasonable_blanks::<Symmetry454>(t0);
        reasonable_blanks::<Symmetry010Solstice>(t0);
        reasonable_blanks::<Symmetry454Solstice>(t0);
    }

    #[test]
    fn symmetry_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<SymmetryMonth, Symmetry010>(t0, t1, 1);
        compare_month::<SymmetryMonth, Symmetry454>(t0, t1, 1);
        compare_month::<SymmetryMonth, Symmetry010Solstice>(t0, t1, 1);
        compare_month::<SymmetryMonth, Symmetry454Solstice>(t0, t1, 1);
    }

    #[test]
    fn symmetry_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Symmetry010>(t0, t1, 0);
        compare_common_weekday::<Symmetry454>(t0, t1, 0);
        compare_common_weekday::<Symmetry010Solstice>(t0, t1, 0);
        compare_common_weekday::<Symmetry454Solstice>(t0, t1, 0);
    }

    #[test]
    fn tranquility_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<TranquilityMoment>(t0);
    }

    #[test]
    fn tranquility_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<TranquilityMonth, TranquilityMoment>(t0, t1, 1);
    }

    #[test]
    fn tranquility_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_perennial_weekday::<TranquilityMonth, Weekday, TranquilityMoment>(t0, t1, 0);
    }
}

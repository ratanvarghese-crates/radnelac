#[cfg(feature = "display")]
use num_traits::FromPrimitive;
#[cfg(feature = "display")]
use num_traits::ToPrimitive;
#[cfg(feature = "display")]
use proptest::proptest;
#[cfg(feature = "display")]
use radnelac::calendar::Armenian;
#[cfg(feature = "display")]
use radnelac::calendar::ArmenianMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Coptic;
#[cfg(feature = "display")]
use radnelac::calendar::CopticMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Cotsworth;
#[cfg(feature = "display")]
use radnelac::calendar::CotsworthMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Egyptian;
#[cfg(feature = "display")]
use radnelac::calendar::EgyptianMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Ethiopic;
#[cfg(feature = "display")]
use radnelac::calendar::EthiopicMonth;
#[cfg(feature = "display")]
use radnelac::calendar::FrenchRevArith;
#[cfg(feature = "display")]
use radnelac::calendar::FrenchRevMonth;
#[cfg(feature = "display")]
use radnelac::calendar::FrenchRevWeekday;
#[cfg(feature = "display")]
use radnelac::calendar::Gregorian;
#[cfg(feature = "display")]
use radnelac::calendar::GregorianMonth;
#[cfg(feature = "display")]
use radnelac::calendar::GuaranteedMonth;
#[cfg(feature = "display")]
use radnelac::calendar::HasIntercalaryDays;
#[cfg(feature = "display")]
use radnelac::calendar::Holocene;
#[cfg(feature = "display")]
use radnelac::calendar::HoloceneMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Julian;
#[cfg(feature = "display")]
use radnelac::calendar::JulianMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Perennial;
#[cfg(feature = "display")]
use radnelac::calendar::Positivist;
#[cfg(feature = "display")]
use radnelac::calendar::PositivistMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Symmetry010;
#[cfg(feature = "display")]
use radnelac::calendar::Symmetry010Solstice;
#[cfg(feature = "display")]
use radnelac::calendar::Symmetry454;
#[cfg(feature = "display")]
use radnelac::calendar::Symmetry454Solstice;
#[cfg(feature = "display")]
use radnelac::calendar::SymmetryMonth;
#[cfg(feature = "display")]
use radnelac::calendar::ToFromCommonDate;
#[cfg(feature = "display")]
use radnelac::calendar::TranquilityMoment;
#[cfg(feature = "display")]
use radnelac::calendar::TranquilityMonth;
#[cfg(feature = "display")]
use radnelac::calendar::ISO;
#[cfg(feature = "display")]
use radnelac::clock::ClockTime;
#[cfg(feature = "display")]
use radnelac::clock::TimeOfDay;
#[cfg(feature = "display")]
use radnelac::day_count::BoundedDayCount;
#[cfg(feature = "display")]
use radnelac::day_count::Epoch;
#[cfg(feature = "display")]
use radnelac::day_count::Fixed;
#[cfg(feature = "display")]
use radnelac::day_count::FromFixed;
#[cfg(feature = "display")]
use radnelac::day_count::FIXED_MAX;
#[cfg(feature = "display")]
use radnelac::day_cycle::Weekday;
#[cfg(feature = "display")]
use radnelac::display::Language;
#[cfg(feature = "display")]
use radnelac::display::PresetDisplay;
#[cfg(feature = "display")]
use radnelac::display::PresetFormat;
#[cfg(feature = "display")]
use radnelac::display::HHMMSS_COLON;
#[cfg(feature = "display")]
use radnelac::display::HHMM_COLON_AMPM;
#[cfg(feature = "display")]
use radnelac::display::LONG_DATE_ERA_ABBR;

#[cfg(feature = "display")]
fn reasonable_blanks<U: ToString + FromFixed>(t0: f64) {
    let d = U::from_fixed(Fixed::new(t0));
    let s = d.to_string();
    assert!(s.len() > 0);
    assert!(s.find("  ").is_none());
    assert!(s.find('\t').is_none());
    assert!(s.find('\n').is_none());
}

#[cfg(feature = "display")]
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

#[cfg(feature = "display")]
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

#[cfg(feature = "display")]
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

#[cfg(feature = "display")]
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
fn compare_era_abbrev<U>(t0: f64, t1: f64)
where
    U: PresetDisplay + Epoch + FromFixed,
{
    let f0 = Fixed::new(t0);
    let f1 = Fixed::new(t1);
    let a0 = U::from_fixed(f0);
    let a1 = U::from_fixed(f1);
    let e = U::epoch();
    let s0 = a0.preset_str(Language::EN, LONG_DATE_ERA_ABBR);
    let s1 = a1.preset_str(Language::EN, LONG_DATE_ERA_ABBR);
    let v0: Vec<&str> = s0.split(' ').collect();
    let v1: Vec<&str> = s1.split(' ').collect();
    assert_eq!(
        (f0 > e) == (f1 > e),
        v0[v0.len() - 1] == v1[v1.len() - 1],
        "{:?} {:?} {:?} {:?} {:?}",
        f0,
        f1,
        e,
        s0,
        s1
    );
}

#[cfg(feature = "display")]
fn clock_segments(preset: PresetFormat, c: ClockTime, sep: &str, idx: usize, expected: &str) {
    let t = TimeOfDay::new_from_clock(c);
    let s = t.preset_str(Language::EN, preset);
    let v: Vec<&str> = s.split(sep).collect();
    assert_eq!(v[idx], expected);
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
    fn armenian_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Armenian>(t0, t1);
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
    fn coptic_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Coptic>(t0, t1);
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
    fn cotsworth_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Cotsworth>(t0, t1);
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
    fn egyptian_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Egyptian>(t0, t1);
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
    fn ethiopic_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Ethiopic>(t0, t1);
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
    fn french_rev_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<FrenchRevArith<false>>(t0, t1);
        compare_era_abbrev::<FrenchRevArith<true>>(t0, t1);
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
    fn gregorian_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Gregorian>(t0, t1);
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
    fn holocene_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Holocene>(t0, t1);
    }

    #[test]
    fn iso_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<ISO>(t0);
    }

    #[test]
    fn iso_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<ISO>(t0, t1);
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
    fn julian_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Julian>(t0, t1);
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
    fn positivist_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Positivist>(t0, t1);
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
    fn symmetry_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Symmetry010>(t0, t1);
        compare_era_abbrev::<Symmetry454>(t0, t1);
        compare_era_abbrev::<Symmetry010Solstice>(t0, t1);
        compare_era_abbrev::<Symmetry454Solstice>(t0, t1);
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

    #[test]
    fn tranquility_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<TranquilityMoment>(t0, t1);
    }

    #[test]
    fn clock_hh_mm_ss_zero_sec(h in 0..23, m in 0..60) {
        let c = ClockTime{ hours: h as u8, minutes: m as u8, seconds: 0.0 };
        clock_segments(HHMMSS_COLON, c, ":", 2, "00");
    }

    #[test]
    fn clock_hh_mm_ss_zero_min(h in 0..23, s in 0.0..59.0) {
        let c = ClockTime{ hours: h as u8, minutes: 0, seconds: s as f32 };
        clock_segments(HHMMSS_COLON, c, ":", 1, "00");
    }

    #[test]
    fn clock_hh_mm_ss_zero_hr(m in 0..59, s in 0.0..59.0) {
        let c = ClockTime{ hours: 0, minutes: m as u8, seconds: s as f32};
        clock_segments(HHMMSS_COLON, c, ":", 0, "00");
    }

    #[test]
    fn clock_hh_mm_ss_default_str(h in 12..23, m in 0..59, s in 0.0..59.0) {
        let c = ClockTime{ hours: h as u8, minutes: m as u8, seconds: s as f32 };
        let t = TimeOfDay::new_from_clock(c);
        let ts0 = t.preset_str(Language::EN, HHMMSS_COLON);
        let ts1 = t.to_string();
        assert_eq!(ts0, ts1);
    }

    #[test]
    fn clock_am(h in 0..11, m in 0..59, s in 0.0..59.0) {
        let c = ClockTime{ hours: h as u8, minutes: m as u8, seconds: s as f32 };
        clock_segments(HHMM_COLON_AMPM, c, " ", 1, "AM");
    }

    #[test]
    fn clock_pm(h in 12..23, m in 0..59, s in 0.0..59.0) {
        let c = ClockTime{ hours: h as u8, minutes: m as u8, seconds: s as f32 };
        clock_segments(HHMM_COLON_AMPM, c, " ", 1, "PM");
    }
}

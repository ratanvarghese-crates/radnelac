// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "display")]
mod display_logic {
    pub use num_traits::FromPrimitive;
    pub use num_traits::ToPrimitive;
    pub use proptest::proptest;
    pub use radnelac::calendar::*;
    pub use radnelac::clock::*;
    pub use radnelac::day_count::*;
    pub use radnelac::day_cycle::*;
    pub use radnelac::display::Language;
    pub use radnelac::display::PresetDisplay;
    pub use radnelac::display::PresetFormat;
    pub use radnelac::display::HHMMSS_COLON;
    pub use radnelac::display::HHMM_COLON_AMPM;
    pub use radnelac::display::LONG_DATE_ERA_ABBR;

    pub fn reasonable_blanks<U: ToString + FromFixed>(t0: f64) {
        let d = U::from_fixed(Fixed::new(t0));
        let s = d.to_string();
        assert!(s.len() > 0);
        assert!(s.find("  ").is_none(), "{}", s);
        assert!(s.find('\t').is_none());
        assert!(s.find('\n').is_none());
    }

    pub fn compare_month<T, U>(t0: f64, t1: f64, m_idx: usize)
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

    pub fn compare_possible_month<T, U>(t0: f64, t1: f64, m_idx: usize)
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

    pub fn compare_cotsworth_month<T, U>(t0: f64, t1: f64, m_idx: usize)
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
        let idx_diff0 = if a0.day() < 29 { 0 } else { 1 };
        let idx_diff1 = if a1.day() < 29 { 0 } else { 1 };
        if a0.try_month().is_some() && a1.try_month().is_some() {
            assert_eq!(
                a0.try_month() == a1.try_month(),
                v0[m_idx - idx_diff0] == v1[m_idx - idx_diff1]
            );
        }
    }

    pub fn compare_common_weekday<U: FromFixed + ToString>(t0: f64, t1: f64, m_idx: usize) {
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

    pub fn compare_perennial_weekday<S, T, U>(t0: f64, t1: f64, m_idx: usize)
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

    pub fn compare_era_abbrev<U>(t0: f64, t1: f64)
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

    pub fn clock_segments(
        preset: PresetFormat,
        c: ClockTime,
        sep: &str,
        idx: usize,
        expected: &str,
    ) {
        let t = TimeOfDay::try_from_clock(c).unwrap();
        let s = t.preset_str(Language::EN, preset);
        let v: Vec<&str> = s.split(sep).collect();
        assert_eq!(v[idx], expected);
    }
}

#[cfg(feature = "display")]
use display_logic::*;

#[cfg(feature = "display")]
proptest! {
    #[test]
    fn armenian_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Armenian>(t0);
        reasonable_blanks::<ArmenianMoment>(t0);
    }

    #[test]
    fn armenian_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<ArmenianMonth, Armenian>(t0, t1, 1);
        compare_possible_month::<ArmenianMonth, ArmenianMoment>(t0, t1, 2);
    }

    #[test]
    fn armenian_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Armenian>(t0, t1, 0);
        compare_common_weekday::<ArmenianMoment>(t0, t1, 1);
    }

    #[test]
    fn armenian_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Armenian>(t0, t1);
        compare_era_abbrev::<ArmenianMoment>(t0, t1);
    }

    #[test]
    fn coptic_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Coptic>(t0);
        reasonable_blanks::<CopticMoment>(t0);
    }

    #[test]
    fn coptic_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<CopticMonth, Coptic>(t0, t1, 1);
        compare_month::<CopticMonth, CopticMoment>(t0, t1, 2);
    }

    #[test]
    fn coptic_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Coptic>(t0, t1, 0);
        compare_common_weekday::<CopticMoment>(t0, t1, 1);
    }

    #[test]
    fn coptic_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Coptic>(t0, t1);
        compare_era_abbrev::<CopticMoment>(t0, t1);
    }

    #[test]
    fn cotsworth_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Cotsworth>(t0);
        reasonable_blanks::<CotsworthMoment>(t0);
    }

    #[test]
    fn cotsworth_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_cotsworth_month::<CotsworthMonth, Cotsworth>(t0, t1, 1);
        compare_cotsworth_month::<CotsworthMonth, CotsworthMoment>(t0, t1, 2);
    }

    #[test]
    fn cotsworth_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_perennial_weekday::<CotsworthMonth, Weekday, Cotsworth>(t0, t1, 0);
        compare_perennial_weekday::<CotsworthMonth, Weekday, CotsworthMoment>(t0, t1, 1);
    }

    #[test]
    fn cotsworth_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Cotsworth>(t0, t1);
        compare_era_abbrev::<CotsworthMoment>(t0, t1);
    }

    #[test]
    fn egyptian_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Egyptian>(t0);
        reasonable_blanks::<EgyptianMoment>(t0);
    }

    #[test]
    fn egyptian_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<EgyptianMonth, Egyptian>(t0, t1, 1);
        compare_possible_month::<EgyptianMonth, EgyptianMoment>(t0, t1, 2);
    }

    #[test]
    fn egyptian_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        let e0 = Egyptian::from_fixed(Fixed::new(t0));
        let e1 = Egyptian::from_fixed(Fixed::new(t1));
        if e0.complementary().is_none() && e1.complementary().is_none() {
            compare_common_weekday::<Egyptian>(t0, t1, 0);
            compare_common_weekday::<EgyptianMoment>(t0, t1, 1);
        }
    }

    #[test]
    fn egyptian_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Egyptian>(t0, t1);
        compare_era_abbrev::<EgyptianMoment>(t0, t1);
    }

    #[test]
    fn ethiopic_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Ethiopic>(t0);
        reasonable_blanks::<EthiopicMoment>(t0);
    }

    #[test]
    fn ethiopic_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<EthiopicMonth, Ethiopic>(t0, t1, 1);
        compare_month::<EthiopicMonth, EthiopicMoment>(t0, t1, 2);
    }

    #[test]
    fn ethiopic_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Ethiopic>(t0, t1, 0);
        compare_common_weekday::<EthiopicMoment>(t0, t1, 1);
    }

    #[test]
    fn ethiopic_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Ethiopic>(t0, t1);
        compare_era_abbrev::<EthiopicMoment>(t0, t1);
    }

    #[test]
    fn french_rev_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<FrenchRevArith<false>>(t0);
        reasonable_blanks::<FrenchRevArith<true>>(t0);
        reasonable_blanks::<FrenchRevArithMoment<false>>(t0);
        reasonable_blanks::<FrenchRevArithMoment<true>>(t0);
    }

    #[test]
    fn french_rev_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<FrenchRevMonth, FrenchRevArith<false>>(t0, t1, 1);
        compare_possible_month::<FrenchRevMonth, FrenchRevArith<true>>(t0, t1, 1);
        compare_possible_month::<FrenchRevMonth, FrenchRevArithMoment<false>>(t0, t1, 2);
        compare_possible_month::<FrenchRevMonth, FrenchRevArithMoment<true>>(t0, t1, 2);
    }

    #[test]
    fn french_rev_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_perennial_weekday::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArith<false>>(t0, t1, 0);
        compare_perennial_weekday::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArith<true>>(t0, t1, 0);
        compare_perennial_weekday::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArithMoment<false>>(t0, t1, 1);
        compare_perennial_weekday::<FrenchRevMonth, FrenchRevWeekday, FrenchRevArithMoment<true>>(t0, t1, 1);
    }

    #[test]
    fn french_rev_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<FrenchRevArith<false>>(t0, t1);
        compare_era_abbrev::<FrenchRevArith<true>>(t0, t1);
        compare_era_abbrev::<FrenchRevArithMoment<false>>(t0, t1);
        compare_era_abbrev::<FrenchRevArithMoment<true>>(t0, t1);
    }

    #[test]
    fn gregorian_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Gregorian>(t0);
        reasonable_blanks::<GregorianMoment>(t0);
    }

    #[test]
    fn gregorian_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<GregorianMonth, Gregorian>(t0, t1, 1);
        compare_month::<GregorianMonth, GregorianMoment>(t0, t1, 2);
    }

    #[test]
    fn gregorian_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Gregorian>(t0, t1, 0);
        compare_common_weekday::<GregorianMoment>(t0, t1, 1);
    }

    #[test]
    fn gregorian_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Gregorian>(t0, t1);
        compare_era_abbrev::<GregorianMoment>(t0, t1);
    }

    #[test]
    fn holocene_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Holocene>(t0);
        reasonable_blanks::<HoloceneMoment>(t0);
    }

    #[test]
    fn holocene_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<HoloceneMonth, Holocene>(t0, t1, 1);
        compare_month::<HoloceneMonth, HoloceneMoment>(t0, t1, 2);
    }

    #[test]
    fn holocene_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Holocene>(t0, t1, 0);
        compare_common_weekday::<HoloceneMoment>(t0, t1, 1);
    }

    #[test]
    fn holocene_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Holocene>(t0, t1);
        compare_era_abbrev::<HoloceneMoment>(t0, t1);
    }

    #[test]
    fn iso_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<ISO>(t0);
        reasonable_blanks::<ISOMoment>(t0);
    }

    #[test]
    fn iso_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<ISO>(t0, t1);
        compare_era_abbrev::<ISOMoment>(t0, t1);
    }

    #[test]
    fn julian_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Julian>(t0);
        reasonable_blanks::<JulianMoment>(t0);
    }

    #[test]
    fn julian_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<JulianMonth, Julian>(t0, t1, 1);
        compare_month::<JulianMonth, JulianMoment>(t0, t1, 2);
    }

    #[test]
    fn julian_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Julian>(t0, t1, 0);
        compare_common_weekday::<JulianMoment>(t0, t1, 1);
    }

    #[test]
    fn julian_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Julian>(t0, t1);
        compare_era_abbrev::<JulianMoment>(t0, t1);
    }

    #[test]
    fn positivist_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Positivist>(t0);
        reasonable_blanks::<PositivistMoment>(t0);
    }

    #[test]
    fn positivist_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<PositivistMonth, Positivist>(t0, t1, 1);
        compare_possible_month::<PositivistMonth, PositivistMoment>(t0, t1, 2);
    }

    #[test]
    fn positivist_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_perennial_weekday::<PositivistMonth, Weekday, Positivist>(t0, t1, 0);
        compare_perennial_weekday::<PositivistMonth, Weekday, PositivistMoment>(t0, t1, 1);
    }

    #[test]
    fn positivist_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Positivist>(t0, t1);
        compare_era_abbrev::<PositivistMoment>(t0, t1);
    }

    #[test]
    fn symmetry_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Symmetry010>(t0);
        reasonable_blanks::<Symmetry454>(t0);
        reasonable_blanks::<Symmetry010Solstice>(t0);
        reasonable_blanks::<Symmetry454Solstice>(t0);
        reasonable_blanks::<Symmetry010Moment>(t0);
        reasonable_blanks::<Symmetry454Moment>(t0);
        reasonable_blanks::<Symmetry010SolsticeMoment>(t0);
        reasonable_blanks::<Symmetry454SolsticeMoment>(t0);
    }

    #[test]
    fn symmetry_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_month::<SymmetryMonth, Symmetry010>(t0, t1, 1);
        compare_month::<SymmetryMonth, Symmetry454>(t0, t1, 1);
        compare_month::<SymmetryMonth, Symmetry010Solstice>(t0, t1, 1);
        compare_month::<SymmetryMonth, Symmetry454Solstice>(t0, t1, 1);
        compare_month::<SymmetryMonth, Symmetry010Moment>(t0, t1, 2);
        compare_month::<SymmetryMonth, Symmetry454Moment>(t0, t1, 2);
        compare_month::<SymmetryMonth, Symmetry010SolsticeMoment>(t0, t1, 2);
        compare_month::<SymmetryMonth, Symmetry454SolsticeMoment>(t0, t1, 2);
    }

    #[test]
    fn symmetry_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_common_weekday::<Symmetry010>(t0, t1, 0);
        compare_common_weekday::<Symmetry454>(t0, t1, 0);
        compare_common_weekday::<Symmetry010Solstice>(t0, t1, 0);
        compare_common_weekday::<Symmetry454Solstice>(t0, t1, 0);
        compare_common_weekday::<Symmetry010Moment>(t0, t1, 1);
        compare_common_weekday::<Symmetry454Moment>(t0, t1, 1);
        compare_common_weekday::<Symmetry010SolsticeMoment>(t0, t1, 1);
        compare_common_weekday::<Symmetry454SolsticeMoment>(t0, t1, 1);
    }

    #[test]
    fn symmetry_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Symmetry010>(t0, t1);
        compare_era_abbrev::<Symmetry454>(t0, t1);
        compare_era_abbrev::<Symmetry010Solstice>(t0, t1);
        compare_era_abbrev::<Symmetry454Solstice>(t0, t1);
        compare_era_abbrev::<Symmetry010Moment>(t0, t1);
        compare_era_abbrev::<Symmetry454Moment>(t0, t1);
        compare_era_abbrev::<Symmetry010SolsticeMoment>(t0, t1);
        compare_era_abbrev::<Symmetry454SolsticeMoment>(t0, t1);
    }

    #[test]
    fn tranquility_blanks(t0 in -FIXED_MAX..FIXED_MAX) {
        reasonable_blanks::<Tranquility>(t0);
        reasonable_blanks::<TranquilityMoment>(t0);
    }

    #[test]
    fn tranquility_month(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_possible_month::<TranquilityMonth, Tranquility>(t0, t1, 1);
        compare_possible_month::<TranquilityMonth, TranquilityMoment>(t0, t1, 2);
    }

    #[test]
    fn tranquility_weekday(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_perennial_weekday::<TranquilityMonth, Weekday, Tranquility>(t0, t1, 0);
        compare_perennial_weekday::<TranquilityMonth, Weekday, TranquilityMoment>(t0, t1, 1);
    }

    #[test]
    fn tranquility_era_abbrev(t0 in -FIXED_MAX..FIXED_MAX, t1 in -FIXED_MAX..FIXED_MAX) {
        compare_era_abbrev::<Tranquility>(t0, t1);
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
        let t = TimeOfDay::try_from_clock(c).unwrap();
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

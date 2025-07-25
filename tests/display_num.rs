use proptest::prelude::TestCaseError;
use proptest::prop_assume;
use proptest::proptest;
use radnelac::day_count::BoundedDayCount;
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
use radnelac::calendar::ISO;
use radnelac::calendar::PerennialWithComplementaryDay;
use radnelac::day_count::Epoch;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::display::PresetDisplay;
use radnelac::display::PresetFormat;
use radnelac::display::DDMMYYYY_SLASH;
use radnelac::display::MMDDYYYY_SLASH;
use radnelac::display::YEAR_WEEK_DAY;
use radnelac::display::YYYYMMDD_DASH;
use radnelac::display::YYYYMMDD_SLASH;
use radnelac::display::YYYYYMMDD_DASH;

const MAX_4DIGIT: f64 = 8000.0 * 365.25;
const MIN_4DIGIT: f64 = 1000.0 * 365.25;
const FR_MIN_4DIGIT: f64 = 1795.0 * 365.25;
const HL_MIN_5DIGIT: f64 = -9000.0 * 365.25;
const HL_MAX_5DIGIT: f64 = 87000.0 * 365.25;
const TQ_MIN_4DIGIT: f64 = 1975.0 * 365.25;

fn ymd_order_raw<T: FromFixed + PresetDisplay + PartialOrd>(preset: PresetFormat, d0: T, d1: T) {
    let s0 = d0.preset_str(preset);
    let s1 = d1.preset_str(preset);
    assert_eq!(d0 < d1, s0 < s1, "<  {} {}", s0, s1);
    assert_eq!(d0 <= d1, s0 <= s1, "<= {} {}", s0, s1);
    assert_eq!(d0 == d1, s0 == s1, "== {} {}", s0, s1);
    assert_eq!(d0 >= d1, s0 >= s1, ">= {} {}", s0, s1);
    assert_eq!(d0 > d1, s0 > s1, "> {} {}", s0, s1);
}

fn ymd_order<T: FromFixed + PresetDisplay + Epoch + PartialOrd>(
    preset: PresetFormat,
    t0: f64,
    t1: f64,
) {
    let f0 = Fixed::new(t0).to_day();
    let f1 = Fixed::new(t1).to_day();
    let d0 = T::from_fixed(f0);
    let d1 = T::from_fixed(f1);
    ymd_order_raw::<T>(preset, d0, d1);
}

fn ymd_order_tq(preset: PresetFormat, t0: f64, t1: f64) -> Result<(), TestCaseError> {
    let f0 = Fixed::new(t0).to_day();
    let f1 = Fixed::new(t1).to_day();
    let d0 = TranquilityMoment::from_fixed(f0);
    let d1 = TranquilityMoment::from_fixed(f1);
    prop_assume!(d0.complementary().is_none() && d1.complementary().is_none());
    ymd_order_raw::<TranquilityMoment>(preset, d0, d1);
    Ok(())
}

fn ymd_vs_dmy_vs_mdy<T: FromFixed + PresetDisplay + Epoch + PartialOrd>(t: f64) {
    let d = T::from_fixed(Fixed::new(t).to_day());
    let ymd0 = d.preset_str(YYYYMMDD_SLASH);
    let ymd1 = d.preset_str(DDMMYYYY_SLASH);
    let ymd2 = d.preset_str(MMDDYYYY_SLASH);
    let ymd3 = d.preset_str(YYYYMMDD_DASH);
    assert_eq!(&ymd0[0..4], &ymd1[6..10]);
    assert_eq!(&ymd0[5..7], &ymd1[3..5]);
    assert_eq!(&ymd0[8..10], &ymd1[0..2]);
    assert_eq!(&ymd0[0..4], &ymd2[6..10]);
    assert_eq!(&ymd0[5..7], &ymd2[0..2]);
    assert_eq!(&ymd0[8..10], &ymd2[3..5]);
    assert_eq!(&ymd0[0..4], &ymd3[0..4]);
    assert_eq!(&ymd0[5..7], &ymd3[5..7]);
    assert_eq!(&ymd0[8..10], &ymd3[8..10]);
}

proptest! {
    #[test]
    fn armenian(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Armenian>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Armenian>(t0);
        ymd_vs_dmy_vs_mdy::<Armenian>(t1);
    }

    #[test]
    fn coptic(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Coptic>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Coptic>(t0);
        ymd_vs_dmy_vs_mdy::<Coptic>(t1);
    }

    #[test]
    fn cotsworth(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Cotsworth>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Cotsworth>(t0);
        ymd_vs_dmy_vs_mdy::<Cotsworth>(t1);
    }

    #[test]
    fn egyptian(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Egyptian>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Egyptian>(t0);
        ymd_vs_dmy_vs_mdy::<Egyptian>(t1);
    }

    #[test]
    fn ethiopic(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Ethiopic>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Ethiopic>(t0);
        ymd_vs_dmy_vs_mdy::<Ethiopic>(t1);
    }

    #[test]
    fn french_rev_arith(t0 in FR_MIN_4DIGIT..MAX_4DIGIT, t1 in FR_MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<FrenchRevArith<true>>(YYYYMMDD_DASH, t0, t1);
        ymd_order::<FrenchRevArith<false>>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<FrenchRevArith<true>>(t0);
        ymd_vs_dmy_vs_mdy::<FrenchRevArith<false>>(t1);
        ymd_vs_dmy_vs_mdy::<FrenchRevArith<true>>(t0);
        ymd_vs_dmy_vs_mdy::<FrenchRevArith<false>>(t1);

    }

    #[test]
    fn gregorian(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Gregorian>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Gregorian>(t0);
        ymd_vs_dmy_vs_mdy::<Gregorian>(t1);
    }

    #[test]
    fn holocene(t0 in HL_MIN_5DIGIT..HL_MAX_5DIGIT, t1 in HL_MIN_5DIGIT..HL_MAX_5DIGIT) {
        ymd_order::<Holocene>(YYYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Holocene>(t0);
        ymd_vs_dmy_vs_mdy::<Holocene>(t1);
    }

    #[test]
    fn iso(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<ISO>(YEAR_WEEK_DAY, t0, t1);
    }

    #[test]
    fn positivist(t0 in FR_MIN_4DIGIT..MAX_4DIGIT, t1 in FR_MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Positivist>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Positivist>(t0);
        ymd_vs_dmy_vs_mdy::<Positivist>(t1);
    }

    #[test]
    fn julian(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Julian>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Julian>(t0);
        ymd_vs_dmy_vs_mdy::<Julian>(t1);
    }

    #[test]
    fn symmetry(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Symmetry454>(YYYYMMDD_DASH, t0, t1);
        ymd_order::<Symmetry010>(YYYYMMDD_DASH, t0, t1);
        ymd_order::<Symmetry454Solstice>(YYYYMMDD_DASH, t0, t1);
        ymd_order::<Symmetry010Solstice>(YYYYMMDD_DASH, t0, t1);
        ymd_vs_dmy_vs_mdy::<Symmetry010>(t0);
        ymd_vs_dmy_vs_mdy::<Symmetry010>(t1);
        ymd_vs_dmy_vs_mdy::<Symmetry454>(t0);
        ymd_vs_dmy_vs_mdy::<Symmetry454>(t1);
        ymd_vs_dmy_vs_mdy::<Symmetry010Solstice>(t0);
        ymd_vs_dmy_vs_mdy::<Symmetry010Solstice>(t1);
        ymd_vs_dmy_vs_mdy::<Symmetry454Solstice>(t0);
        ymd_vs_dmy_vs_mdy::<Symmetry454Solstice>(t1);

    }

    #[test]
    fn tranquility(t0 in TQ_MIN_4DIGIT..MAX_4DIGIT, t1 in TQ_MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order_tq(YYYYMMDD_DASH, t0, t1)?;
        ymd_vs_dmy_vs_mdy::<TranquilityMoment>(t0);
        ymd_vs_dmy_vs_mdy::<TranquilityMoment>(t1);
    }
}

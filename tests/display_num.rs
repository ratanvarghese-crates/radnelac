use proptest::prelude::TestCaseError;
use proptest::prop_assume;
use proptest::proptest;
use radnelac::bound::BoundedDayCount;
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
use radnelac::date::PerennialWithComplementaryDay;
use radnelac::day_count::Epoch;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::display::preset_fmt::PresetDisplay;
use radnelac::display::preset_fmt::PresetFormat;
use radnelac::display::preset_fmt::Y5_MD_DASH;
use radnelac::display::preset_fmt::YEAR_WEEK_DAY;
use radnelac::display::preset_fmt::YMD_DASH;

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

proptest! {
    #[test]
    fn armenian(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Armenian>(YMD_DASH, t0, t1);
    }

    #[test]
    fn coptic(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Coptic>(YMD_DASH, t0, t1);
    }

    #[test]
    fn cotsworth(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Cotsworth>(YMD_DASH, t0, t1);
    }

    #[test]
    fn egyptian(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Egyptian>(YMD_DASH, t0, t1);
    }

    #[test]
    fn ethiopic(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Ethiopic>(YMD_DASH, t0, t1);
    }

    #[test]
    fn french_rev_arith(t0 in FR_MIN_4DIGIT..MAX_4DIGIT, t1 in FR_MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<FrenchRevArith<true>>(YMD_DASH, t0, t1);
        ymd_order::<FrenchRevArith<false>>(YMD_DASH, t0, t1);
    }

    #[test]
    fn gregorian(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Gregorian>(YMD_DASH, t0, t1);
    }

    #[test]
    fn holocene(t0 in HL_MIN_5DIGIT..HL_MAX_5DIGIT, t1 in HL_MIN_5DIGIT..HL_MAX_5DIGIT) {
        ymd_order::<Holocene>(Y5_MD_DASH, t0, t1);
    }

    #[test]
    fn iso(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<ISO>(YEAR_WEEK_DAY, t0, t1);
    }

    #[test]
    fn positivist(t0 in FR_MIN_4DIGIT..MAX_4DIGIT, t1 in FR_MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Positivist>(YMD_DASH, t0, t1);
    }

    #[test]
    fn julian(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Julian>(YMD_DASH, t0, t1);
    }

    #[test]
    fn symmetry(t0 in MIN_4DIGIT..MAX_4DIGIT, t1 in MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order::<Symmetry454>(YMD_DASH, t0, t1);
        ymd_order::<Symmetry010>(YMD_DASH, t0, t1);
        ymd_order::<Symmetry454Solstice>(YMD_DASH, t0, t1);
        ymd_order::<Symmetry010Solstice>(YMD_DASH, t0, t1);
    }

    #[test]
    fn tranquility(t0 in TQ_MIN_4DIGIT..MAX_4DIGIT, t1 in TQ_MIN_4DIGIT..MAX_4DIGIT) {
        ymd_order_tq(YMD_DASH, t0, t1)?
    }
}

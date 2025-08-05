#[cfg(feature = "display")]
use proptest::proptest;
#[cfg(feature = "display")]
use radnelac::calendar::Armenian;
#[cfg(feature = "display")]
use radnelac::calendar::Coptic;
#[cfg(feature = "display")]
use radnelac::calendar::Cotsworth;
#[cfg(feature = "display")]
use radnelac::calendar::Egyptian;
#[cfg(feature = "display")]
use radnelac::calendar::Ethiopic;
#[cfg(feature = "display")]
use radnelac::calendar::FrenchRevArith;
#[cfg(feature = "display")]
use radnelac::calendar::Gregorian;
#[cfg(feature = "display")]
use radnelac::calendar::Holocene;
#[cfg(feature = "display")]
use radnelac::calendar::Julian;
#[cfg(feature = "display")]
use radnelac::calendar::Positivist;
#[cfg(feature = "display")]
use radnelac::calendar::Symmetry010;
#[cfg(feature = "display")]
use radnelac::calendar::Symmetry010Solstice;
#[cfg(feature = "display")]
use radnelac::calendar::Symmetry454;
#[cfg(feature = "display")]
use radnelac::calendar::Symmetry454Solstice;
#[cfg(feature = "display")]
use radnelac::day_count::BoundedDayCount;
#[cfg(feature = "display")]
use radnelac::day_count::Fixed;
#[cfg(feature = "display")]
use radnelac::day_count::FromFixed;
#[cfg(feature = "display")]
use radnelac::day_count::FIXED_MAX;
#[cfg(feature = "display")]
use radnelac::display::Language;
#[cfg(feature = "display")]
use radnelac::display::PresetDisplay;
#[cfg(feature = "display")]
use radnelac::display::PresetFormat;
#[cfg(feature = "display")]
use radnelac::display::HHMMSS_COLON;

#[cfg(feature = "display")]
fn display_exact<T: FromFixed + PresetDisplay>(t: f64, fmt: PresetFormat, s: &str) {
    let d = T::from_fixed(Fixed::new(t));
    let ds = d.preset_str(Language::EN, fmt);
    assert_eq!(ds, s)
}

#[cfg(feature = "display")]
fn display_midnight<T: FromFixed + PresetDisplay>(t: f64) {
    display_exact::<T>(t, HHMMSS_COLON, "00:00:00");
}

#[cfg(feature = "display")]
proptest! {
    #[test]
    fn armenian(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Armenian>(t0);
    }

    #[test]
    fn coptic(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Coptic>(t0);
    }

    #[test]
    fn cotsworth(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Cotsworth>(t0);
    }

    #[test]
    fn egyptian(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Egyptian>(t0);
    }

    #[test]
    fn ethiopic(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Ethiopic>(t0);
    }

    #[test]
    fn french_rev(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<FrenchRevArith<false>>(t0);
        display_midnight::<FrenchRevArith<true>>(t0);
    }

    #[test]
    fn gregorian(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Gregorian>(t0);
    }

    #[test]
    fn holocene(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Holocene>(t0);
    }

    #[test]
    fn julian(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Julian>(t0);
    }

    #[test]
    fn positivist(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Positivist>(t0);
    }

    #[test]
    fn symmetry(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Symmetry010>(t0);
        display_midnight::<Symmetry454>(t0);
        display_midnight::<Symmetry010Solstice>(t0);
        display_midnight::<Symmetry454Solstice>(t0);
    }
}

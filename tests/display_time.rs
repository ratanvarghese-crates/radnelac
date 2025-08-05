#[cfg(feature = "display")]
mod display_logic {
    pub use proptest::proptest;
    pub use radnelac::calendar::Armenian;
    pub use radnelac::calendar::Coptic;
    pub use radnelac::calendar::Cotsworth;
    pub use radnelac::calendar::Egyptian;
    pub use radnelac::calendar::Ethiopic;
    pub use radnelac::calendar::FrenchRevArith;
    pub use radnelac::calendar::Gregorian;
    pub use radnelac::calendar::Holocene;
    pub use radnelac::calendar::Julian;
    pub use radnelac::calendar::Positivist;
    pub use radnelac::calendar::Symmetry010;
    pub use radnelac::calendar::Symmetry010Solstice;
    pub use radnelac::calendar::Symmetry454;
    pub use radnelac::calendar::Symmetry454Solstice;
    pub use radnelac::day_count::BoundedDayCount;
    pub use radnelac::day_count::Fixed;
    pub use radnelac::day_count::FromFixed;
    pub use radnelac::day_count::FIXED_MAX;
    pub use radnelac::display::Language;
    pub use radnelac::display::PresetDisplay;
    pub use radnelac::display::PresetFormat;
    pub use radnelac::display::HHMMSS_COLON;

    pub fn display_exact<T: FromFixed + PresetDisplay>(t: f64, fmt: PresetFormat, s: &str) {
        let d = T::from_fixed(Fixed::new(t));
        let ds = d.preset_str(Language::EN, fmt);
        assert_eq!(ds, s)
    }

    pub fn display_midnight<T: FromFixed + PresetDisplay>(t: f64) {
        display_exact::<T>(t, HHMMSS_COLON, "00:00:00");
    }
}

#[cfg(feature = "display")]
use display_logic::*;

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

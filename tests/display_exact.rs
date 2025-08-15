// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
    pub use radnelac::display::COMPL_ONLY;
    pub use radnelac::display::HHMMSS_COLON;

    pub fn display_exact<T: FromFixed + PresetDisplay>(t: f64, fmt: PresetFormat, s: &str) {
        let d = T::from_fixed(Fixed::new(t));
        let ds = d.preset_str(Language::EN, fmt);
        assert_eq!(ds, s)
    }

    pub fn display_midnight<T: FromFixed + PresetDisplay>(t: f64) {
        display_exact::<T>(t, HHMMSS_COLON, "00:00:00");
    }

    pub fn display_blank_complementary<T: FromFixed + PresetDisplay>(t: f64) {
        display_exact::<T>(t, COMPL_ONLY, "");
    }
}

#[cfg(feature = "display")]
use display_logic::*;

#[cfg(feature = "display")]
proptest! {
    #[test]
    fn armenian_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Armenian>(t0);
    }

    #[test]
    fn coptic_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Coptic>(t0);
    }

    #[test]
    fn cotsworth_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Cotsworth>(t0);
    }

    #[test]
    fn egyptian_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Egyptian>(t0);
    }

    #[test]
    fn ethiopic_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Ethiopic>(t0);
    }

    #[test]
    fn ethiopic_epagomenae(t0 in -FIXED_MAX..FIXED_MAX) {
        display_blank_complementary::<Ethiopic>(t0);
    }

    #[test]
    fn french_rev_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<FrenchRevArith<false>>(t0);
        display_midnight::<FrenchRevArith<true>>(t0);
    }

    #[test]
    fn gregorian_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Gregorian>(t0);
    }

    #[test]
    fn gregorian_epagomenae(t0 in -FIXED_MAX..FIXED_MAX) {
        display_blank_complementary::<Gregorian>(t0);
    }

    #[test]
    fn holocene_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Holocene>(t0);
    }

    #[test]
    fn holocene_epagomenae(t0 in -FIXED_MAX..FIXED_MAX) {
        display_blank_complementary::<Holocene>(t0);
    }

    #[test]
    fn julian_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Julian>(t0);
    }

    #[test]
    fn julian_epagomenae(t0 in -FIXED_MAX..FIXED_MAX) {
        display_blank_complementary::<Julian>(t0);
    }

    #[test]
    fn positivist_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Positivist>(t0);
    }

    #[test]
    fn symmetry_midnight(t0 in -FIXED_MAX..FIXED_MAX) {
        display_midnight::<Symmetry010>(t0);
        display_midnight::<Symmetry454>(t0);
        display_midnight::<Symmetry010Solstice>(t0);
        display_midnight::<Symmetry454Solstice>(t0);
    }

    #[test]
    fn symmetry_epagomenae(t0 in -FIXED_MAX..FIXED_MAX) {
        display_blank_complementary::<Symmetry010>(t0);
        display_blank_complementary::<Symmetry454>(t0);
        display_blank_complementary::<Symmetry010Solstice>(t0);
        display_blank_complementary::<Symmetry454Solstice>(t0);
    }
}

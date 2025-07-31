#[cfg(feature = "display")]
use num_traits::cast::FromPrimitive;
#[cfg(feature = "display")]
use proptest::proptest;
#[cfg(feature = "display")]
use radnelac::calendar::Cotsworth;
#[cfg(feature = "display")]
use radnelac::calendar::CotsworthMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Gregorian;
#[cfg(feature = "display")]
use radnelac::calendar::GregorianMonth;
#[cfg(feature = "display")]
use radnelac::calendar::GuaranteedMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Holocene;
#[cfg(feature = "display")]
use radnelac::calendar::HoloceneMonth;
#[cfg(feature = "display")]
use radnelac::calendar::Julian;
#[cfg(feature = "display")]
use radnelac::calendar::JulianMonth;
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
use radnelac::display::Language;
#[cfg(feature = "display")]
use radnelac::display::PresetDisplay;
#[cfg(feature = "display")]
use radnelac::display::LONG_DATE;

use radnelac::day_count::FIXED_MAX;

#[cfg(feature = "display")]
const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

#[cfg(feature = "display")]
fn long_date_contains<T: PresetDisplay>(d: T, lang: Language, s: &str) {
    assert!(d.preset_str(lang, LONG_DATE).contains(s));
}

#[cfg(feature = "display")]
fn bilingual_long_date_contains<T: PresetDisplay + Copy>(d: T, s_en: &str, s_fr: &str) {
    assert!(T::supported_lang(Language::EN));
    assert!(T::supported_lang(Language::FR));
    long_date_contains(d, Language::EN, s_en);
    long_date_contains(d, Language::FR, s_fr);
}

#[cfg(feature = "display")]
fn gregorian_like_date_contains<T: PresetDisplay + Copy + GuaranteedMonth<GregorianMonth>>(d: T) {
    match d.month() {
        GregorianMonth::January => bilingual_long_date_contains(d, "January", "janvier"),
        GregorianMonth::February => bilingual_long_date_contains(d, "February", "février"),
        GregorianMonth::March => bilingual_long_date_contains(d, "March", "mars"),
        GregorianMonth::April => bilingual_long_date_contains(d, "April", "avril"),
        GregorianMonth::May => bilingual_long_date_contains(d, "May", "mai"),
        GregorianMonth::June => bilingual_long_date_contains(d, "June", "juin"),
        GregorianMonth::July => bilingual_long_date_contains(d, "July", "juillet"),
        GregorianMonth::August => bilingual_long_date_contains(d, "August", "août"),
        GregorianMonth::September => bilingual_long_date_contains(d, "September", "septembre"),
        GregorianMonth::October => bilingual_long_date_contains(d, "October", "octobre"),
        GregorianMonth::November => bilingual_long_date_contains(d, "November", "novembre"),
        GregorianMonth::December => bilingual_long_date_contains(d, "December", "decembre"),
    };
}

#[cfg(feature = "display")]
fn symmetry_date_contains<T: PresetDisplay + Copy + GuaranteedMonth<SymmetryMonth>>(d: T) {
    match d.month() {
        SymmetryMonth::January => bilingual_long_date_contains(d, "January", "janvier"),
        SymmetryMonth::February => bilingual_long_date_contains(d, "February", "février"),
        SymmetryMonth::March => bilingual_long_date_contains(d, "March", "mars"),
        SymmetryMonth::April => bilingual_long_date_contains(d, "April", "avril"),
        SymmetryMonth::May => bilingual_long_date_contains(d, "May", "mai"),
        SymmetryMonth::June => bilingual_long_date_contains(d, "June", "juin"),
        SymmetryMonth::July => bilingual_long_date_contains(d, "July", "juillet"),
        SymmetryMonth::August => bilingual_long_date_contains(d, "August", "août"),
        SymmetryMonth::September => bilingual_long_date_contains(d, "September", "septembre"),
        SymmetryMonth::October => bilingual_long_date_contains(d, "October", "octobre"),
        SymmetryMonth::November => bilingual_long_date_contains(d, "November", "novembre"),
        SymmetryMonth::December => bilingual_long_date_contains(d, "December", "decembre"),
        SymmetryMonth::Irvember => bilingual_long_date_contains(d, "Irvember", "irvembre"),
    };
}

#[cfg(feature = "display")]
proptest! {
    #[test]
    fn cotsworth(year in -MAX_YEARS..MAX_YEARS, m in 1..13, day in 1..28) {
        let month = CotsworthMonth::from_i32(m).unwrap();
        let d = Cotsworth::try_new(year, month, day as u8).unwrap();
        match month {
            CotsworthMonth::January => bilingual_long_date_contains(d, "January", "janvier"),
            CotsworthMonth::February => bilingual_long_date_contains(d, "February", "février"),
            CotsworthMonth::March => bilingual_long_date_contains(d, "March", "mars"),
            CotsworthMonth::April => bilingual_long_date_contains(d, "April", "avril"),
            CotsworthMonth::May => bilingual_long_date_contains(d, "May", "mai"),
            CotsworthMonth::June => bilingual_long_date_contains(d, "June", "juin"),
            CotsworthMonth::July => bilingual_long_date_contains(d, "July", "juillet"),
            CotsworthMonth::Sol => bilingual_long_date_contains(d, "Sol", "sol"),
            CotsworthMonth::August => bilingual_long_date_contains(d, "August", "août"),
            CotsworthMonth::September => bilingual_long_date_contains(d, "September", "septembre"),
            CotsworthMonth::October => bilingual_long_date_contains(d, "October", "octobre"),
            CotsworthMonth::November => bilingual_long_date_contains(d, "November", "novembre"),
            CotsworthMonth::December => bilingual_long_date_contains(d, "December", "decembre"),
        };
    }

    #[test]
    fn gregorian(year in -MAX_YEARS..MAX_YEARS, m in 1..12, day in 1..28) {
        let month = GregorianMonth::from_i32(m).unwrap();
        let d = Gregorian::try_new(year, month, day as u8).unwrap();
        gregorian_like_date_contains(d);
    }

    #[test]
    fn holocene(year in -MAX_YEARS..MAX_YEARS, m in 1..12, day in 1..28) {
        let month = HoloceneMonth::from_i32(m).unwrap();
        let d = Holocene::try_new(year, month, day as u8).unwrap();
        gregorian_like_date_contains(d);
    }

    #[test]
    fn julian(year in -MAX_YEARS..MAX_YEARS, m in 1..12, day in 1..28) {
        let month = JulianMonth::from_i32(m).unwrap();
        let d = Julian::try_new(year, month, day as u8).unwrap();
        gregorian_like_date_contains(d);
    }

    #[test]
    fn symmetry(year in -MAX_YEARS..MAX_YEARS, m in 1..12, day in 1..28) {
        let month = SymmetryMonth::from_i32(m).unwrap();
        symmetry_date_contains(Symmetry454::try_new(year, month, day as u8).unwrap());
        symmetry_date_contains(Symmetry010::try_new(year, month, day as u8).unwrap());
        symmetry_date_contains(Symmetry454Solstice::try_new(year, month, day as u8).unwrap());
        symmetry_date_contains(Symmetry010Solstice::try_new(year, month, day as u8).unwrap());
    }
}

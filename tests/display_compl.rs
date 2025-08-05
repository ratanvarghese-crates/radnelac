#[cfg(feature = "display")]
mod display_logic {
    pub use num_traits::FromPrimitive;
    pub use num_traits::ToPrimitive;
    pub use proptest::prop_assume;
    pub use proptest::proptest;
    pub use radnelac::calendar::CommonDate;
    pub use radnelac::calendar::Cotsworth;
    pub use radnelac::calendar::CotsworthComplementaryDay;
    pub use radnelac::calendar::CotsworthMonth;
    pub use radnelac::calendar::FrenchRevArith;
    pub use radnelac::calendar::FrenchRevMonth;
    pub use radnelac::calendar::HasIntercalaryDays;
    pub use radnelac::calendar::HasLeapYears;
    pub use radnelac::calendar::Positivist;
    pub use radnelac::calendar::PositivistComplementaryDay;
    pub use radnelac::calendar::PositivistMonth;
    pub use radnelac::calendar::Sansculottide;
    pub use radnelac::calendar::ToFromCommonDate;
    pub use radnelac::calendar::TranquilityComplementaryDay;
    pub use radnelac::calendar::TranquilityMoment;
    pub use radnelac::calendar::TranquilityMonth;
    pub use radnelac::day_count::FIXED_MAX;
    pub use radnelac::display::Language;
    pub use radnelac::display::PresetDisplay;
    pub use radnelac::display::COMPL_ONLY;

    pub const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

    pub fn perennial_compl<S, T, U>(cd: CommonDate, lang: Language, expected: &str)
    where
        S: ToPrimitive + FromPrimitive,
        T: ToPrimitive + FromPrimitive,
        U: ToFromCommonDate<S> + HasIntercalaryDays<T> + PresetDisplay + PartialOrd,
    {
        let d = U::try_from_common_date(cd).unwrap();
        let s = d.preset_str(lang, COMPL_ONLY);
        assert_eq!(s, expected);
    }
}

#[cfg(feature = "display")]
use display_logic::*;

#[cfg(feature = "display")]
proptest! {
    #[test]
    fn cotsworth_year_day(year in (-MAX_YEARS)..MAX_YEARS) {
        let cd = CommonDate::new(year, 13, 29);
        perennial_compl::<CotsworthMonth, CotsworthComplementaryDay, Cotsworth>(cd, Language::EN, "Year Day");
        perennial_compl::<CotsworthMonth, CotsworthComplementaryDay, Cotsworth>(cd, Language::FR, "Jour de l'année");
    }

    #[test]
    fn cotsworth_leap_day(year in (-MAX_YEARS/4)..MAX_YEARS/4) {
        let y = (year * 4) as i32;
        prop_assume!(Cotsworth::is_leap(y));
        let cd = CommonDate::new(y, 6, 29);
        perennial_compl::<CotsworthMonth, CotsworthComplementaryDay, Cotsworth>(cd, Language::EN, "Leap Day");
        perennial_compl::<CotsworthMonth, CotsworthComplementaryDay, Cotsworth>(cd, Language::FR, "Journée bissextile");
    }

    #[test]
    fn cotsworth_invalid(year in (-MAX_YEARS)..MAX_YEARS, month in 1..13, day in 1..28) {
        let cd = CommonDate::new(year, month as u8, day as u8);
        perennial_compl::<CotsworthMonth, CotsworthComplementaryDay, Cotsworth>(cd, Language::EN, "");
        perennial_compl::<CotsworthMonth, CotsworthComplementaryDay, Cotsworth>(cd, Language::FR, "");
    }

    #[test]
    fn french_rev_sansculottide(year in (-MAX_YEARS)..MAX_YEARS) {
        let item_list = [
            (1, "Celebration of Virtue", "La Fête de la Vertu"),
            (2, "Celebration of Talent", "La Fête du Génie"),
            (3, "Celebration of Labor", "La Fête du Travail"),
            (4, "Celebration of Convictions", "La Fête de l'Opinion"),
            (5, "Celebration of Honours", "La Fête des Récompenses")
        ];

        for item in item_list {
            let cd = CommonDate::new(year, 13, item.0);
            perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<true>>(cd, Language::EN, item.1);
            perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<true>>(cd, Language::FR, item.2);
            perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<false>>(cd, Language::EN, item.1);
            perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<false>>(cd, Language::FR, item.2);
        }
    }

    #[test]
    fn french_rev_leap_sansculottide(year in (-MAX_YEARS/4)..MAX_YEARS/4) {
        let y = (year * 4) as i32;
        prop_assume!(FrenchRevArith::<false>::is_leap(y));
        let item_list = [
            (6, "Celebration of the Revolution", "La Fête de la Révolution")
        ];
        for item in item_list {
            let cd = CommonDate::new(y, 13, item.0);
            perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<false>>(cd, Language::EN, item.1);
            perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<false>>(cd, Language::FR, item.2);
        }
    }

    #[test]
    fn french_rev_invalid(year in (-MAX_YEARS)..MAX_YEARS, month in 1..12, day in 1..30) {
        let cd = CommonDate::new(year, month as u8, day as u8);
        perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<false>>(cd, Language::EN, "");
        perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<false>>(cd, Language::FR, "");
        perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<true>>(cd, Language::EN, "");
        perennial_compl::<FrenchRevMonth, Sansculottide, FrenchRevArith<true>>(cd, Language::FR, "");
    }

    #[test]
    fn positivist_festival_of_dead(year in (-MAX_YEARS)..MAX_YEARS) {
        let cd = CommonDate::new(year, 14, 1);
        perennial_compl::<PositivistMonth, PositivistComplementaryDay, Positivist>(cd, Language::EN, "Festival of the Dead");
        perennial_compl::<PositivistMonth, PositivistComplementaryDay, Positivist>(cd, Language::FR, "La Fête universelle des Morts");
    }

    #[test]
    fn positivist_festival_of_women(year in (-MAX_YEARS/4)..MAX_YEARS/4) {
        let y = (year * 4) as i32;
        prop_assume!(Positivist::is_leap(y));
        let cd = CommonDate::new(y, 14, 2);
        perennial_compl::<PositivistMonth, PositivistComplementaryDay, Positivist>(cd, Language::EN, "Festival of Holy Women");
        perennial_compl::<PositivistMonth, PositivistComplementaryDay, Positivist>(cd, Language::FR, "La Fête Générale des Saintes Femmes");
    }

    #[test]
    fn positivist_invalid(year in (-MAX_YEARS)..MAX_YEARS, month in 1..13, day in 1..28) {
        let cd = CommonDate::new(year, month as u8, day as u8);
        perennial_compl::<PositivistMonth, PositivistComplementaryDay, Positivist>(cd, Language::EN, "");
        perennial_compl::<PositivistMonth, PositivistComplementaryDay, Positivist>(cd, Language::FR, "");
    }

    #[test]
    fn tranquility_armstrong(year in (-MAX_YEARS)..MAX_YEARS) {
        prop_assume!(year != -1 && year != 0 && year != 1);
        let cd = CommonDate::new(year, 0, 1);
        perennial_compl::<TranquilityMonth, TranquilityComplementaryDay, TranquilityMoment>(cd, Language::EN, "Armstrong Day");
    }

    #[test]
    fn tranquility_invalid(year in (-MAX_YEARS)..MAX_YEARS, month in 1..13, day in 1..28) {
        let cd = CommonDate::new(year, month as u8, day as u8);
        perennial_compl::<TranquilityMonth, TranquilityComplementaryDay, TranquilityMoment>(cd, Language::EN, "");
    }
}

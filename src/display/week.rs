use crate::day_cycle::Weekday;
use crate::display::private::fmt_number;
use crate::display::private::fmt_string;
use crate::display::private::get_dict;
use crate::display::private::Content;
use crate::display::private::DisplayItem;
use crate::display::private::Item;
use crate::display::private::NumericContent;
use crate::display::private::Sign;
use crate::display::private::TextContent;
use crate::display::text::en::EN_DICTIONARY;
use crate::display::text::prelude::Language;
use crate::display::PresetDisplay;
use crate::display::WEEKDAY_NAME_ONLY;
use std::fmt;

use crate::display::private::DisplayOptions;

impl DisplayItem for Weekday {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).common_weekday.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::DayOfWeek => fmt_number(*self as i16, opt),
            _ => "".to_string(),
        }
    }
    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        match (t, get_dict(lang).common_weekday.as_ref()) {
            (TextContent::DayOfWeekName, Some(dict)) => {
                let days: [&str; 7] = [
                    dict.sunday,
                    dict.monday,
                    dict.tuesday,
                    dict.wednesday,
                    dict.thursday,
                    dict.friday,
                    dict.saturday,
                ];
                let name = days[*self as usize];
                fmt_string(name, opt)
            }
            (_, _) => "".to_string(),
        }
    }
}

impl PresetDisplay for Weekday {}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const O: DisplayOptions = DisplayOptions {
            numerals: None,
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
        };
        let item = Item::new(Content::Text(TextContent::DayOfWeekName), O);
        write!(f, "{}", self.fmt_item(Language::EN, item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::ToPrimitive;

    #[test]
    fn weekday_display_french() {
        assert!(Weekday::supported_display_lang(Language::EN));
        assert!(Weekday::supported_display_lang(Language::FR));
        let w_list = [
            (Weekday::Monday, "Monday", "Lundi", "1"),
            (Weekday::Tuesday, "Tuesday", "Mardi", "2"),
            (Weekday::Wednesday, "Wednesday", "Mercredi", "3"),
            (Weekday::Thursday, "Thursday", "Jeudi", "4"),
            (Weekday::Friday, "Friday", "Vendredi", "5"),
            (Weekday::Saturday, "Saturday", "Samedi", "6"),
            (Weekday::Sunday, "Sunday", "Dimanche", "0"),
        ];
        for item in w_list {
            let w = item.0;
            let s0_en = item.1;
            let s0_fr = item.2;
            let n0 = item.3;
            let s1_en = w.preset_str(Language::EN, WEEKDAY_NAME_ONLY);
            let s1_fr = w.preset_str(Language::FR, WEEKDAY_NAME_ONLY);
            assert_eq!(s0_en, s1_en);
            assert_eq!(s0_fr, s1_fr);
            const O: DisplayOptions = DisplayOptions {
                numerals: None,
                width: None,
                align: None,
                padding: None,
                case: None,
                sign: Sign::Never,
            };
            assert_eq!(w.fmt_numeric(NumericContent::DayOfWeek, O), n0);
        }
    }
}

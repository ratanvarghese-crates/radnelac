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

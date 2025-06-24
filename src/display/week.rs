use crate::day_cycle::Weekday;
use crate::display::private::fmt_number;
use crate::display::private::fmt_string;
use crate::format::Content;
use crate::format::DisplayItem;
use crate::format::Item;
use crate::format::Locale;
use crate::format::NumericContent;
use crate::format::Sign;
use crate::format::TextContent;
use std::fmt;

use crate::format::DisplayOptions;

impl DisplayItem for Weekday {
    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::DayOfWeek => fmt_number(*self as i16, opt),
            _ => "".to_string(),
        }
    }
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        match t {
            TextContent::DayOfWeekName => {
                const DAYS: [&str; 7] = [
                    "Sunday",
                    "Monday",
                    "Tuesday",
                    "Wednesday",
                    "Thursday",
                    "Friday",
                    "Saturday",
                ];
                let name = DAYS[*self as usize];
                fmt_string(name, opt)
            }
            _ => "".to_string(),
        }
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const O: DisplayOptions = DisplayOptions { numerals: None,
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        let item = Item::new(Content::Text(TextContent::DayOfWeekName), O);
        write!(f, "{}", self.fmt_item(item))
    }
}

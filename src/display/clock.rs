use crate::clock::ClockTime;
use crate::clock::TimeOfDay;
use crate::common::math::TermNum;
use crate::display::common::fmt_number;
use crate::display::common::fmt_string;
use crate::display::common::Content;
use crate::display::common::DisplayItem;
use crate::display::common::Item;
use crate::display::common::Locale;
use crate::display::common::NumericContent;
use crate::display::common::Sign;

use crate::display::common::DisplayOptions;

use crate::display::common::TextContent;

use std::fmt;

impl DisplayItem for TimeOfDay {
    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Hour1to12 => fmt_number(
                (ClockTime::new(*self).hours as i64).adjusted_remainder(12),
                opt,
            ),
            NumericContent::Hour0to23 => fmt_number(ClockTime::new(*self).hours as i16, opt),
            NumericContent::Minute => fmt_number(ClockTime::new(*self).minutes as i16, opt),
            NumericContent::Second => fmt_number(ClockTime::new(*self).seconds as i16, opt),
            _ => "".to_string(),
        }
    }
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        match t {
            TextContent::HalfDayName => {
                if *self < TimeOfDay::new(0.5) {
                    fmt_string("Ante Meridiem", opt)
                } else {
                    fmt_string("Post Meridiem", opt)
                }
            }
            TextContent::HalfDayAbbrev => {
                if *self < TimeOfDay::new(0.5) {
                    fmt_string("AM", opt)
                } else {
                    fmt_string("PM", opt)
                }
            }
            _ => "".to_string(),
        }
    }
}
impl fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const O: DisplayOptions = DisplayOptions {
            width: Some(2),
            align: None,
            padding: Some('0'),
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };

        const ITEMS: [Item<'_>; 5] = [
            Item::new(Content::Numeric(NumericContent::Hour0to23), O),
            Item::new(Content::Literal(":"), O),
            Item::new(Content::Numeric(NumericContent::Minute), O),
            Item::new(Content::Literal(":"), O),
            Item::new(Content::Numeric(NumericContent::Second), O),
        ];
        for item in ITEMS {
            write!(f, "{}", self.fmt_item(item))?;
        }
        Ok(())
    }
}

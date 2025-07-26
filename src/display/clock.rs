use crate::clock::ClockTime;
use crate::clock::TimeOfDay;
use crate::common::math::TermNum;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::HHMMSS_COLON;
use crate::display::private::fmt_number;
use crate::display::private::fmt_string;
use crate::display::private::DisplayItem;
use crate::display::private::NumericContent;

use crate::display::private::DisplayOptions;

use crate::display::private::TextContent;

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

impl PresetDisplay for TimeOfDay {}

impl fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.preset_str(HHMMSS_COLON))
    }
}

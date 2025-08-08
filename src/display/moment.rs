use crate::calendar::CalendarMoment;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::NumericContent;
use crate::display::private::TextContent;
use crate::display::Language;
use crate::display::PresetDisplay;
use crate::display::HHMMSS_COLON;
use std::fmt;

use crate::clock::ClockTime;

pub trait DisplayMomentItem {}

impl<T: DisplayItem + Clone + DisplayMomentItem> DisplayItem for CalendarMoment<T> {
    fn supported_lang(lang: Language) -> bool {
        T::supported_lang(lang) && ClockTime::supported_lang(lang)
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Hour1to12
            | NumericContent::Hour0to23
            | NumericContent::Minute
            | NumericContent::Second => self.clone().time_of_day().fmt_numeric(n, opt),
            _ => self.clone().date().fmt_numeric(n, opt),
        }
    }

    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        match t {
            TextContent::HalfDayName | TextContent::HalfDayAbbrev => {
                self.clone().time_of_day().fmt_text(t, lang, opt)
            }
            _ => self.clone().date().fmt_text(t, lang, opt),
        }
    }
}

impl<T: PresetDisplay + Clone + DisplayMomentItem> PresetDisplay for CalendarMoment<T> {
    fn long_date(&self) -> String {
        self.clone().date().long_date()
    }

    fn short_date(&self) -> String {
        self.clone().date().short_date()
    }
}

impl<T: fmt::Display + PresetDisplay + Clone + DisplayMomentItem> fmt::Display
    for CalendarMoment<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.preset_str(Language::EN, HHMMSS_COLON));
        self.clone().date().fmt(f)
    }
}

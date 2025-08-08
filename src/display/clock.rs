use crate::clock::ClockTime;
use crate::clock::TimeOfDay;
use crate::common::math::TermNum;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::HHMMSS_COLON;
use crate::display::private::fmt_number;
use crate::display::private::fmt_string;
use crate::display::private::get_dict;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::NumericContent;
use crate::display::private::TextContent;
use crate::display::text::prelude::Language;
use std::fmt;

impl DisplayItem for ClockTime {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).common_clock.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Hour1to12 => {
                fmt_number((self.hours as i64).adjusted_remainder(12), opt)
            }
            NumericContent::Hour0to23 => fmt_number(self.hours as i16, opt),
            NumericContent::Minute => fmt_number(self.minutes as i16, opt),
            NumericContent::Second => fmt_number(self.seconds as i16, opt),
            _ => "".to_string(),
        }
    }

    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        let dict_opt = get_dict(lang).common_clock.as_ref();
        let before_noon = self.hours < 12;
        match (t, dict_opt, before_noon) {
            (TextContent::HalfDayName, Some(dict), true) => fmt_string(dict.am_full, opt),
            (TextContent::HalfDayName, Some(dict), false) => fmt_string(dict.pm_full, opt),
            (TextContent::HalfDayAbbrev, Some(dict), true) => fmt_string(dict.am_abr, opt),
            (TextContent::HalfDayAbbrev, Some(dict), false) => fmt_string(dict.pm_abr, opt),
            (_, _, _) => "".to_string(),
        }
    }
}

impl PresetDisplay for ClockTime {}

impl fmt::Display for ClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.preset_str(Language::EN, HHMMSS_COLON))
    }
}

impl DisplayItem for TimeOfDay {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).common_clock.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        self.to_clock().fmt_numeric(n, opt)
    }

    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        self.to_clock().fmt_text(t, lang, opt)
    }
}

impl PresetDisplay for TimeOfDay {}

impl fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.preset_str(Language::EN, HHMMSS_COLON))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_languages() {
        assert!(TimeOfDay::supported_lang(Language::EN));
        assert!(TimeOfDay::supported_lang(Language::FR));
    }
}

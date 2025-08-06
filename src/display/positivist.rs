use crate::calendar::HasIntercalaryDays;
use crate::calendar::Perennial;
use crate::calendar::Positivist;
use crate::calendar::ToFromCommonDate;
use crate::calendar::ToFromOrdinalDate;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::LONG_COMPL;
use crate::display::prelude::LONG_DATE;
use crate::display::private::fmt_days_since_epoch;
use crate::display::private::fmt_number;
use crate::display::private::fmt_quarter;
use crate::display::private::fmt_seconds_since_epoch;
use crate::display::private::fmt_string;
use crate::display::private::get_dict;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::NumericContent;
use crate::display::private::TextContent;
use crate::display::text::prelude::Language;
use std::fmt;

impl DisplayItem for Positivist {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).positivist.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month | NumericContent::DayOfMonth | NumericContent::Year => {
                self.to_common_date().fmt_numeric(n, opt)
            }
            NumericContent::DayOfWeek => match self.weekday() {
                Some(d) => d.fmt_numeric(n, opt),
                None => "".to_string(),
            },
            NumericContent::DayOfYear => self.to_ordinal().fmt_numeric(n, opt),
            NumericContent::Hour1to12
            | NumericContent::Hour0to23
            | NumericContent::Minute
            | NumericContent::Second => self.convert::<TimeOfDay>().fmt_numeric(n, opt),
            NumericContent::SecondsSinceEpoch => fmt_seconds_since_epoch(*self, opt),
            NumericContent::Quarter => fmt_quarter(*self, opt),
            NumericContent::DaysSinceEpoch => fmt_days_since_epoch(*self, opt),
            NumericContent::ComplementaryDay => match self.complementary() {
                Some(d) => fmt_number(d as i8, opt),
                None => "".to_string(),
            },
            NumericContent::WeekOfYear => match self.try_week_of_year() {
                Some(w) => fmt_number(w as i8, opt),
                None => "".to_string(),
            },
        }
    }
    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        match (t, get_dict(lang).positivist.as_ref()) {
            (TextContent::MonthName, Some(dict)) => {
                let months: [&str; 13] = [
                    dict.moses,
                    dict.homer,
                    dict.aristotle,
                    dict.archimedes,
                    dict.caesar,
                    dict.saint_paul,
                    dict.charlemagne,
                    dict.dante,
                    dict.gutenburg,
                    dict.shakespeare,
                    dict.descartes,
                    dict.frederick,
                    dict.bichat,
                ];
                let name = match self.try_month() {
                    Some(m) => months[(m as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            (TextContent::DayOfMonthName, _) => fmt_string("", opt),
            (TextContent::DayOfWeekName, _) => match self.weekday() {
                Some(m) => m.fmt_text(t, lang, opt),
                None => fmt_string("", opt),
            },
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, _) => {
                self.convert::<TimeOfDay>().fmt_text(t, lang, opt)
            }
            (TextContent::EraName, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_crisis_full, opt)
                } else {
                    fmt_string(dict.after_crisis_full, opt)
                }
            }
            (TextContent::EraAbbreviation, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_crisis_abr, opt)
                } else {
                    fmt_string(dict.after_crisis_abr, opt)
                }
            }
            (TextContent::ComplementaryDayName, Some(dict)) => {
                let compl: [&str; 2] = [dict.festival_of_dead, dict.festival_of_holy_women];
                let name = match self.complementary() {
                    Some(d) => compl[(d as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            (_, _) => String::from(""),
        }
    }
}

impl PresetDisplay for Positivist {
    fn long_date(&self) -> String {
        if self.complementary().is_some() {
            self.preset_str(Language::EN, LONG_COMPL)
        } else {
            self.preset_str(Language::EN, LONG_DATE)
        }
    }
}

impl fmt::Display for Positivist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_languages() {
        assert!(Positivist::supported_lang(Language::EN));
        assert!(Positivist::supported_lang(Language::FR));
    }
}

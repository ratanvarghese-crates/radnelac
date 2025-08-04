use crate::calendar::CommonWeekOfYear;
use crate::calendar::Coptic;
use crate::calendar::ToFromCommonDate;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::display::prelude::PresetDisplay;
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
//use crate::calendar::CopticMonth;

impl DisplayItem for Coptic {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).coptic.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month | NumericContent::DayOfMonth | NumericContent::Year => {
                self.to_common_date().fmt_numeric(n, opt)
            }
            NumericContent::DayOfWeek => self.convert::<Weekday>().fmt_numeric(n, opt),
            NumericContent::Hour1to12
            | NumericContent::Hour0to23
            | NumericContent::Minute
            | NumericContent::Second => self.convert::<TimeOfDay>().fmt_numeric(n, opt),
            NumericContent::SecondsSinceEpoch => fmt_seconds_since_epoch(*self, opt),
            NumericContent::Quarter => fmt_quarter(*self, opt),
            NumericContent::DaysSinceEpoch => fmt_days_since_epoch(*self, opt),
            NumericContent::ComplementaryDay => String::from(""),
            NumericContent::WeekOfYear => fmt_number(self.week_of_year() as i16, opt),
        }
    }

    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        match (t, get_dict(lang).coptic.as_ref()) {
            (TextContent::MonthName, Some(dict)) => {
                let months: [&str; 13] = [
                    dict.thoout,
                    dict.paope,
                    dict.athor,
                    dict.koiak,
                    dict.tobe,
                    dict.meshir,
                    dict.paremotep,
                    dict.parmoute,
                    dict.pashons,
                    dict.paone,
                    dict.epep,
                    dict.mesore,
                    dict.epagomene,
                ];
                let name = months[self.to_common_date().month as usize - 1];
                fmt_string(name, opt)
            }
            (TextContent::DayOfWeekName, _) => self.convert::<Weekday>().fmt_text(t, lang, opt),
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, _) => {
                self.convert::<TimeOfDay>().fmt_text(t, lang, opt)
            }
            (TextContent::EraName, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_martyrs_full, opt)
                } else {
                    fmt_string(dict.after_martyrs_full, opt)
                }
            }
            (TextContent::EraAbbreviation, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_martyrs_abr, opt)
                } else {
                    fmt_string(dict.after_martyrs_abr, opt)
                }
            }
            (_, _) => String::from(""),
        }
    }
}

impl PresetDisplay for Coptic {}

impl fmt::Display for Coptic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_languages() {
        assert!(Coptic::supported_lang(Language::EN));
    }
}

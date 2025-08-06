use crate::calendar::ISO;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::YEAR_WEEK_DAY;
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

impl DisplayItem for ISO {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).iso.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month | NumericContent::DayOfMonth => "".to_string(),
            NumericContent::Year => fmt_number(self.year(), opt),
            NumericContent::DayOfWeek => fmt_number(self.day_num() as i8, opt),
            NumericContent::DayOfYear => "".to_string(),
            NumericContent::Hour1to12
            | NumericContent::Hour0to23
            | NumericContent::Minute
            | NumericContent::Second => self.convert::<TimeOfDay>().fmt_numeric(n, opt),
            NumericContent::SecondsSinceEpoch => fmt_seconds_since_epoch(*self, opt),
            NumericContent::Quarter => fmt_quarter(*self, opt),
            NumericContent::DaysSinceEpoch => fmt_days_since_epoch(*self, opt),
            NumericContent::ComplementaryDay => "".to_string(),
            NumericContent::WeekOfYear => fmt_number(self.week().get() as i8, opt),
        }
    }
    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        match (t, get_dict(lang).iso.as_ref()) {
            (TextContent::DayOfWeekName, _) => self.day().fmt_text(t, lang, opt),
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, _) => {
                self.convert::<TimeOfDay>().fmt_text(t, lang, opt)
            }
            (TextContent::EraName, Some(dict)) => {
                if self.year() < 0 {
                    fmt_string(dict.before_epoch_full, opt)
                } else {
                    fmt_string(dict.after_epoch_full, opt)
                }
            }
            (TextContent::EraAbbreviation, Some(dict)) => {
                if self.year() < 0 {
                    fmt_string(dict.before_epoch_abr, opt)
                } else {
                    fmt_string(dict.after_epoch_abr, opt)
                }
            }
            (_, _) => String::from(""),
        }
    }
}

impl PresetDisplay for ISO {
    fn long_date(&self) -> String {
        self.preset_str(Language::EN, YEAR_WEEK_DAY)
    }

    fn short_date(&self) -> String {
        self.preset_str(Language::EN, YEAR_WEEK_DAY)
    }
}

impl fmt::Display for ISO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::CommonDate;
    use crate::calendar::Gregorian;
    use crate::calendar::ToFromCommonDate;

    #[test]
    fn expected_languages() {
        assert!(ISO::supported_lang(Language::EN));
    }

    #[test]
    fn w1() {
        let dg = Gregorian::try_from_common_date(CommonDate::new(2007, 1, 1)).unwrap();
        let di = dg.convert::<ISO>();
        let s = di.short_date();
        assert_eq!(&s, "2007-W01-1")
    }
}

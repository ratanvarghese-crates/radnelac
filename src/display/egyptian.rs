use crate::calendar::CommonWeekOfYear;
use crate::calendar::Egyptian;
use crate::calendar::HasIntercalaryDays;
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
use crate::display::private::NumericContent;
use crate::display::text::prelude::Language;
use crate::display::LONG_COMPL;
use crate::display::LONG_DATE;
use std::fmt;

use crate::display::private::TextContent;

use crate::display::private::DisplayOptions;

impl DisplayItem for Egyptian {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).egyptian.as_ref().is_some()
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
            NumericContent::ComplementaryDay => match self.complementary() {
                Some(d) => fmt_number(d as i16, opt),
                None => "".to_string(),
            },
            NumericContent::WeekOfYear => fmt_number(self.week_of_year() as i16, opt),
        }
    }
    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        match (t, get_dict(lang).egyptian.as_ref()) {
            (TextContent::MonthName, Some(dict)) => {
                let months: [&str; 13] = [
                    dict.thoth,
                    dict.phaophi,
                    dict.athyr,
                    dict.choiak,
                    dict.tybi,
                    dict.mechir,
                    dict.phamenoth,
                    dict.pharmuthi,
                    dict.pachon,
                    dict.payni,
                    dict.epiphi,
                    dict.mesori,
                    dict.epagomenae,
                ];
                let m = self.to_common_date().month;
                let name = months[m as usize - 1];
                fmt_string(name, opt)
            }
            (TextContent::DayOfMonthName, _) => fmt_string("", opt),
            (TextContent::DayOfWeekName, _) => self.convert::<Weekday>().fmt_text(t, lang, opt),
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, _) => {
                self.convert::<TimeOfDay>().fmt_text(t, lang, opt)
            }
            (TextContent::EraName, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_nabonassar_full, opt)
                } else {
                    fmt_string(dict.after_nabonassar_full, opt)
                }
            }
            (TextContent::EraAbbreviation, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_nabonassar_abr, opt)
                } else {
                    fmt_string(dict.after_nabonassar_abr, opt)
                }
            }
            (TextContent::ComplementaryDayName, Some(dict)) => {
                // https://helda.helsinki.fi/server/api/core/bitstreams/4ed34849-903b-416b-97d6-a9a76eb1fb1d/content
                let days: [&str; 5] = [
                    dict.birth_of_osiris,
                    dict.birth_of_horus,
                    dict.birth_of_seth,
                    dict.birth_of_isis,
                    dict.birth_of_nephthys,
                ];
                match self.complementary() {
                    Some(d) => fmt_string(days[d as usize - 1], opt),
                    None => fmt_string("", opt),
                }
            }
            (_, _) => fmt_string("", opt),
        }
    }
}

impl PresetDisplay for Egyptian {
    fn long_date(&self) -> String {
        let p = match self.complementary() {
            None => LONG_DATE,
            Some(_) => LONG_COMPL,
        };
        self.preset_str(Language::EN, p)
    }
}

impl fmt::Display for Egyptian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::CommonWeekOfYear;
use crate::calendar::Julian;
use crate::calendar::ToFromCommonDate;
use crate::calendar::ToFromOrdinalDate;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::display::moment::DisplayMomentItem;
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

impl DisplayItem for Julian {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).julian.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month | NumericContent::DayOfMonth | NumericContent::Year => {
                self.to_common_date().fmt_numeric(n, opt)
            }
            NumericContent::DayOfWeek => self.convert::<Weekday>().fmt_numeric(n, opt),
            NumericContent::DayOfYear => self.to_ordinal().fmt_numeric(n, opt),
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
        match (t, get_dict(lang).julian.as_ref()) {
            (TextContent::MonthName, Some(dict)) => {
                let months: [&str; 12] = [
                    dict.january,
                    dict.february,
                    dict.march,
                    dict.april,
                    dict.may,
                    dict.june,
                    dict.july,
                    dict.august,
                    dict.september,
                    dict.october,
                    dict.november,
                    dict.december,
                ];
                let name = months[self.to_common_date().month as usize - 1];
                fmt_string(name, opt)
            }
            (TextContent::DayOfMonthName, _) => fmt_string("", opt),
            (TextContent::DayOfWeekName, _) => self.convert::<Weekday>().fmt_text(t, lang, opt),
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, _) => {
                self.convert::<TimeOfDay>().fmt_text(t, lang, opt)
            }
            (TextContent::EraName, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_christ_full, opt)
                } else {
                    fmt_string(dict.anno_domini_full, opt)
                }
            }
            (TextContent::EraAbbreviation, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_christ_abr, opt)
                } else {
                    fmt_string(dict.anno_domini_abr, opt)
                }
            }
            (_, _) => String::from(""),
        }
    }
}

impl PresetDisplay for Julian {}

impl fmt::Display for Julian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

impl DisplayMomentItem for Julian {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_languages() {
        assert!(Julian::supported_lang(Language::EN));
        assert!(Julian::supported_lang(Language::FR));
    }
}

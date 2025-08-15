// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::Cotsworth;
use crate::calendar::HasEpagemonae;
use crate::calendar::Perennial;
use crate::calendar::ToFromCommonDate;
use crate::calendar::ToFromOrdinalDate;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
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
use crate::display::LONG_COMPL;
use crate::display::LONG_DATE;
use std::fmt;

impl DisplayItem for Cotsworth {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).cotsworth.as_ref().is_some()
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
            NumericContent::ComplementaryDay => match self.epagomenae() {
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
        match (t, get_dict(lang).cotsworth.as_ref()) {
            (TextContent::MonthName, Some(dict)) => {
                let months: [&str; 13] = [
                    dict.january,
                    dict.february,
                    dict.march,
                    dict.april,
                    dict.may,
                    dict.june,
                    dict.sol,
                    dict.july,
                    dict.august,
                    dict.september,
                    dict.october,
                    dict.november,
                    dict.december,
                ];
                let name = match self.try_month() {
                    Some(m) => months[(m as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            (TextContent::DayOfWeekName, _) => match self.weekday() {
                Some(m) => m.fmt_text(t, lang, opt),
                None => fmt_string("", opt),
            },
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, _) => {
                self.convert::<TimeOfDay>().fmt_text(t, lang, opt)
            }
            (TextContent::EraName, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_epoch_full, opt)
                } else {
                    fmt_string(dict.after_epoch_full, opt)
                }
            }
            (TextContent::EraAbbreviation, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_epoch_abr, opt)
                } else {
                    fmt_string(dict.after_epoch_abr, opt)
                }
            }
            (TextContent::ComplementaryDayName, Some(dict)) => {
                let compl: [&str; 2] = [dict.year_day, dict.leap_day];
                let name = match self.epagomenae() {
                    Some(d) => compl[(d as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            (_, _) => fmt_string("", opt),
        }
    }
}

impl PresetDisplay for Cotsworth {
    fn long_date(&self) -> String {
        //Avoid non-existent weekday
        self.preset_str(Language::EN, LONG_DATE).trim().to_string()
    }
}

impl fmt::Display for Cotsworth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

impl DisplayMomentItem for Cotsworth {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_languages() {
        assert!(Cotsworth::supported_lang(Language::EN));
        assert!(Cotsworth::supported_lang(Language::FR));
    }
}

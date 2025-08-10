// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::Armenian;
use crate::calendar::CommonWeekOfYear;
use crate::calendar::HasIntercalaryDays;
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
use crate::display::private::NumericContent;
use crate::display::text::prelude::Language;
use crate::display::LONG_DATE;
use crate::display::LONG_DAY_OF_MONTH;
use crate::display::YYYYMMDD_DASH;
use std::fmt;

use crate::display::private::TextContent;

use crate::display::private::DisplayOptions;

impl DisplayItem for Armenian {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).armenian.as_ref().is_some()
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
            NumericContent::ComplementaryDay => "".to_string(),
            NumericContent::WeekOfYear => fmt_number(self.week_of_year() as i16, opt),
        }
    }
    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        //https://en.wikipedia.org/wiki/Armenian_calendar
        match (t, get_dict(lang).armenian.as_ref()) {
            (TextContent::MonthName, Some(dict)) => {
                let months: [&str; 13] = [
                    dict.nawasardi,
                    dict.hori,
                    dict.sahmi,
                    dict.tre,
                    dict.kaloch,
                    dict.arach,
                    dict.mehekani,
                    dict.areg,
                    dict.ahekani,
                    dict.mareri,
                    dict.margach,
                    dict.hrotich,
                    dict.aweleac,
                ];
                let m = self.to_common_date().month;
                let name = months[m as usize - 1];
                fmt_string(name, opt)
            }
            (TextContent::DayOfMonthName, Some(dict)) => {
                let days: [&str; 30] = [
                    dict.areg_day,
                    dict.hrand,
                    dict.aram,
                    dict.margar,
                    dict.ahrank,
                    dict.mazdel,
                    dict.astlik,
                    dict.mihr,
                    dict.jopaber,
                    dict.murc,
                    dict.erezhan,
                    dict.ani,
                    dict.parkhar,
                    dict.vanat,
                    dict.aramazd,
                    dict.mani,
                    dict.asak,
                    dict.masis,
                    dict.anahit,
                    dict.aragats,
                    dict.gorgor,
                    dict.kordvik,
                    dict.tsmak,
                    dict.lusnak,
                    dict.tsron,
                    dict.npat,
                    dict.vahagn,
                    dict.sim,
                    dict.varag,
                    dict.giseravar,
                ];
                match self.day_name() {
                    Some(d) => fmt_string(days[d as usize - 1], opt),
                    None => fmt_string("", opt),
                }
            }
            (TextContent::DayOfWeekName, Some(_)) => {
                self.convert::<Weekday>().fmt_text(t, lang, opt)
            }
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, Some(_)) => {
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
            (_, _) => fmt_string("", opt),
        }
    }
}

impl PresetDisplay for Armenian {
    fn long_date(&self) -> String {
        let p = match self.complementary() {
            None => LONG_DAY_OF_MONTH,
            Some(_) => LONG_DATE,
        };
        self.preset_str(Language::EN, p)
    }
}

impl fmt::Display for Armenian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

impl DisplayMomentItem for Armenian {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_languages() {
        assert!(Armenian::supported_lang(Language::EN));
    }
}

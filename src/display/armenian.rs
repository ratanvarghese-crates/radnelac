use crate::calendar::Armenian;
use crate::clock::TimeOfDay;
use crate::common::date::CommonWeekOfYear;
use crate::common::date::ToFromCommonDate;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::display::preset_fmt::PresetDisplay;
use crate::display::private::fmt_days_since_epoch;
use crate::display::private::fmt_number;
use crate::display::private::fmt_quarter;
use crate::display::private::fmt_seconds_since_epoch;
use crate::display::private::fmt_string;
use crate::display::private::DisplayItem;
use crate::display::private::NumericContent;
use std::fmt;

use crate::display::private::TextContent;

use crate::display::private::DisplayOptions;

impl DisplayItem for Armenian {
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
            NumericContent::ComplementaryDay => "".to_string(),
            NumericContent::WeekOfYear => fmt_number(self.week_of_year() as i16, opt),
        }
    }
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        //https://en.wikipedia.org/wiki/Armenian_calendar
        match t {
            TextContent::MonthName => {
                const MONTHS: [&str; 12] = [
                    "Nawasard",
                    "Hoṙi",
                    "Sahmi",
                    "Trē",
                    "Kʿałocʿ",
                    "Aracʿ",
                    "Mehekan",
                    "Areg",
                    "Ahekan",
                    "Mareri",
                    "Margacʿ",
                    "Hroticʿ",
                ];
                let m = self.to_common_date().month;
                if m > 12 {
                    fmt_string("", opt)
                } else {
                    let name = MONTHS[m as usize - 1];
                    fmt_string(name, opt)
                }
            }
            TextContent::DayOfMonthName => {
                const DAYS: [&str; 30] = [
                    "Areg",
                    "Hrand",
                    "Aram",
                    "Margar",
                    "Ahrank’",
                    "Mazdeł",
                    "Astłik",
                    "Mihr",
                    "Jopaber",
                    "Murç",
                    "Erezhan",
                    "Ani",
                    "Parkhar",
                    "Vanat",
                    "Aramazd",
                    "Mani",
                    "Asak",
                    "Masis",
                    "Anahit",
                    "Aragats",
                    "Gorgor",
                    "Kordvik",
                    "Tsmak",
                    "Lusnak",
                    "Tsrōn",
                    "Npat",
                    "Vahagn",
                    "Sim",
                    "Varag",
                    "Gišeravar",
                ];
                match self.day_name() {
                    Some(d) => fmt_string(DAYS[d as usize - 1], opt),
                    None => fmt_string("", opt),
                }
            }
            TextContent::DayOfWeekName => self.convert::<Weekday>().fmt_text(t, opt),
            TextContent::HalfDayName | TextContent::HalfDayAbbrev => {
                self.convert::<TimeOfDay>().fmt_text(t, opt)
            }
            TextContent::EraName => {
                if self.to_common_date().year < 0 {
                    fmt_string("Before Armenian Era", opt)
                } else {
                    fmt_string("Armenian Era", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.to_common_date().year < 0 {
                    fmt_string("BAE", opt)
                } else {
                    fmt_string("AE", opt)
                }
            }
            TextContent::ComplementaryDayName => fmt_string("", opt),
        }
    }
}

impl PresetDisplay for Armenian {}

impl fmt::Display for Armenian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

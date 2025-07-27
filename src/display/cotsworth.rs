use crate::calendar::Cotsworth;
use crate::calendar::HasIntercalaryDays;
use crate::calendar::Perennial;
use crate::calendar::ToFromCommonDate;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
use crate::display::prelude::PresetDisplay;
use crate::display::private::fmt_days_since_epoch;
use crate::display::private::fmt_number;
use crate::display::private::fmt_quarter;
use crate::display::private::fmt_seconds_since_epoch;
use crate::display::private::fmt_string;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::NumericContent;
use crate::display::private::TextContent;
use std::fmt;

impl DisplayItem for Cotsworth {
    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month | NumericContent::DayOfMonth | NumericContent::Year => {
                self.to_common_date().fmt_numeric(n, opt)
            }
            NumericContent::DayOfWeek => match self.weekday() {
                Some(d) => d.fmt_numeric(n, opt),
                None => "".to_string(),
            },
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
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        match t {
            TextContent::MonthName => {
                const MONTHS: [&str; 13] = [
                    "January",
                    "February",
                    "March",
                    "April",
                    "May",
                    "June",
                    "Sol",
                    "July",
                    "August",
                    "September",
                    "October",
                    "November",
                    "December",
                ];
                let name = match self.try_month() {
                    Some(m) => MONTHS[(m as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            TextContent::DayOfMonthName => fmt_string("", opt),
            TextContent::DayOfWeekName => match self.weekday() {
                Some(m) => m.fmt_text(t, opt),
                None => fmt_string("", opt),
            },
            TextContent::HalfDayName | TextContent::HalfDayAbbrev => {
                self.convert::<TimeOfDay>().fmt_text(t, opt)
            }
            TextContent::EraName => {
                if self.to_common_date().year < 0 {
                    fmt_string("Before Cotsworth Era", opt)
                } else {
                    fmt_string("Cotsworth Era", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.to_common_date().year < 0 {
                    fmt_string("BCE", opt)
                } else {
                    fmt_string("CE", opt)
                }
            }
            TextContent::ComplementaryDayName => {
                const COMPL: [&str; 2] = ["Year Day", "Leap Day"];
                let name = match self.complementary() {
                    Some(d) => COMPL[(d as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
        }
    }
}

impl PresetDisplay for Cotsworth {}

impl fmt::Display for Cotsworth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

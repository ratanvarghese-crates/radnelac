use crate::calendar::Positivist;
use crate::clock::TimeOfDay;
use crate::calendar::ComplementaryWeekOfYear;
use crate::calendar::PerennialWithComplementaryDay;
use crate::calendar::ToFromCommonDate;
use crate::calendar::TryMonth;
use crate::day_count::ToFixed;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::LONG_COMPL;
use crate::display::prelude::LONG_DATE;
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

impl DisplayItem for Positivist {
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
                    "Moses",
                    "Homer",
                    "Aristotle",
                    "Archimedes",
                    "Caesar",
                    "Saint Paul",
                    "Charlemagne",
                    "Dante",
                    "Gutenburg",
                    "Shakespeare",
                    "Descartes",
                    "Frederick",
                    "Bichat",
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
                    fmt_string("Before the Great Crisis", opt)
                } else {
                    fmt_string("After the Great Crisis", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.to_common_date().year < 0 {
                    fmt_string("BGC", opt)
                } else {
                    fmt_string("AGC", opt)
                }
            }
            TextContent::ComplementaryDayName => {
                const COMPL: [&str; 2] = ["Festival Of The Dead", "Festival Of Holy Women"];
                let name = match self.complementary() {
                    Some(d) => COMPL[(d as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
        }
    }
}

impl PresetDisplay for Positivist {
    fn long_date(&self) -> String {
        if self.complementary().is_some() {
            self.preset_str(LONG_COMPL)
        } else {
            self.preset_str(LONG_DATE)
        }
    }
}

impl fmt::Display for Positivist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

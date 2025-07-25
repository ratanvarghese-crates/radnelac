use crate::calendar::Egyptian;
use crate::clock::TimeOfDay;
use crate::calendar::CommonWeekOfYear;
use crate::calendar::ToFromCommonDate;
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

impl DisplayItem for Egyptian {
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
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        match t {
            TextContent::MonthName => {
                const MONTHS: [&str; 12] = [
                    "Thoth",
                    "Phaophi",
                    "Athyr",
                    "Choiak",
                    "Tybi",
                    "Mechir",
                    "Phamenoth",
                    "Pharmuthi",
                    "Pachon",
                    "Payni",
                    "Epiphi",
                    "Mesori",
                ];
                let m = self.to_common_date().month;
                if m > 12 {
                    fmt_string("", opt)
                } else {
                    let name = MONTHS[m as usize - 1];
                    fmt_string(name, opt)
                }
            }
            TextContent::DayOfMonthName => fmt_string("", opt),
            TextContent::DayOfWeekName => self.convert::<Weekday>().fmt_text(t, opt),
            TextContent::HalfDayName | TextContent::HalfDayAbbrev => {
                self.convert::<TimeOfDay>().fmt_text(t, opt)
            }
            TextContent::EraName => {
                if self.to_common_date().year < 0 {
                    fmt_string("Before Nabonassar Era", opt)
                } else {
                    fmt_string("Nabonassar Era", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.to_common_date().year < 0 {
                    fmt_string("BNE", opt)
                } else {
                    fmt_string("NE", opt)
                }
            }
            TextContent::ComplementaryDayName => {
                // https://helda.helsinki.fi/server/api/core/bitstreams/4ed34849-903b-416b-97d6-a9a76eb1fb1d/content
                const DAYS: [&str; 5] = [
                    "Birth of Osiris",
                    "Birth of Horus",
                    "Birth of Seth",
                    "Birth of Isis",
                    "Birth of Nephthys",
                ];
                match self.complementary() {
                    Some(d) => fmt_string(DAYS[d as usize - 1], opt),
                    None => fmt_string("", opt),
                }
            }
        }
    }
}

impl PresetDisplay for Egyptian {}

impl fmt::Display for Egyptian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

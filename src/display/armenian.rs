use crate::calendar::Armenian;
use crate::clock::TimeOfDay;
use crate::common::date::CommonDate;
use crate::common::date::ToFromCommonDate;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::display::common::fmt_days_since_epoch;
use crate::display::common::fmt_number;
use crate::display::common::fmt_seconds_since_epoch;
use crate::display::common::fmt_string;
use crate::display::common::Content;
use crate::display::common::DisplayItem;
use crate::display::common::Item;
use crate::display::common::Locale;
use crate::display::common::NumericContent;
use crate::display::common::Sign;
use std::fmt;

use crate::display::common::TextContent;

use crate::display::common::DisplayOptions;

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
            NumericContent::Quarter => {
                let m = self.to_common_date().month as i16;
                if m == 13 {
                    fmt_number(4, opt)
                } else {
                    fmt_number((m / 4) + 1, opt)
                }
            }
            NumericContent::DaysSinceEpoch => fmt_days_since_epoch(*self, opt),
            NumericContent::ComplementaryDay => "".to_string(),
            NumericContent::WeekOfYear => {
                let today = self.to_fixed();
                let start =
                    Self::try_from_common_date(CommonDate::new(self.to_common_date().year, 1, 1))
                        .expect("New year should be valid for any date")
                        .to_fixed();
                fmt_number(((today.get_day_i() - start.get_day_i()) / 7) + 1, opt)
            }
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

impl fmt::Display for Armenian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const O: DisplayOptions = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        const ITEMS: [Item<'_>; 11] = [
            Item::new(Content::Text(TextContent::MonthName), O),
            Item::new(Content::Text(TextContent::ComplementaryDayName), O),
            Item::new(Content::Literal(" "), O),
            Item::new(Content::Text(TextContent::DayOfMonthName), O),
            Item::new(Content::Literal(" "), O),
            Item::new(Content::Literal("("), O),
            Item::new(Content::Numeric(NumericContent::DayOfMonth), O),
            Item::new(Content::Literal("), "), O),
            Item::new(Content::Numeric(NumericContent::Year), O),
            Item::new(Content::Literal(" "), O),
            Item::new(Content::Text(TextContent::EraName), O),
        ];
        for item in ITEMS {
            write!(f, "{}", self.fmt_item(item))?;
        }
        Ok(())
    }
}

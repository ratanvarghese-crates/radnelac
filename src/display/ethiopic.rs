use crate::calendar::Ethiopic;
use crate::clock::TimeOfDay;
use crate::common::date::CommonWeekOfYear;
use crate::common::date::ToFromCommonDate;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::display::private::fmt_days_since_epoch;
use crate::display::private::fmt_number;
use crate::display::private::fmt_quarter;
use crate::display::private::fmt_seconds_since_epoch;
use crate::display::private::fmt_string;
use crate::format::Content;
use crate::format::DisplayItem;
use crate::format::DisplayOptions;
use crate::format::Item;
use crate::format::Locale;
use crate::format::NumericContent;
use crate::format::Sign;
use crate::format::TextContent;
use std::fmt;
//use crate::calendar::EthiopicMonth;

impl DisplayItem for Ethiopic {
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
            NumericContent::ComplementaryDay => String::from(""),
            NumericContent::WeekOfYear => fmt_number(self.week_of_year() as i16, opt),
        }
    }

    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        match t {
            TextContent::MonthName => {
                const MONTHS: [&str; 13] = [
                    "Mäskäräm",
                    "Ṭəqəmt",
                    "Ḫədar",
                    "Taḫśaś",
                    "Ṭərr",
                    "Yäkatit",
                    "Mägabit",
                    "Miyazya",
                    "Gənbo",
                    "Säne",
                    "Ḥamle",
                    "Nähase",
                    "Ṗagʷəmen",
                ];
                let name = MONTHS[self.to_common_date().month as usize - 1];
                fmt_string(name, opt)
            }
            TextContent::DayOfMonthName => fmt_string("", opt),
            TextContent::DayOfWeekName => self.convert::<Weekday>().fmt_text(t, opt),
            TextContent::HalfDayName | TextContent::HalfDayAbbrev => {
                self.convert::<TimeOfDay>().fmt_text(t, opt)
            }
            TextContent::EraName => {
                if self.to_common_date().year < 0 {
                    fmt_string("Before Incarnation Era", opt)
                } else {
                    fmt_string("Incarnation Era", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.to_common_date().year < 0 {
                    fmt_string("BD", opt)
                } else {
                    fmt_string("AM", opt)
                }
            }
            TextContent::ComplementaryDayName => String::from(""),
        }
    }
}

impl fmt::Display for Ethiopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const O: DisplayOptions = DisplayOptions {
            numerals: None,
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        const ITEMS: [Item<'_>; 7] = [
            Item::new(Content::Text(TextContent::MonthName), O),
            Item::new(Content::Literal(" "), O),
            Item::new(Content::Numeric(NumericContent::DayOfMonth), O),
            Item::new(Content::Literal(", "), O),
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

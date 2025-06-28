use crate::calendar::Cotsworth;
use crate::clock::TimeOfDay;
use crate::common::date::ComplementaryWeekOfYear;
use crate::common::date::PerennialWithComplementaryDay;
use crate::common::date::ToFromCommonDate;
use crate::common::date::TryMonth;
use crate::day_count::ToFixed;
use crate::display::private::fmt_days_since_epoch;
use crate::display::private::fmt_number;
use crate::display::private::fmt_quarter;
use crate::display::private::fmt_seconds_since_epoch;
use crate::display::private::fmt_string;
use crate::display::private::Content;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::Item;
use crate::display::private::Locale;
use crate::display::private::NumericContent;
use crate::display::private::Sign;
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
                const COMPLEMENTARY: [&str; 2] = ["Year Day", "Leap Day"];
                let name = match self.complementary() {
                    Some(d) => COMPLEMENTARY[(d as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
        }
    }
}

impl fmt::Display for Cotsworth {
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
        if self.complementary().is_some() {
            const ITEMS_COMPLEMENTARY: [Item<'_>; 5] = [
                Item::new(Content::Text(TextContent::ComplementaryDayName), O),
                Item::new(Content::Literal(", "), O),
                Item::new(Content::Numeric(NumericContent::Year), O),
                Item::new(Content::Literal(" "), O),
                Item::new(Content::Text(TextContent::EraName), O),
            ];
            for item in ITEMS_COMPLEMENTARY {
                write!(f, "{}", self.fmt_item(item))?;
            }
        } else {
            const ITEMS_COMMON: [Item<'_>; 9] = [
                Item::new(Content::Text(TextContent::DayOfWeekName), O),
                Item::new(Content::Literal(" "), O),
                Item::new(Content::Text(TextContent::MonthName), O),
                Item::new(Content::Literal(" "), O),
                Item::new(Content::Numeric(NumericContent::DayOfMonth), O),
                Item::new(Content::Literal(", "), O),
                Item::new(Content::Numeric(NumericContent::Year), O),
                Item::new(Content::Literal(" "), O),
                Item::new(Content::Text(TextContent::EraName), O),
            ];
            for item in ITEMS_COMMON {
                write!(f, "{}", self.fmt_item(item))?;
            }
        }
        Ok(())
    }
}

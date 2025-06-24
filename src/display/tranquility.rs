use crate::calendar::TranquilityComplementaryDay;
use crate::calendar::TranquilityMoment;
use crate::clock::TimeOfDay;
use crate::common::date::PerennialWithComplementaryDay;
use crate::common::date::ToFromCommonDate;
use crate::common::date::TryMonth;
use crate::day_count::ToFixed;
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

impl DisplayItem for TranquilityMoment {
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
            NumericContent::WeekOfYear => {
                let w: i8 = match self.try_month() {
                    Some(month) => ((((month as i8) - 1) * 28) + (self.day() as i8) - 1) / 7 + 1,
                    None => match self.complementary() {
                        Some(TranquilityComplementaryDay::MoonLandingDay) => 53,
                        Some(TranquilityComplementaryDay::ArmstrongDay) => 53,
                        Some(TranquilityComplementaryDay::AldrinDay) => 48,
                        None => panic!("Non-complementary day without month"),
                    },
                };
                fmt_number(w, opt)
            }
        }
    }
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        match t {
            TextContent::MonthName => {
                const MONTHS: [&str; 13] = [
                    "Archimedes",
                    "Brahe",
                    "Copernicus",
                    "Darwin",
                    "Einstein",
                    "Faraday",
                    "Galileo",
                    "Hippocrates",
                    "Imhotep",
                    "Jung",
                    "Kepler",
                    "Lavoisier",
                    "Mendel",
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
                if self.is_after_tranquility() {
                    fmt_string("After Tranquility", opt)
                } else {
                    fmt_string("Before Tranquility", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.is_after_tranquility() {
                    fmt_string("AT", opt)
                } else {
                    fmt_string("BT", opt)
                }
            }
            TextContent::ComplementaryDayName => {
                const COMPLEMENTARY: [&str; 3] =
                    ["Moon Landing Day", "Armstrong Day", "Aldrin Day"];
                let name = match self.complementary() {
                    Some(d) => COMPLEMENTARY[d as usize],
                    None => "",
                };
                fmt_string(name, opt)
            }
        }
    }
}

impl fmt::Display for TranquilityMoment {
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

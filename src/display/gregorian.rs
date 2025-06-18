use crate::calendar::Gregorian;
use crate::clock::ClockTime;
use crate::clock::TimeOfDay;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::ToFromCommonDate;
use crate::common::math::TermNum;
use crate::day_count::Epoch;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::display::common::fmt_number;
use crate::display::common::fmt_string;
use crate::display::common::Case;
use crate::display::common::Content;
use crate::display::common::DisplayItem;
use crate::display::common::DisplayOptions;
use crate::display::common::Item;
use crate::display::common::Locale;
use crate::display::common::NumericContent;
use crate::display::common::Sign;
use crate::display::common::TextContent;
use std::fmt;
//use crate::calendar::GregorianMonth;

impl DisplayItem for Gregorian {
    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month => fmt_number(self.to_common_date().month as i16, opt),
            NumericContent::DayOfWeek => fmt_number(self.convert::<Weekday>() as i16, opt),
            NumericContent::DayOfMonth => fmt_number(self.to_common_date().day as i16, opt),
            NumericContent::Hour1to12 => fmt_number(
                (ClockTime::new(self.convert::<TimeOfDay>()).hours as i64).adjusted_remainder(12),
                opt,
            ),
            NumericContent::Hour0to23 => fmt_number(
                ClockTime::new(self.convert::<TimeOfDay>()).hours as i16,
                opt,
            ),
            NumericContent::Minute => fmt_number(
                ClockTime::new(self.convert::<TimeOfDay>()).minutes as i16,
                opt,
            ),
            NumericContent::Second => fmt_number(
                ClockTime::new(self.convert::<TimeOfDay>()).seconds as i16,
                opt,
            ),
            NumericContent::SecondsSinceEpoch => fmt_number(
                ((self.to_fixed().get() - Gregorian::epoch().get()) * (24.0 * 60.0 * 60.0)) as i16,
                opt,
            ),
            NumericContent::Year => fmt_number(self.to_common_date().year, opt),
            NumericContent::Quarter => {
                fmt_number(((self.to_common_date().month as i16) / 4) + 1, opt)
            }
            NumericContent::DaysSinceEpoch => fmt_number(
                self.to_fixed().get_day_i() - Gregorian::epoch().get_day_i(),
                opt,
            ),
            NumericContent::ComplementaryDay => String::from(""),
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
        match t {
            TextContent::MonthName => {
                const MONTHS: [&str; 12] = [
                    "January",
                    "February",
                    "March",
                    "April",
                    "May",
                    "June",
                    "July",
                    "August",
                    "September",
                    "October",
                    "November",
                    "December",
                ];
                let name = MONTHS[self.to_common_date().month as usize - 1];
                fmt_string(name, opt)
            }
            TextContent::DayOfWeekName => {
                const DAYS: [&str; 7] = [
                    "Sunday",
                    "Monday",
                    "Tuesday",
                    "Wednesday",
                    "Thursday",
                    "Friday",
                    "Saturday",
                ];
                let name = DAYS[self.convert::<Weekday>() as usize];
                fmt_string(name, opt)
            }
            TextContent::HalfDayName => {
                if self.convert::<TimeOfDay>() < TimeOfDay::new(0.5) {
                    fmt_string("Ante Meridiem", opt)
                } else {
                    fmt_string("Post Meridiem", opt)
                }
            }
            TextContent::HalfDayAbbrev => {
                if self.convert::<TimeOfDay>() < TimeOfDay::new(0.5) {
                    fmt_string("AM", opt)
                } else {
                    fmt_string("PM", opt)
                }
            }
            TextContent::EraName => {
                if self.to_common_date().year < 0 {
                    fmt_string("Before Common Era", opt)
                } else {
                    fmt_string("Common Era", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.to_common_date().year < 0 {
                    fmt_string("BCE", opt)
                } else {
                    fmt_string("CE", opt)
                }
            }
            TextContent::ComplementaryDayName => String::from(""),
        }
    }
}

impl fmt::Display for Gregorian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const O3: DisplayOptions = DisplayOptions {
            width: Some(3),
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        const O: DisplayOptions = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        const ITEMS: [Item<'_>; 7] = [
            Item::new(Content::Text(TextContent::MonthName), O3),
            Item::new(Content::Literal(" "), O),
            Item::new(Content::Numeric(NumericContent::DayOfMonth), O),
            Item::new(Content::Literal(", "), O),
            Item::new(Content::Numeric(NumericContent::Year), O),
            Item::new(Content::Literal(" "), O),
            Item::new(Content::Text(TextContent::EraAbbreviation), O),
        ];
        for item in ITEMS {
            write!(f, "{}", self.fmt_item(item))?;
        }
        Ok(())
    }
}

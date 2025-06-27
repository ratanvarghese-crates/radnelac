use crate::calendar::ISO;
use crate::clock::TimeOfDay;
use crate::common::date::WeekOfYear;
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

impl DisplayItem for ISO {
    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month | NumericContent::DayOfMonth => "".to_string(),
            NumericContent::Year => fmt_number(self.year(), opt),
            NumericContent::DayOfWeek => fmt_number(self.day_num() as i8, opt),
            NumericContent::Hour1to12
            | NumericContent::Hour0to23
            | NumericContent::Minute
            | NumericContent::Second => self.convert::<TimeOfDay>().fmt_numeric(n, opt),
            NumericContent::SecondsSinceEpoch => fmt_seconds_since_epoch(*self, opt),
            NumericContent::Quarter => fmt_quarter(*self, opt),
            NumericContent::DaysSinceEpoch => fmt_days_since_epoch(*self, opt),
            NumericContent::ComplementaryDay => "".to_string(),
            NumericContent::WeekOfYear => fmt_number(self.week_of_year() as i8, opt),
        }
    }
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        match t {
            TextContent::MonthName
            | TextContent::DayOfMonthName
            | TextContent::ComplementaryDayName => fmt_string("", opt),
            TextContent::DayOfWeekName => self.day().fmt_text(t, opt),
            TextContent::HalfDayName | TextContent::HalfDayAbbrev => {
                self.convert::<TimeOfDay>().fmt_text(t, opt)
            }
            TextContent::EraName => {
                if self.year() < 0 {
                    fmt_string("Before ISO Era", opt)
                } else {
                    fmt_string("ISO Era", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.year() < 0 {
                    fmt_string("BIE", opt)
                } else {
                    fmt_string("IE", opt)
                }
            }
        }
    }
}

impl fmt::Display for ISO {
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
        const ITEMS: [Item<'_>; 5] = [
            Item::new(Content::Numeric(NumericContent::Year), O),
            Item::new(Content::Literal("-W"), O),
            Item::new(Content::Numeric(NumericContent::WeekOfYear), O),
            Item::new(Content::Literal("-"), O),
            Item::new(Content::Numeric(NumericContent::DayOfWeek), O),
        ];
        for item in ITEMS {
            write!(f, "{}", self.fmt_item(item))?;
        }
        Ok(())
    }
}

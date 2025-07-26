use crate::calendar::Holocene;
use crate::clock::TimeOfDay;
use crate::calendar::CommonWeekOfYear;
use crate::calendar::ToFromCommonDate;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::YYYYYMMDD_DASH;
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
//use crate::calendar::HoloceneMonth;

impl DisplayItem for Holocene {
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
            TextContent::DayOfMonthName => fmt_string("", opt),
            TextContent::DayOfWeekName => self.convert::<Weekday>().fmt_text(t, opt),
            TextContent::HalfDayName | TextContent::HalfDayAbbrev => {
                self.convert::<TimeOfDay>().fmt_text(t, opt)
            }
            TextContent::EraName => {
                if self.to_common_date().year < 0 {
                    fmt_string("Before Human Era", opt)
                } else {
                    fmt_string("Human Era", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.to_common_date().year < 0 {
                    fmt_string("BHE", opt)
                } else {
                    fmt_string("HE", opt)
                }
            }
            TextContent::ComplementaryDayName => String::from(""),
        }
    }
}

impl PresetDisplay for Holocene {
    fn short_date(&self) -> String {
        self.preset_str(YYYYYMMDD_DASH)
    }
}

impl fmt::Display for Holocene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::CommonDate;

    #[test]
    fn long_date() {
        let d_list = [
            (
                CommonDate::new(11582, 10, 15),
                "Friday October 15, 11582 Human Era",
            ),
            (
                CommonDate::new(12012, 12, 21),
                "Friday December 21, 12012 Human Era",
            ),
            (
                CommonDate::new(12025, 1, 1),
                "Wednesday January 1, 12025 Human Era",
            ),
            (
                CommonDate::new(12025, 6, 29),
                "Sunday June 29, 12025 Human Era",
            ),
            (
                CommonDate::new(12025, 6, 30),
                "Monday June 30, 12025 Human Era",
            ),
            (
                CommonDate::new(12025, 7, 1),
                "Tuesday July 1, 12025 Human Era",
            ),
        ];

        for item in d_list {
            let d = Holocene::try_from_common_date(item.0).unwrap();
            let s = d.long_date();
            assert_eq!(s, item.1);
        }
    }

    #[test]
    fn short_date() {
        let d_list = [
            (CommonDate::new(1582, 10, 15), "01582-10-15"),
            (CommonDate::new(2012, 12, 21), "02012-12-21"),
            (CommonDate::new(2025, 1, 1), "02025-01-01"),
            (CommonDate::new(2025, 6, 29), "02025-06-29"),
            (CommonDate::new(2025, 6, 30), "02025-06-30"),
            (CommonDate::new(2025, 7, 1), "02025-07-01"),
            (CommonDate::new(11582, 10, 15), "11582-10-15"),
            (CommonDate::new(12012, 12, 21), "12012-12-21"),
            (CommonDate::new(12025, 1, 1), "12025-01-01"),
            (CommonDate::new(12025, 6, 29), "12025-06-29"),
            (CommonDate::new(12025, 6, 30), "12025-06-30"),
            (CommonDate::new(12025, 7, 1), "12025-07-01"),
        ];

        for item in d_list {
            let d = Holocene::try_from_common_date(item.0).unwrap();
            let s = d.short_date();
            assert_eq!(s, item.1);
        }
    }
}

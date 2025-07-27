use crate::calendar::PerennialWithComplementaryDay;
use crate::calendar::ToFromCommonDate;
use crate::calendar::TranquilityComplementaryDay;
use crate::calendar::TranquilityMoment;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::COMPL_ONLY;
use crate::display::prelude::LONG_COMPL;
use crate::display::prelude::LONG_DATE;
use crate::display::prelude::YEAR_COMPL;
use crate::display::prelude::YEAR_MDD;
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
                const COMPL: [&str; 3] = ["Moon Landing Day", "Armstrong Day", "Aldrin Day"];
                let name = match self.complementary() {
                    Some(d) => COMPL[d as usize],
                    None => "",
                };
                fmt_string(name, opt)
            }
        }
    }
}

impl PresetDisplay for TranquilityMoment {
    fn long_date(&self) -> String {
        let p = match self.complementary() {
            None => LONG_DATE,
            Some(TranquilityComplementaryDay::MoonLandingDay) => COMPL_ONLY,
            Some(_) => LONG_COMPL,
        };
        self.preset_str(p)
    }

    fn short_date(&self) -> String {
        let p = match self.complementary() {
            None => YEAR_MDD,
            Some(_) => YEAR_COMPL,
        };
        self.preset_str(p)
    }
}

impl fmt::Display for TranquilityMoment {
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
            (CommonDate::new(0, 0, 0), "Moon Landing Day"),
            (
                CommonDate::new(1, 1, 1),
                "Friday Archimedes 1, 1 After Tranquility",
            ),
            (
                CommonDate::new(2, 2, 2),
                "Saturday Brahe 2, 2 After Tranquility",
            ),
            (
                CommonDate::new(3, 3, 3),
                "Sunday Copernicus 3, 3 After Tranquility",
            ),
            (
                CommonDate::new(4, 4, 4),
                "Monday Darwin 4, 4 After Tranquility",
            ),
            (
                CommonDate::new(5, 5, 5),
                "Tuesday Einstein 5, 5 After Tranquility",
            ),
            (
                CommonDate::new(6, 6, 6),
                "Wednesday Faraday 6, 6 After Tranquility",
            ),
            (
                CommonDate::new(7, 7, 7),
                "Thursday Galileo 7, 7 After Tranquility",
            ),
            (
                CommonDate::new(8, 8, 8),
                "Friday Hippocrates 8, 8 After Tranquility",
            ),
            (
                CommonDate::new(9, 9, 9),
                "Saturday Imhotep 9, 9 After Tranquility",
            ),
            (
                CommonDate::new(10, 10, 10),
                "Sunday Jung 10, 10 After Tranquility",
            ),
            (
                CommonDate::new(11, 11, 11),
                "Monday Kepler 11, 11 After Tranquility",
            ),
            (
                CommonDate::new(12, 12, 12),
                "Tuesday Lavoisier 12, 12 After Tranquility",
            ),
            (
                CommonDate::new(-1, 13, 28),
                "Thursday Mendel 28, 1 Before Tranquility",
            ),
            (
                CommonDate::new(-2, 0, 1),
                "Armstrong Day, 2 Before Tranquility",
            ),
            (
                CommonDate::new(31, 8, 27),
                "Wednesday Hippocrates 27, 31 After Tranquility",
            ),
            (
                CommonDate::new(31, 0, 2),
                "Aldrin Day, 31 After Tranquility",
            ),
            (
                CommonDate::new(31, 8, 28),
                "Thursday Hippocrates 28, 31 After Tranquility",
            ),
            (
                CommonDate::new(56, 4, 1),
                "Friday Darwin 1, 56 After Tranquility",
            ),
        ];
        for item in d_list {
            let d = TranquilityMoment::try_from_common_date(item.0).unwrap();
            let s = d.long_date();
            assert_eq!(s, item.1);
        }
    }

    #[test]
    fn short_date() {
        let d_list = [
            (CommonDate::new(0, 0, 0), "0-MOO"),
            (CommonDate::new(1, 1, 1), "1-A01"),
            (CommonDate::new(2, 2, 2), "2-B02"),
            (CommonDate::new(3, 3, 3), "3-C03"),
            (CommonDate::new(4, 4, 4), "4-D04"),
            (CommonDate::new(5, 5, 5), "5-E05"),
            (CommonDate::new(6, 6, 6), "6-F06"),
            (CommonDate::new(7, 7, 7), "7-G07"),
            (CommonDate::new(8, 8, 8), "8-H08"),
            (CommonDate::new(9, 9, 9), "9-I09"),
            (CommonDate::new(10, 10, 10), "10-J10"),
            (CommonDate::new(11, 11, 11), "11-K11"),
            (CommonDate::new(12, 12, 12), "12-L12"),
            (CommonDate::new(-1, 13, 28), "-1-M28"),
            (CommonDate::new(-2, 0, 1), "-2-ARM"),
            (CommonDate::new(31, 8, 27), "31-H27"),
            (CommonDate::new(31, 0, 2), "31-ALD"),
            (CommonDate::new(31, 8, 28), "31-H28"),
            (CommonDate::new(56, 4, 1), "56-D01"),
        ];
        for item in d_list {
            let d = TranquilityMoment::try_from_common_date(item.0).unwrap();
            let s = d.short_date();
            assert_eq!(s, item.1);
        }
    }
}

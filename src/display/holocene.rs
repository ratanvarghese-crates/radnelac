use crate::calendar::CommonWeekOfYear;
use crate::calendar::Holocene;
use crate::calendar::ToFromCommonDate;
use crate::calendar::ToFromOrdinalDate;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::YYYYYMMDD_DASH;
use crate::display::private::fmt_days_since_epoch;
use crate::display::private::fmt_number;
use crate::display::private::fmt_quarter;
use crate::display::private::fmt_seconds_since_epoch;
use crate::display::private::fmt_string;
use crate::display::private::get_dict;
use crate::display::private::DisplayItem;
use crate::display::moment::DisplayMomentItem;
use crate::display::private::DisplayOptions;
use crate::display::private::NumericContent;
use crate::display::private::TextContent;
use crate::display::text::prelude::Language;
use std::fmt;
//use crate::calendar::HoloceneMonth;

impl DisplayItem for Holocene {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).holocene.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month | NumericContent::DayOfMonth | NumericContent::Year => {
                self.to_common_date().fmt_numeric(n, opt)
            }
            NumericContent::DayOfWeek => self.convert::<Weekday>().fmt_numeric(n, opt),
            NumericContent::DayOfYear => self.to_ordinal().fmt_numeric(n, opt),
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

    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        match (t, get_dict(lang).holocene.as_ref()) {
            (TextContent::MonthName, Some(dict)) => {
                let months: [&str; 12] = [
                    dict.january,
                    dict.february,
                    dict.march,
                    dict.april,
                    dict.may,
                    dict.june,
                    dict.july,
                    dict.august,
                    dict.september,
                    dict.october,
                    dict.november,
                    dict.december,
                ];
                let name = months[self.to_common_date().month as usize - 1];
                fmt_string(name, opt)
            }
            (TextContent::DayOfMonthName, _) => fmt_string("", opt),
            (TextContent::DayOfWeekName, _) => self.convert::<Weekday>().fmt_text(t, lang, opt),
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, _) => {
                self.convert::<TimeOfDay>().fmt_text(t, lang, opt)
            }
            (TextContent::EraName, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_human_era_full, opt)
                } else {
                    fmt_string(dict.human_era_full, opt)
                }
            }
            (TextContent::EraAbbreviation, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_human_era_abr, opt)
                } else {
                    fmt_string(dict.human_era_abr, opt)
                }
            }
            (_, _) => String::from(""),
        }
    }
}

impl PresetDisplay for Holocene {
    fn short_date(&self) -> String {
        self.preset_str(Language::EN, YYYYYMMDD_DASH)
    }
}

impl fmt::Display for Holocene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

impl DisplayMomentItem for Holocene {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::CommonDate;

    #[test]
    fn expected_languages() {
        assert!(Holocene::supported_lang(Language::EN));
        assert!(Holocene::supported_lang(Language::FR));
    }

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

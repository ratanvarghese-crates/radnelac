use crate::calendar::FrenchRevArith;
use crate::calendar::HasIntercalaryDays;
use crate::calendar::Perennial;
use crate::calendar::ToFromCommonDate;
use crate::clock::TimeOfDay;
use crate::day_count::ToFixed;
use crate::display::prelude::PresetDisplay;
use crate::display::prelude::LONG_COMPL;
use crate::display::prelude::LONG_DATE;
use crate::display::private::fmt_days_since_epoch;
use crate::display::private::fmt_number;
use crate::display::private::fmt_quarter;
use crate::display::private::fmt_seconds_since_epoch;
use crate::display::private::fmt_string;
use crate::display::private::get_dict;
use crate::display::private::DisplayItem;
use crate::display::private::NumericContent;
use crate::display::private::TextContent;
use crate::display::text::prelude::Language;
use std::fmt;

use crate::display::private::DisplayOptions;

impl<const L: bool> DisplayItem for FrenchRevArith<L> {
    fn supported_lang(lang: Language) -> bool {
        get_dict(lang).french_rev.as_ref().is_some()
    }

    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month | NumericContent::DayOfMonth | NumericContent::Year => {
                self.to_common_date().fmt_numeric(n, opt)
            }
            NumericContent::DayOfWeek => match self.weekday() {
                Some(d) => fmt_number(d as i8, opt),
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
    fn fmt_text(&self, t: TextContent, lang: Language, opt: DisplayOptions) -> String {
        match (t, get_dict(lang).french_rev.as_ref()) {
            (TextContent::MonthName, Some(dict)) => {
                let months: [&str; 12] = [
                    dict.vendemiaire,
                    dict.brumaire,
                    dict.frimaire,
                    dict.nivose,
                    dict.pluviose,
                    dict.ventose,
                    dict.germinal,
                    dict.floreal,
                    dict.prairial,
                    dict.messidor,
                    dict.thermidor,
                    dict.fructidor,
                ];
                let name = match self.try_month() {
                    Some(m) => months[(m as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            (TextContent::DayOfMonthName, _) => fmt_string("", opt),
            (TextContent::DayOfWeekName, Some(dict)) => {
                let weekdays: [&str; 10] = [
                    dict.primidi,
                    dict.duodi,
                    dict.tridi,
                    dict.quartidi,
                    dict.quintidi,
                    dict.sextidi,
                    dict.septidi,
                    dict.octidi,
                    dict.nonidi,
                    dict.decadi,
                ];
                let name = match self.weekday() {
                    Some(m) => weekdays[(m as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            (TextContent::HalfDayName | TextContent::HalfDayAbbrev, _) => {
                self.convert::<TimeOfDay>().fmt_text(t, lang, opt)
            }
            (TextContent::EraName, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_republic_full, opt)
                } else {
                    fmt_string(dict.after_republic_full, opt)
                }
            }
            (TextContent::EraAbbreviation, Some(dict)) => {
                if self.to_common_date().year < 0 {
                    fmt_string(dict.before_republic_abr, opt)
                } else {
                    fmt_string(dict.after_republic_abr, opt)
                }
            }
            (TextContent::ComplementaryDayName, Some(dict)) => {
                let sansculottides: [&str; 6] = [
                    dict.fete_de_la_vertu,
                    dict.fete_du_genie,
                    dict.fete_du_travail,
                    dict.fete_de_lopinion,
                    dict.fete_des_recompenses,
                    dict.fete_de_la_revolution,
                ];
                let name = match self.complementary() {
                    Some(d) => sansculottides[(d as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            (_, _) => fmt_string("", opt),
        }
    }
}

impl<const L: bool> PresetDisplay for FrenchRevArith<L> {
    fn long_date(&self) -> String {
        if self.complementary().is_some() {
            self.preset_str(Language::EN, LONG_COMPL)
        } else {
            self.preset_str(Language::EN, LONG_DATE)
        }
    }
}

impl<const L: bool> fmt::Display for FrenchRevArith<L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long_date())
    }
}

// TODO: Seperate EN and FR strings

use crate::calendar::FrenchRevArith;
use crate::clock::TimeOfDay;
use crate::common::date::CommonDay;
use crate::common::date::PerennialWithComplementaryDay;
use crate::common::date::ToFromCommonDate;
use crate::common::date::TryMonth;
use crate::day_count::ToFixed;
use crate::display::common::fmt_days_since_epoch;
use crate::display::common::fmt_number;
use crate::display::common::fmt_quarter;
use crate::display::common::fmt_seconds_since_epoch;
use crate::display::common::fmt_string;
use crate::display::common::Content;
use crate::display::common::DisplayItem;
use crate::display::common::Item;
use crate::display::common::Locale;
use crate::display::common::NumericContent;
use crate::display::common::Sign;
use crate::display::common::TextContent;
use std::fmt;

use crate::display::common::DisplayOptions;

impl<const L: bool> DisplayItem for FrenchRevArith<L> {
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
            NumericContent::WeekOfYear => {
                let w: i8 = match self.try_month() {
                    Some(month) => ((((month as i8) - 1) * 30) + (self.day() as i8) - 1) / 10 + 1,
                    None => 37,
                };
                fmt_number(w, opt)
            }
        }
    }
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String {
        match t {
            TextContent::MonthName => {
                const MONTHS: [&str; 12] = [
                    "Vendémiaire",
                    "Brumaire",
                    "Frimaire",
                    "Nivôse",
                    "Pluviôse",
                    "Ventôse",
                    "Germinal",
                    "Floréal",
                    "Prairial",
                    "Messidor",
                    "Thermidor",
                    "Fructidor",
                ];
                let name = match self.try_month() {
                    Some(m) => MONTHS[(m as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            TextContent::DayOfMonthName => fmt_string("", opt),
            TextContent::DayOfWeekName => {
                const WEEKDAYS: [&str; 10] = [
                    "Primidi", "Duodi", "Tridi", "Quartidi", "Quintidi", "Sextidi", "Septidi",
                    "Octidi", "Nonidi", "Décadi",
                ];
                let name = match self.weekday() {
                    Some(m) => WEEKDAYS[(m as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
            TextContent::HalfDayName | TextContent::HalfDayAbbrev => {
                self.convert::<TimeOfDay>().fmt_text(t, opt)
            }
            TextContent::EraName => {
                if self.to_common_date().year < 0 {
                    fmt_string("Before Republican Era", opt)
                } else {
                    fmt_string("Republican Era", opt)
                }
            }
            TextContent::EraAbbreviation => {
                if self.to_common_date().year < 0 {
                    fmt_string("BRE", opt)
                } else {
                    fmt_string("RE", opt)
                }
            }
            TextContent::ComplementaryDayName => {
                const SANSCULOTTIDES: [&str; 6] = [
                    "La Fête de la Vertu",
                    "La Fête du Génie",
                    "La Fête du Travail",
                    "La Fête de l'Opinion",
                    "La Fête des Récompenses",
                    "La Fête de la Révolution",
                ];
                let name = match self.complementary() {
                    Some(d) => SANSCULOTTIDES[(d as usize) - 1],
                    None => "",
                };
                fmt_string(name, opt)
            }
        }
    }
}

impl<const L: bool> fmt::Display for FrenchRevArith<L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const O: DisplayOptions = DisplayOptions {
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

use crate::display::private::Case;
use crate::display::private::Content;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::Item;
use crate::display::private::NumericContent;
use crate::display::private::Sign;
use crate::display::private::TextContent;
pub use crate::display::text::prelude::Language;

const O_LITERAL: DisplayOptions = DisplayOptions {
    numerals: None,
    width: None,
    align: None,
    padding: None,
    case: None,
    sign: Sign::OnlyNegative,
};

const O_YEAR_IN_ERA: DisplayOptions = DisplayOptions {
    numerals: None,
    width: None,
    align: None,
    padding: None,
    case: None,
    sign: Sign::Never,
};

const O_N1: DisplayOptions = DisplayOptions {
    numerals: None,
    width: Some(1),
    align: None,
    padding: None,
    case: Some(Case::Upper),
    sign: Sign::Never,
};

const O_N2: DisplayOptions = DisplayOptions {
    numerals: None,
    width: Some(2),
    align: None,
    padding: Some('0'),
    case: Some(Case::Upper),
    sign: Sign::Never,
};

const O_N3: DisplayOptions = DisplayOptions {
    numerals: None,
    width: Some(3),
    align: None,
    padding: None,
    case: Some(Case::Upper),
    sign: Sign::Never,
};

macro_rules! NumericDateItems {
    ($name: ident, $sep: literal, $prefix: ident, $n_prefix: literal, $stem: ident, $n_stem: literal, $suffix: ident, $n_suffix: literal) => {
        const $name: [Item<'_>; 5] = [
            Item::new(
                Content::Numeric(NumericContent::$prefix),
                DisplayOptions {
                    numerals: None,
                    width: Some($n_prefix),
                    align: None,
                    padding: Some('0'),
                    case: None,
                    sign: Sign::OnlyNegative,
                },
            ),
            Item::new(Content::Literal($sep), O_LITERAL),
            Item::new(
                Content::Numeric(NumericContent::$stem),
                DisplayOptions {
                    numerals: None,
                    width: Some($n_stem),
                    align: None,
                    padding: Some('0'),
                    case: None,
                    sign: Sign::OnlyNegative,
                },
            ),
            Item::new(Content::Literal($sep), O_LITERAL),
            Item::new(
                Content::Numeric(NumericContent::$suffix),
                DisplayOptions {
                    numerals: None,
                    width: Some($n_suffix),
                    align: None,
                    padding: Some('0'),
                    case: None,
                    sign: Sign::OnlyNegative,
                },
            ),
        ];
    };
}

const I_HHMM_COLON_AMPM: [Item<'_>; 5] = [
    Item::new(
        Content::Numeric(NumericContent::Hour1to12),
        DisplayOptions {
            numerals: None,
            width: Some(2),
            align: None,
            padding: Some('0'),
            case: None,
            sign: Sign::OnlyNegative,
        },
    ),
    Item::new(Content::Literal(":"), O_LITERAL),
    Item::new(
        Content::Numeric(NumericContent::Minute),
        DisplayOptions {
            numerals: None,
            width: Some(2),
            align: None,
            padding: Some('0'),
            case: None,
            sign: Sign::OnlyNegative,
        },
    ),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::HalfDayAbbrev), O_LITERAL),
];

NumericDateItems!(I_HHMMSS_COLON, ":", Hour0to23, 2, Minute, 2, Second, 2);
NumericDateItems!(I_YYYYMMDD_DASH, "-", Year, 4, Month, 2, DayOfMonth, 2);
NumericDateItems!(I_YYYYYMMDD_DASH, "-", Year, 5, Month, 2, DayOfMonth, 2);
NumericDateItems!(I_YYYYMMDD_SLASH, "/", Year, 4, Month, 2, DayOfMonth, 2);
NumericDateItems!(I_DDMMYYYY_SLASH, "/", DayOfMonth, 2, Month, 2, Year, 4);
NumericDateItems!(I_DDMMYYYY_DOT, ".", DayOfMonth, 2, Month, 2, Year, 4);
NumericDateItems!(I_MMDDYYYY_SLASH, "/", Month, 2, DayOfMonth, 2, Year, 4);

const I_LONG_DATE: [Item<'_>; 9] = [
    Item::new(Content::Text(TextContent::DayOfWeekName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::MonthName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::DayOfMonth), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_YEAR_IN_ERA),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraName), O_LITERAL),
];

const I_LONG_DAY_OF_MONTH: [Item<'_>; 9] = [
    Item::new(Content::Text(TextContent::DayOfWeekName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::MonthName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::DayOfMonthName), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_YEAR_IN_ERA),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraName), O_LITERAL),
];

const I_LONG_COMPL: [Item<'_>; 5] = [
    Item::new(Content::Text(TextContent::ComplementaryDayName), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_YEAR_IN_ERA),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraName), O_LITERAL),
];

const I_LONG_DATE_ERA_ABBR: [Item<'_>; 9] = [
    Item::new(Content::Text(TextContent::DayOfWeekName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::MonthName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::DayOfMonth), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_YEAR_IN_ERA),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraAbbreviation), O_LITERAL),
];

const I_LONG_DAY_OF_MONTH_ERA_ABBR: [Item<'_>; 9] = [
    Item::new(Content::Text(TextContent::DayOfWeekName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::MonthName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::DayOfMonthName), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_YEAR_IN_ERA),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraAbbreviation), O_LITERAL),
];

const I_LONG_COMPL_ERA_ABBR: [Item<'_>; 5] = [
    Item::new(Content::Text(TextContent::ComplementaryDayName), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_YEAR_IN_ERA),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraAbbreviation), O_LITERAL),
];

const I_YEAR_WEEK_DAY: [Item<'_>; 5] = [
    Item::new(Content::Numeric(NumericContent::Year), O_LITERAL),
    Item::new(Content::Literal("-W"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::WeekOfYear), O_N2),
    Item::new(Content::Literal("-"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::DayOfWeek), O_N1),
];

const I_YEAR_MDD: [Item<'_>; 4] = [
    Item::new(Content::Numeric(NumericContent::Year), O_LITERAL),
    Item::new(Content::Literal("-"), O_LITERAL),
    Item::new(Content::Text(TextContent::MonthName), O_N1),
    Item::new(Content::Numeric(NumericContent::DayOfMonth), O_N2),
];

const I_YEAR_COMPL: [Item<'_>; 3] = [
    Item::new(Content::Numeric(NumericContent::Year), O_LITERAL),
    Item::new(Content::Literal("-"), O_LITERAL),
    Item::new(Content::Text(TextContent::ComplementaryDayName), O_N3),
];

const I_COMPL_ONLY: [Item<'_>; 1] = [Item::new(
    Content::Text(TextContent::ComplementaryDayName),
    O_LITERAL,
)];

const I_WEEKDAY_NAME_ONLY: [Item<'_>; 1] = [Item::new(
    Content::Text(TextContent::DayOfWeekName),
    O_LITERAL,
)];

const I_EPOCH_SECONDS_ONLY: [Item<'_>; 1] = [Item::new(
    Content::Numeric(NumericContent::SecondsSinceEpoch),
    O_LITERAL,
)];

const I_EPOCH_DAYS_ONLY: [Item<'_>; 1] = [Item::new(
    Content::Numeric(NumericContent::DaysSinceEpoch),
    O_LITERAL,
)];

/// Represents a date format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PresetFormat<'a>(&'a [Item<'a>]);

/// HH:MM AM/PM time format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const HHMM_COLON_AMPM: PresetFormat<'static> = PresetFormat::<'static>(&I_HHMM_COLON_AMPM);
/// HH:MM:SS time format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const HHMMSS_COLON: PresetFormat<'static> = PresetFormat::<'static>(&I_HHMMSS_COLON);
/// YYYY-MM-DD numeric date format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const YYYYMMDD_DASH: PresetFormat<'static> = PresetFormat::<'static>(&I_YYYYMMDD_DASH);
/// YYYYY-MM-DD numeric date format
///
/// This is intended for the Holocene Calendar.
///
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const YYYYYMMDD_DASH: PresetFormat<'static> = PresetFormat::<'static>(&I_YYYYYMMDD_DASH);
/// YYYY/MM/DD numeric date format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const YYYYMMDD_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&I_YYYYMMDD_SLASH);
/// DD/MM/YYYY numeric date format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const DDMMYYYY_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&I_DDMMYYYY_SLASH);
/// DD.MM.YYYY numeric date format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const DDMMYYYY_DOT: PresetFormat<'static> = PresetFormat::<'static>(&I_DDMMYYYY_DOT);
/// MM/DD/YYYY numeric date format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const MMDDYYYY_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&I_MMDDYYYY_SLASH);
/// Calendar-specific long date format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const LONG_DATE: PresetFormat<'static> = PresetFormat::<'static>(&I_LONG_DATE);
/// Calendar-specific long date format with day of month name
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const LONG_DAY_OF_MONTH: PresetFormat<'static> = PresetFormat::<'static>(&I_LONG_DAY_OF_MONTH);
/// Calendar-specific long complementary day format
///
/// This is intended for calendars with complementary days.
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const LONG_COMPL: PresetFormat<'static> = PresetFormat::<'static>(&I_LONG_COMPL);
/// Calendar-specific long date format, with abbreviated era
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const LONG_DATE_ERA_ABBR: PresetFormat<'static> =
    PresetFormat::<'static>(&I_LONG_DATE_ERA_ABBR);
/// Calendar-specific long date format with day of month name and abbreviated era
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const LONG_DAY_OF_MONTH_ERA_ABBR: PresetFormat<'static> =
    PresetFormat::<'static>(&I_LONG_DAY_OF_MONTH_ERA_ABBR);
/// Calendar-specific long complementary day format, with abbreviated era
///
/// This is intended for calendars with complementary days.
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const LONG_COMPL_ERA_ABBR: PresetFormat<'static> =
    PresetFormat::<'static>(&I_LONG_COMPL_ERA_ABBR);
/// YYYY-Www-DD alphanumeric date format
///
/// This is inteded for the ISO calendar.
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const YEAR_WEEK_DAY: PresetFormat<'static> = PresetFormat::<'static>(&I_YEAR_WEEK_DAY);
/// Y-mDD alphanumeric date format, where Y has variable length, m is a single character
///
/// This is intended for the Tranquility calendar
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const YEAR_MDD: PresetFormat<'static> = PresetFormat::<'static>(&I_YEAR_MDD);
/// Y-CCC alphanumeric date format, where Y has variable length, CCC is the complementary day.
///
/// This is intended for calendars with complementary days.
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const YEAR_COMPL: PresetFormat<'static> = PresetFormat::<'static>(&I_YEAR_COMPL);
/// Format which is the full name of the complementary day
///
/// This is intended for calendars with complementary days.
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const COMPL_ONLY: PresetFormat<'static> = PresetFormat::<'static>(&I_COMPL_ONLY);
/// Format which is the full name of the weekday
///
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const WEEKDAY_NAME_ONLY: PresetFormat<'static> = PresetFormat::<'static>(&I_WEEKDAY_NAME_ONLY);
/// Format which is the seconds since an epoch only
///
/// The epoch is specific to the timekeeping system.
///
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const EPOCH_SECONDS_ONLY: PresetFormat<'static> =
    PresetFormat::<'static>(&I_EPOCH_SECONDS_ONLY);
/// Format which is the days since an epoch only
///
/// The epoch is specific to the timekeeping system.
///
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub const EPOCH_DAYS_ONLY: PresetFormat<'static> = PresetFormat::<'static>(&I_EPOCH_DAYS_ONLY);

/// Format a date in a preset format
/// ## Crate Features
///
/// This is only available if `display` is enabled.
pub trait PresetDisplay: DisplayItem {
    /// Checks if language is supported
    fn supported_display_lang(lang: Language) -> bool {
        Self::supported_lang(lang)
    }

    /// Format a date in any `PresetFormat`
    fn preset_str(&self, lang: Language, preset: PresetFormat) -> String {
        let mut result = String::new();
        for item in preset.0 {
            result.push_str(&self.fmt_item(lang, *item))
        }
        result
    }

    /// Format a date in a calendar-specific long format
    fn long_date(&self) -> String {
        self.preset_str(Language::EN, LONG_DATE)
    }

    /// Format a date in a calendar-specific short format
    fn short_date(&self) -> String {
        self.preset_str(Language::EN, YYYYMMDD_DASH)
    }
}

use crate::display::private::Case;
use crate::display::private::Content;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::Item;
use crate::display::private::Locale;
use crate::display::private::NumericContent;
use crate::display::private::Sign;
use crate::display::private::TextContent;

const O_LITERAL: DisplayOptions = DisplayOptions {
    numerals: None,
    width: None,
    align: None,
    padding: None,
    case: None,
    sign: Sign::OnlyNegative,
    locale: Locale::en_CA,
};

const O_YEAR_IN_ERA: DisplayOptions = DisplayOptions {
    numerals: None,
    width: None,
    align: None,
    padding: None,
    case: None,
    sign: Sign::Never,
    locale: Locale::en_CA,
};

const O_N1: DisplayOptions = DisplayOptions {
    numerals: None,
    width: Some(1),
    align: None,
    padding: None,
    case: Some(Case::Upper),
    sign: Sign::Never,
    locale: Locale::en_CA,
};

const O_N2: DisplayOptions = DisplayOptions {
    numerals: None,
    width: Some(2),
    align: None,
    padding: Some('0'),
    case: Some(Case::Upper),
    sign: Sign::Never,
    locale: Locale::en_CA,
};

const O_N3: DisplayOptions = DisplayOptions {
    numerals: None,
    width: Some(3),
    align: None,
    padding: None,
    case: Some(Case::Upper),
    sign: Sign::Never,
    locale: Locale::en_CA,
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
                    locale: Locale::en_CA,
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
                    locale: Locale::en_CA,
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
                    locale: Locale::en_CA,
                },
            ),
        ];
    };
}

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

const I_LONG_COMPL: [Item<'_>; 5] = [
    Item::new(Content::Text(TextContent::ComplementaryDayName), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_YEAR_IN_ERA),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraName), O_LITERAL),
];

const I_YEAR_WEEK_DAY: [Item<'_>; 5] = [
    Item::new(Content::Numeric(NumericContent::Year), O_LITERAL),
    Item::new(Content::Literal("-W"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::WeekOfYear), O_LITERAL),
    Item::new(Content::Literal("-"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::DayOfWeek), O_LITERAL),
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PresetFormat<'a>(&'a [Item<'a>]);

pub const HHMMSS_COLON: PresetFormat<'static> = PresetFormat::<'static>(&I_HHMMSS_COLON);
pub const YYYYMMDD_DASH: PresetFormat<'static> = PresetFormat::<'static>(&I_YYYYMMDD_DASH);
pub const YYYYYMMDD_DASH: PresetFormat<'static> = PresetFormat::<'static>(&I_YYYYYMMDD_DASH);
pub const YYYYMMDD_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&I_YYYYMMDD_SLASH);
pub const DDMMYYYY_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&I_DDMMYYYY_SLASH);
pub const DDMMYYYY_DOT: PresetFormat<'static> = PresetFormat::<'static>(&I_DDMMYYYY_DOT);
pub const MMDDYYYY_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&I_MMDDYYYY_SLASH);
pub const LONG_DATE: PresetFormat<'static> = PresetFormat::<'static>(&I_LONG_DATE);
pub const LONG_COMPL: PresetFormat<'static> = PresetFormat::<'static>(&I_LONG_COMPL);
pub const YEAR_WEEK_DAY: PresetFormat<'static> = PresetFormat::<'static>(&I_YEAR_WEEK_DAY);
pub const YEAR_MDD: PresetFormat<'static> = PresetFormat::<'static>(&I_YEAR_MDD);
pub const YEAR_COMPL: PresetFormat<'static> = PresetFormat::<'static>(&I_YEAR_COMPL);
pub const COMPL_ONLY: PresetFormat<'static> = PresetFormat::<'static>(&I_COMPL_ONLY);

pub trait PresetDisplay: DisplayItem {
    fn preset_str(&self, preset: PresetFormat) -> String {
        let mut result = String::new();
        for item in preset.0 {
            result.push_str(&self.fmt_item(*item))
        }
        result
    }

    fn long_date(&self) -> String {
        self.preset_str(LONG_DATE)
    }

    fn short_date(&self) -> String {
        self.preset_str(YYYYMMDD_DASH)
    }
}

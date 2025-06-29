use crate::display::private::Content;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::Item;
use crate::display::private::Locale;
use crate::display::private::NumericContent;
use crate::display::private::Sign;
use crate::display::private::TextContent;
use std::fmt;

const O_LITERAL: DisplayOptions = DisplayOptions {
    numerals: None,
    width: None,
    align: None,
    padding: None,
    case: None,
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

NumericDateItems!(ITEMS_HMS_COLON, ":", Hour0to23, 2, Minute, 2, Second, 2);
NumericDateItems!(ITEMS_YMD_DASH, "-", Year, 4, Month, 2, DayOfMonth, 2);
NumericDateItems!(ITEMS_Y5_MD_DASH, "-", Year, 5, Month, 2, DayOfMonth, 2);
NumericDateItems!(ITEMS_YMD_SLASH, "/", Year, 4, Month, 2, DayOfMonth, 2);
NumericDateItems!(ITEMS_DMY_SLASH, "/", DayOfMonth, 2, Month, 2, Year, 4);
NumericDateItems!(ITEMS_DMY_DOT, ".", DayOfMonth, 2, Month, 2, Year, 4);
NumericDateItems!(ITEMS_MDY_SLASH, "/", Month, 2, DayOfMonth, 2, Year, 4);

const ITEMS_LONG_DATE: [Item<'_>; 9] = [
    Item::new(Content::Text(TextContent::DayOfWeekName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::MonthName), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::DayOfMonth), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraName), O_LITERAL),
];

const ITEMS_COMPLEMENTARY: [Item<'_>; 5] = [
    Item::new(Content::Text(TextContent::ComplementaryDayName), O_LITERAL),
    Item::new(Content::Literal(", "), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Year), O_LITERAL),
    Item::new(Content::Literal(" "), O_LITERAL),
    Item::new(Content::Text(TextContent::EraName), O_LITERAL),
];

const ITEMS_YEAR_WEEK_DAY: [Item<'_>; 5] = [
    Item::new(Content::Numeric(NumericContent::Year), O_LITERAL),
    Item::new(Content::Literal("-W"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::WeekOfYear), O_LITERAL),
    Item::new(Content::Literal("-"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::DayOfWeek), O_LITERAL),
];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PresetFormat<'a>(&'a [Item<'a>]);

pub const HMS_COLON: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_HMS_COLON);
pub const YMD_DASH: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_YMD_DASH);
pub const Y5_MD_DASH: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_Y5_MD_DASH);
pub const YMD_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_YMD_SLASH);
pub const DMY_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_DMY_SLASH);
pub const DMY_DOT: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_DMY_DOT);
pub const MDY_SLASH: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_MDY_SLASH);
pub const LONG_DATE: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_LONG_DATE);
pub const LONG_COMPLEMENTARY: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_COMPLEMENTARY);
pub const YEAR_WEEK_DAY: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_YEAR_WEEK_DAY);

pub trait PresetDisplay: DisplayItem {
    fn preset_str(&self, preset: PresetFormat) -> String {
        let mut result = String::new();
        for item in preset.0 {
            result.push_str(&self.fmt_item(*item))
        }
        result
    }

    fn preset_fmt(&self, f: &mut fmt::Formatter, preset: PresetFormat) -> fmt::Result {
        for item in preset.0 {
            write!(f, "{}", self.fmt_item(*item))?;
        }
        Ok(())
    }
}

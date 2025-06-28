use crate::display::private::Content;
use crate::display::private::DisplayItem;
use crate::display::private::DisplayOptions;
use crate::display::private::Item;
use crate::display::private::Locale;
use crate::display::private::NumericContent;
use crate::display::private::Sign;
use crate::display::private::TextContent;
use std::fmt;

const O_N2: DisplayOptions = DisplayOptions {
    numerals: None,
    width: Some(2),
    align: None,
    padding: Some('0'),
    case: None,
    sign: Sign::Never,
    locale: Locale::en_CA,
};

const O_LITERAL: DisplayOptions = DisplayOptions {
    numerals: None,
    width: None,
    align: None,
    padding: None,
    case: None,
    sign: Sign::Never,
    locale: Locale::en_CA,
};

const ITEMS_TIME: [Item<'_>; 5] = [
    Item::new(Content::Numeric(NumericContent::Hour0to23), O_N2),
    Item::new(Content::Literal(":"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Minute), O_N2),
    Item::new(Content::Literal(":"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::Second), O_N2),
];

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
    Item::new(Content::Numeric(NumericContent::WeekOfYear), O_N2),
    Item::new(Content::Literal("-"), O_LITERAL),
    Item::new(Content::Numeric(NumericContent::DayOfWeek), O_N2),
];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PresetFormat<'a>(&'a [Item<'a>]);

pub const TIME: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_TIME);
pub const LONG_DATE: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_LONG_DATE);
pub const LONG_COMPLEMENTARY: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_COMPLEMENTARY);
pub const YEAR_WEEK_DAY: PresetFormat<'static> = PresetFormat::<'static>(&ITEMS_YEAR_WEEK_DAY);

pub trait PresetDisplay: DisplayItem {
    fn preset_fmt(&self, f: &mut fmt::Formatter, preset: PresetFormat) -> fmt::Result {
        for item in preset.0 {
            write!(f, "{}", self.fmt_item(*item))?;
        }
        Ok(())
    }
}

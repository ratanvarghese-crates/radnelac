#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum NumericContent {
    Month,
    DayOfWeek,
    DayOfMonth,
    Hour1to12,
    Hour0to23,
    Minute,
    Second,
    SecondsSinceEpoch,
    Year,
    Quarter,
    DaysSinceEpoch,
    ComplementaryDay,
    WeekOfYear,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum TextContent {
    MonthName,
    DayOfMonthName,
    DayOfWeekName,
    HalfDayName,
    HalfDayAbbrev,
    EraName,
    EraAbbreviation,
    ComplementaryDayName,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Content<'a> {
    Literal(&'a str),
    Numeric(NumericContent),
    Text(TextContent),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Case {
    Upper,
    Lower,
    Title,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Sign {
    Always,
    OnlyNegative,
    Never,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Locale {
    en_CA,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Numerals {
    HinduArabic,
    Roman,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct DisplayOptions {
    pub width: Option<usize>,
    pub align: Option<Align>,
    pub case: Option<Case>,
    pub padding: Option<char>,
    pub numerals: Option<Numerals>,
    pub sign: Sign,
    pub locale: Locale,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Item<'a> {
    pub content: Content<'a>,
    pub options: DisplayOptions,
}

impl<'a> Item<'a> {
    pub const fn new(content: Content<'a>, options: DisplayOptions) -> Self {
        Item {
            content: content,
            options: options,
        }
    }
}

pub trait DisplayItem {
    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String;
    fn fmt_text(&self, t: TextContent, opt: DisplayOptions) -> String;

    fn fmt_item(&self, item: Item) -> String {
        match item.content {
            Content::Literal(s) => String::from(s),
            Content::Numeric(n) => self.fmt_numeric(n, item.options),
            Content::Text(t) => self.fmt_text(t, item.options),
        }
    }
}

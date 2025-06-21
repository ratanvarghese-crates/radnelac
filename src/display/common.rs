use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::Quarter;
use crate::day_count::Epoch;
use crate::day_count::ToFixed;
use convert_case;
use convert_case::Casing;
use num_traits::NumAssign;
use num_traits::Signed;

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
pub struct DisplayOptions {
    pub width: Option<usize>,
    pub align: Option<Align>,
    pub case: Option<Case>,
    pub padding: Option<char>,
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

pub fn fmt_string(root: &str, opt: DisplayOptions) -> String {
    let mut result = String::new();
    let cased_root = if opt.case.is_some() {
        let case = match opt.case.unwrap() {
            Case::Upper => convert_case::Case::UpperFlat,
            Case::Lower => convert_case::Case::Flat,
            Case::Title => convert_case::Case::UpperCamel,
        };
        root.to_case(case)
    } else {
        String::from(root)
    };

    if opt.width.is_some() && opt.width.unwrap() > cased_root.len() {
        let align = opt.align.unwrap_or(Align::Left);
        let pad_char = opt.padding.unwrap_or(' ');
        let pad_width = opt.width.unwrap() - cased_root.len();
        let pad_left = std::iter::repeat(pad_char)
            .take((pad_width / 2) + (pad_width % 2))
            .collect::<String>();
        let pad_right = std::iter::repeat(pad_char)
            .take(pad_width - ((pad_width / 2) + (pad_width % 2)))
            .collect::<String>();
        let positions: [&str; 3] = match align {
            Align::Left => [&pad_left, &pad_right, &cased_root],
            Align::Right => [&cased_root, &pad_left, &pad_right],
            Align::Center => [&pad_left, &cased_root, &pad_right],
        };
        for item in positions {
            result.push_str(item);
        }
    } else {
        result.push_str(&cased_root);
        let max_len = opt.width.unwrap_or(cased_root.len());
        if cased_root.len() > max_len {
            let max_idx = cased_root
                .char_indices()
                .map(|x| x.0)
                .rfind(|x| *x <= max_len)
                .unwrap_or(0);
            result.truncate(max_idx);
        }
    }
    result
}

pub fn fmt_number<T: itoa::Integer + NumAssign + Signed + PartialOrd>(
    n: T,
    opt: DisplayOptions,
) -> String {
    let mut root_buffer = itoa::Buffer::new();
    let root = root_buffer.format(n.abs());
    let prefix = match (opt.sign, n >= T::zero()) {
        (Sign::Always, true) => "+",
        (Sign::Always, false) => "-",
        (Sign::OnlyNegative, true) => "",
        (Sign::OnlyNegative, false) => "-",
        (Sign::Never, _) => "",
    };
    let mut joined = String::from(prefix);
    if opt.padding == Some('0') && opt.align.unwrap_or(Align::Left) == Align::Left {
        let non_pad_width = prefix.len() + root.len();
        let pad_width = opt.width.unwrap_or(non_pad_width) - non_pad_width;
        let padding = std::iter::repeat('0').take(pad_width).collect::<String>();
        joined.push_str(&padding);
    }
    joined.push_str(root);
    fmt_string(&joined, opt)
}

pub fn fmt_days_since_epoch<T: Epoch + ToFixed>(t: T, opt: DisplayOptions) -> String {
    fmt_number(t.to_fixed().get_day_i() - T::epoch().get_day_i(), opt)
}

pub fn fmt_seconds_since_epoch<T: Epoch + ToFixed>(t: T, opt: DisplayOptions) -> String {
    fmt_number(
        ((t.to_fixed().get() - T::epoch().get()) * (24.0 * 60.0 * 60.0)) as i16,
        opt,
    )
}

pub fn fmt_quarter<T: Quarter>(t: T, opt: DisplayOptions) -> String {
    fmt_number(t.quarter().get() as i16, opt)
}

impl DisplayItem for CommonDate {
    fn fmt_numeric(&self, n: NumericContent, opt: DisplayOptions) -> String {
        match n {
            NumericContent::Month => fmt_number(self.month as i16, opt),
            NumericContent::DayOfMonth => fmt_number(self.day as i16, opt),
            NumericContent::Year => fmt_number(self.year, opt),
            _ => String::from(""),
        }
    }
    fn fmt_text(&self, _t: TextContent, _opt: DisplayOptions) -> String {
        String::from("")
    }
}

mod tests {
    use super::*;

    #[test]
    fn basic_number() {
        let opt_0 = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_0), "2025");
        assert_eq!(fmt_number(-2025, opt_0), "2025");
        let opt_1 = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Always,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_1), "+2025");
        assert_eq!(fmt_number(-2025, opt_1), "-2025");
        let opt_2 = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::OnlyNegative,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_2), "2025");
        assert_eq!(fmt_number(-2025, opt_2), "-2025");
    }

    #[test]
    fn basic_text() {
        let opt_0 = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("January", opt_0), "January");
    }

    #[test]
    fn case_text() {
        let opt_0 = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: Some(Case::Upper),
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("mAy", opt_0), "MAY");
        let opt_1 = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: Some(Case::Lower),
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("mAy", opt_1), "may");
        let opt_2 = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: Some(Case::Title),
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("mAy", opt_2), "MAy");
        let opt_3 = DisplayOptions {
            width: None,
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("mAy", opt_3), "mAy");
    }

    #[test]
    fn pad_number() {
        let opt_0 = DisplayOptions {
            width: Some(8),
            align: None,
            padding: Some('@'),
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_0), "@@@@2025");
        assert_eq!(fmt_number(-2025, opt_0), "@@@@2025");
        let opt_1 = DisplayOptions {
            width: Some(8),
            align: None,
            padding: Some('@'),
            case: None,
            sign: Sign::Always,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_1), "@@@+2025");
        assert_eq!(fmt_number(-2025, opt_1), "@@@-2025");
        let opt_2 = DisplayOptions {
            width: Some(8),
            align: None,
            padding: Some('0'),
            case: None,
            sign: Sign::Always,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_2), "+0002025");
        assert_eq!(fmt_number(-2025, opt_2), "-0002025");
    }

    #[test]
    fn align_number() {
        let opt_0 = DisplayOptions {
            width: Some(8),
            align: Some(Align::Left),
            padding: Some('@'),
            case: None,
            sign: Sign::OnlyNegative,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_0), "@@@@2025");
        assert_eq!(fmt_number(-2025, opt_0), "@@@-2025");
        let opt_1 = DisplayOptions {
            width: Some(8),
            align: Some(Align::Right),
            padding: Some('@'),
            case: None,
            sign: Sign::OnlyNegative,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_1), "2025@@@@");
        assert_eq!(fmt_number(-2025, opt_1), "-2025@@@");
        let opt_2 = DisplayOptions {
            width: Some(8),
            align: Some(Align::Center),
            padding: Some('@'),
            case: None,
            sign: Sign::OnlyNegative,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_2), "@@2025@@");
        assert_eq!(fmt_number(-2025, opt_2), "@@-2025@");
    }

    #[test]
    fn trunc_number() {
        let opt_0 = DisplayOptions {
            width: Some(2),
            align: None,
            padding: None,
            case: None,
            sign: Sign::OnlyNegative,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_number(2025, opt_0), "20");
        assert_eq!(fmt_number(-2025, opt_0), "-2");
    }

    #[test]
    fn align_text() {
        let opt_0 = DisplayOptions {
            width: Some(8),
            align: Some(Align::Left),
            padding: Some('@'),
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("June", opt_0), "@@@@June");
        let opt_1 = DisplayOptions {
            width: Some(8),
            align: Some(Align::Right),
            padding: Some('@'),
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("June", opt_1), "June@@@@");
        let opt_2 = DisplayOptions {
            width: Some(8),
            align: Some(Align::Center),
            padding: Some('@'),
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("June", opt_2), "@@June@@");
    }

    #[test]
    fn trunc_text() {
        let opt_0 = DisplayOptions {
            width: Some(3),
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("January", opt_0), "Jan");
    }

    #[test]
    fn trunc_text_unicode() {
        let opt_0 = DisplayOptions {
            width: Some(1),
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("😀", opt_0), "");
        assert_eq!(fmt_string("😀😂", opt_0), "");
        let opt_1 = DisplayOptions {
            width: Some(4),
            align: None,
            padding: None,
            case: None,
            sign: Sign::Never,
            locale: Locale::en_CA,
        };
        assert_eq!(fmt_string("😀", opt_1), "😀");
        assert_eq!(fmt_string("😀😂", opt_1), "😀");
    }
}

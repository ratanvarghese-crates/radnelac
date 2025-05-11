#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct CommonDate {
    pub year: i16,
    pub month: u8,
    pub day: u8,
}

pub trait ValidCommonDate {
    fn is_valid(date: CommonDate) -> bool;
}

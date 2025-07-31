use crate::calendar::Roman;
use crate::calendar::RomanMonthlyEvent;
use crate::display::private::get_dict;
use crate::display::text::prelude::Language;
use numerals;
use std::fmt;

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dict = get_dict(Language::EN)
            .roman
            .as_ref()
            .expect("Known to exist");

        let event_name = match self.event() {
            RomanMonthlyEvent::Kalends => dict.kalends,
            RomanMonthlyEvent::Nones => dict.nones,
            RomanMonthlyEvent::Ides => dict.ides,
        };

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
        let month_name = months[self.month() as usize - 1];
        let year = Roman::auc_year_from_julian(self.year());

        if self.count().get() == 1 {
            write!(
                f,
                "{} {} {}, {} {}",
                event_name, dict.x_of_y, month_name, year, dict.after_auc_full
            )
        } else if self.count().get() == 2 {
            write!(
                f,
                "{} {} {}, {} {}",
                dict.pridie, event_name, month_name, year, dict.after_auc_full
            )
        } else {
            let bissextum = if self.leap() { dict.bissextum } else { "" };
            let bissextum_space = if self.leap() { " " } else { "" };
            write!(
                f,
                "{} {}{}{:X} {} {}, {} {}",
                dict.ante_diem,
                bissextum,
                bissextum_space,
                numerals::roman::Roman::from(self.count().get() as i16),
                event_name,
                month_name,
                year,
                dict.after_auc_full
            )
        }
    }
}

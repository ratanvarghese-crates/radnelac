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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::CommonDate;
    use crate::calendar::Julian;
    use crate::calendar::ToFromCommonDate;
    use crate::day_count::FromFixed;
    use crate::day_count::ToFixed;

    #[test]
    fn second_sixth_day_before_kalends_of_march() {
        let j24 = Julian::try_from_common_date(CommonDate::new(4, 2, 24)).unwrap();
        let j25 = Julian::try_from_common_date(CommonDate::new(4, 2, 25)).unwrap();
        let f24 = j24.to_fixed();
        let f25 = j25.to_fixed();
        let r24s = Roman::from_fixed(f24).to_string();
        let r25s = Roman::from_fixed(f25).to_string();
        assert!(r24s.starts_with("ante diem VI Kalends March"), "{}", r24s);
        assert!(
            r25s.starts_with("ante diem bissextum VI Kalends March"),
            "{}",
            r25s
        );
    }

    #[test]
    fn pridie_nones_of_march() {
        let j = Julian::try_from_common_date(CommonDate::new(-44, 3, 6)).unwrap();
        let f = j.to_fixed();
        let r = Roman::from_fixed(f);
        assert!(r.to_string().starts_with("pridie Nones March"));
    }

    #[test]
    fn pridie_ides_of_march() {
        let j = Julian::try_from_common_date(CommonDate::new(-44, 3, 14)).unwrap();
        let f = j.to_fixed();
        let r = Roman::from_fixed(f);
        assert!(r.to_string().starts_with("pridie Ides March"));
    }

    #[test]
    fn ides_of_march() {
        let j = Julian::try_from_common_date(CommonDate::new(-44, 3, 15)).unwrap();
        let f = j.to_fixed();
        let r = Roman::from_fixed(f);
        assert!(r.to_string().starts_with("Ides of March"));
    }
}

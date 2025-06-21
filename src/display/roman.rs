use crate::calendar::Roman;
use crate::calendar::RomanMonthlyEvent;
use std::fmt;

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let event_name = match self.event() {
            RomanMonthlyEvent::Kalends => "Kalends",
            RomanMonthlyEvent::Nones => "Nones",
            RomanMonthlyEvent::Ides => "Ides",
        };

        const MONTHS: [&str; 12] = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let month_name = MONTHS[self.month() as usize - 1];
        let year = Roman::auc_year_from_julian(self.year());

        if self.count().get() == 1 {
            write!(
                f,
                "{} of {}, {} Ab urbe condita",
                event_name, month_name, year
            )
        } else if self.count().get() == 2 {
            write!(
                f,
                "pridie {} {}, {} Ab urbe condita",
                event_name, month_name, year
            )
        } else {
            let bissextum = if self.leap() { "bissextum " } else { "" };
            write!(
                f,
                "ante diem {}{} {} {}, {} Ab urbe condita",
                bissextum,
                self.count(),
                event_name,
                month_name,
                year
            )
        }
    }
}

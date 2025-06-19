use crate::day_cycle::Akan;
use crate::day_cycle::AkanPrefix;
use crate::day_cycle::AkanStem;
use std::fmt;

impl fmt::Display for Akan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Using names from Wikipedia
        // https://en.wikipedia.org/wiki/Akan_calendar
        let prefix = match self.prefix() {
            AkanPrefix::Nwona => "Nwuna",
            AkanPrefix::Nkyi => "Nkyi",
            AkanPrefix::Kuru => "Kuro",
            AkanPrefix::Kwa => "Kwa",
            AkanPrefix::Mono => "Mono",
            AkanPrefix::Fo => "Fo",
        };
        let stem = match self.stem() {
            AkanStem::Wukuo => "Wukuo",
            AkanStem::Yaw => "Ya",
            AkanStem::Fie => {
                if self.prefix() == AkanPrefix::Fo {
                    "Fi"
                } else {
                    "Afi"
                }
            }
            AkanStem::Memene => "Mene",
            AkanStem::Kwasi => "Kwasi",
            AkanStem::Dwo => "Dwo",
            AkanStem::Bene => "Bena",
        };
        write!(f, "{}-{}", prefix, stem)
    }
}

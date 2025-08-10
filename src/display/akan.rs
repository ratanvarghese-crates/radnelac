// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::day_cycle::Akan;
use crate::day_cycle::AkanPrefix;
use crate::day_cycle::AkanStem;
use crate::display::private::get_dict;
use crate::display::text::prelude::Language;
use std::fmt;

impl fmt::Display for Akan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dict = get_dict(Language::EN)
            .akan_cycle
            .as_ref()
            .expect("Known to exist");
        // Using names from Wikipedia
        // https://en.wikipedia.org/wiki/Akan_calendar
        let prefix = match self.prefix() {
            AkanPrefix::Nwona => dict.nwona,
            AkanPrefix::Nkyi => dict.nkyi,
            AkanPrefix::Kuru => dict.kuru,
            AkanPrefix::Kwa => dict.kwa,
            AkanPrefix::Mono => dict.mono,
            AkanPrefix::Fo => dict.fo,
        };
        let stem = match self.stem() {
            AkanStem::Wukuo => dict.wukuo,
            AkanStem::Yaw => dict.yaw,
            AkanStem::Fie => match self.prefix() {
                AkanPrefix::Fo => dict.fie,
                _ => dict.afie,
            },
            AkanStem::Memene => dict.memene,
            AkanStem::Kwasi => dict.kwasi,
            AkanStem::Dwo => dict.dwo,
            AkanStem::Bene => dict.bene,
        };
        write!(f, "{}-{}", prefix, stem)
    }
}

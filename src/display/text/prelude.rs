/// Represents a written and spoken language
///
/// Languages are identified by their ISO 639-1 language code.
///
/// For functions that take a Language parameter, the following strings may be output in
/// the specified language:
///
/// + names of epochs
/// + names of months
/// + names of days of weeks
/// + names of days of month
///
/// Currently, Language::EN (English) is supported for all timekeeping systems, and
/// Language::FR (French) is only supported for a subset of timekeeping systems.
///
/// # Further Reading
/// + [Wikipedia](//https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Language {
    /// English
    EN,
    /// French
    FR,
}

#[derive(Debug)]
pub struct AkanCycleDictionary<'a> {
    //Prefix
    pub nwona: &'a str,
    pub nkyi: &'a str,
    pub kuru: &'a str,
    pub kwa: &'a str,
    pub mono: &'a str,
    pub fo: &'a str,
    //Stem
    pub wukuo: &'a str,
    pub yaw: &'a str,
    pub fie: &'a str,
    pub afie: &'a str, //Alternate name for "fie"
    pub memene: &'a str,
    pub kwasi: &'a str,
    pub dwo: &'a str,
    pub bene: &'a str,
    //Punctuation
    pub seperator: &'a str,
}

#[derive(Debug)]
pub struct ArmenianDictionary<'a> {
    //Months
    pub nawasardi: &'a str,
    pub hori: &'a str,
    pub sahmi: &'a str,
    pub tre: &'a str,
    pub kaloch: &'a str,
    pub arach: &'a str,
    pub mehekani: &'a str,
    pub areg: &'a str,
    pub ahekani: &'a str,
    pub mareri: &'a str,
    pub margach: &'a str,
    pub hrotich: &'a str,
    //Epagomenal
    pub aweleac: &'a str,
    //Days of month
    pub areg_day: &'a str, //Note that there is a month with a similar name
    pub hrand: &'a str,
    pub aram: &'a str,
    pub margar: &'a str,
    pub ahrank: &'a str,
    pub mazdel: &'a str,
    pub astlik: &'a str,
    pub mihr: &'a str,
    pub jopaber: &'a str,
    pub murc: &'a str,
    pub erezhan: &'a str,
    pub ani: &'a str,
    pub parkhar: &'a str,
    pub vanat: &'a str,
    pub aramazd: &'a str,
    pub mani: &'a str,
    pub asak: &'a str,
    pub masis: &'a str,
    pub anahit: &'a str,
    pub aragats: &'a str,
    pub gorgor: &'a str,
    pub kordvik: &'a str,
    pub tsmak: &'a str,
    pub lusnak: &'a str,
    pub tsron: &'a str,
    pub npat: &'a str,
    pub vahagn: &'a str,
    pub sim: &'a str,
    pub varag: &'a str,
    pub giseravar: &'a str,
    //Epoch
    pub before_epoch_full: &'a str,
    pub after_epoch_full: &'a str,
    pub before_epoch_abr: &'a str,
    pub after_epoch_abr: &'a str,
}

#[derive(Debug)]
pub struct CommonClockDictionary<'a> {
    pub am_full: &'a str,
    pub pm_full: &'a str,
    pub am_abr: &'a str,
    pub pm_abr: &'a str,
}

#[derive(Debug)]
pub struct CopticDictionary<'a> {
    //Months
    pub thoout: &'a str,
    pub paope: &'a str,
    pub athor: &'a str,
    pub koiak: &'a str,
    pub tobe: &'a str,
    pub meshir: &'a str,
    pub paremotep: &'a str,
    pub parmoute: &'a str,
    pub pashons: &'a str,
    pub paone: &'a str,
    pub epep: &'a str,
    pub mesore: &'a str,
    pub epagomene: &'a str,
    //Epoch
    pub before_martyrs_full: &'a str,
    pub after_martyrs_full: &'a str,
    pub before_martyrs_abr: &'a str,
    pub after_martyrs_abr: &'a str,
}

#[derive(Debug)]
pub struct CotsworthDictionary<'a> {
    //Months
    pub january: &'a str,
    pub february: &'a str,
    pub march: &'a str,
    pub april: &'a str,
    pub may: &'a str,
    pub june: &'a str,
    pub sol: &'a str,
    pub july: &'a str,
    pub august: &'a str,
    pub september: &'a str,
    pub october: &'a str,
    pub november: &'a str,
    pub december: &'a str,
    //Epoch
    pub before_epoch_full: &'a str,
    pub after_epoch_full: &'a str,
    pub before_epoch_abr: &'a str,
    pub after_epoch_abr: &'a str,
    //Intercalary Days
    pub year_day: &'a str,
    pub leap_day: &'a str,
}

#[derive(Debug)]
pub struct EgyptianDictionary<'a> {
    //Months
    pub thoth: &'a str,
    pub phaophi: &'a str,
    pub athyr: &'a str,
    pub choiak: &'a str,
    pub tybi: &'a str,
    pub mechir: &'a str,
    pub phamenoth: &'a str,
    pub pharmuthi: &'a str,
    pub pachon: &'a str,
    pub payni: &'a str,
    pub epiphi: &'a str,
    pub mesori: &'a str,
    //Epagomenae
    pub epagomenae: &'a str,
    //Epoch
    pub before_nabonassar_full: &'a str,
    pub after_nabonassar_full: &'a str,
    pub before_nabonassar_abr: &'a str,
    pub after_nabonassar_abr: &'a str,
    //Intercalary Days
    pub birth_of_osiris: &'a str,
    pub birth_of_horus: &'a str,
    pub birth_of_seth: &'a str,
    pub birth_of_isis: &'a str,
    pub birth_of_nephthys: &'a str,
}

#[derive(Debug)]
pub struct EthiopicDictionary<'a> {
    //Months
    pub maskaram: &'a str,
    pub teqemt: &'a str,
    pub hedar: &'a str,
    pub takhsas: &'a str,
    pub ter: &'a str,
    pub yakatit: &'a str,
    pub magabit: &'a str,
    pub miyazya: &'a str,
    pub genbot: &'a str,
    pub sane: &'a str,
    pub hamle: &'a str,
    pub nahase: &'a str,
    pub paguemen: &'a str,
    //Epoch
    pub before_incarnation_full: &'a str,
    pub after_incarnation_full: &'a str,
    pub before_incarnation_abr: &'a str,
    pub after_incarnation_abr: &'a str,
}

#[derive(Debug)]
pub struct FrenchRevolutionaryDictionary<'a> {
    //Months
    pub vendemiaire: &'a str,
    pub brumaire: &'a str,
    pub frimaire: &'a str,
    pub nivose: &'a str,
    pub pluviose: &'a str,
    pub ventose: &'a str,
    pub germinal: &'a str,
    pub floreal: &'a str,
    pub prairial: &'a str,
    pub messidor: &'a str,
    pub thermidor: &'a str,
    pub fructidor: &'a str,
    //Weekdays
    pub primidi: &'a str,
    pub duodi: &'a str,
    pub tridi: &'a str,
    pub quartidi: &'a str,
    pub quintidi: &'a str,
    pub sextidi: &'a str,
    pub septidi: &'a str,
    pub octidi: &'a str,
    pub nonidi: &'a str,
    pub decadi: &'a str,
    //Sansculottide
    pub fete_de_la_vertu: &'a str,
    pub fete_du_genie: &'a str,
    pub fete_du_travail: &'a str,
    pub fete_de_lopinion: &'a str,
    pub fete_des_recompenses: &'a str,
    pub fete_de_la_revolution: &'a str,
    //Epoch
    pub before_republic_full: &'a str,
    pub after_republic_full: &'a str,
    pub before_republic_abr: &'a str,
    pub after_republic_abr: &'a str,
}

#[derive(Debug)]
pub struct GregorianDictionary<'a> {
    //Months
    pub january: &'a str,
    pub february: &'a str,
    pub march: &'a str,
    pub april: &'a str,
    pub may: &'a str,
    pub june: &'a str,
    pub july: &'a str,
    pub august: &'a str,
    pub september: &'a str,
    pub october: &'a str,
    pub november: &'a str,
    pub december: &'a str,
    //Epoch
    pub before_common_era_full: &'a str,
    pub common_era_full: &'a str,
    pub before_common_era_abr: &'a str,
    pub common_era_abr: &'a str,
}

#[derive(Debug)]
pub struct HoloceneDictionary<'a> {
    //Months
    pub january: &'a str,
    pub february: &'a str,
    pub march: &'a str,
    pub april: &'a str,
    pub may: &'a str,
    pub june: &'a str,
    pub july: &'a str,
    pub august: &'a str,
    pub september: &'a str,
    pub october: &'a str,
    pub november: &'a str,
    pub december: &'a str,
    //Epoch
    pub before_human_era_full: &'a str,
    pub human_era_full: &'a str,
    pub before_human_era_abr: &'a str,
    pub human_era_abr: &'a str,
}

#[derive(Debug)]
pub struct ISODictionary<'a> {
    //Epoch
    pub before_epoch_full: &'a str,
    pub after_epoch_full: &'a str,
    pub before_epoch_abr: &'a str,
    pub after_epoch_abr: &'a str,
}

#[derive(Debug)]
pub struct JulianDictionary<'a> {
    //Months
    pub january: &'a str,
    pub february: &'a str,
    pub march: &'a str,
    pub april: &'a str,
    pub may: &'a str,
    pub june: &'a str,
    pub july: &'a str,
    pub august: &'a str,
    pub september: &'a str,
    pub october: &'a str,
    pub november: &'a str,
    pub december: &'a str,
    //Epoch
    pub before_christ_full: &'a str,
    pub anno_domini_full: &'a str,
    pub before_christ_abr: &'a str,
    pub anno_domini_abr: &'a str,
}

#[derive(Debug)]
pub struct PositivistDictionary<'a> {
    //Months
    pub moses: &'a str,
    pub homer: &'a str,
    pub aristotle: &'a str,
    pub archimedes: &'a str,
    pub caesar: &'a str,
    pub saint_paul: &'a str,
    pub charlemagne: &'a str,
    pub dante: &'a str,
    pub gutenburg: &'a str,
    pub shakespeare: &'a str,
    pub descartes: &'a str,
    pub frederick: &'a str,
    pub bichat: &'a str,
    //Epoch
    pub before_crisis_full: &'a str,
    pub after_crisis_full: &'a str,
    pub before_crisis_abr: &'a str,
    pub after_crisis_abr: &'a str,
    //Intercalary Days
    pub festival_of_dead: &'a str,
    pub festival_of_holy_women: &'a str,
}

#[derive(Debug)]
pub struct RomanDictionary<'a> {
    //Monthly events
    pub kalends: &'a str,
    pub nones: &'a str,
    pub ides: &'a str,
    //Months
    pub january: &'a str,
    pub february: &'a str,
    pub march: &'a str,
    pub april: &'a str,
    pub may: &'a str,
    pub june: &'a str,
    pub july: &'a str,
    pub august: &'a str,
    pub september: &'a str,
    pub october: &'a str,
    pub november: &'a str,
    pub december: &'a str,
    //BC/AD Epoch
    pub before_christ_full: &'a str,
    pub anno_domini_full: &'a str,
    pub before_christ_abr: &'a str,
    pub anno_domini_abr: &'a str,
    //AUC Epoch
    pub before_auc_full: &'a str,
    pub after_auc_full: &'a str,
    pub before_auc_abr: &'a str,
    pub after_auc_abr: &'a str,
    //Misc
    pub pridie: &'a str,
    pub ante_diem: &'a str,
    pub bissextum: &'a str,
    pub x_of_y: &'a str,
}

#[derive(Debug)]
pub struct SymmetryDictionary<'a> {
    //Months
    pub january: &'a str,
    pub february: &'a str,
    pub march: &'a str,
    pub april: &'a str,
    pub may: &'a str,
    pub june: &'a str,
    pub july: &'a str,
    pub august: &'a str,
    pub september: &'a str,
    pub october: &'a str,
    pub november: &'a str,
    pub december: &'a str,
    pub irvember: &'a str,
    //Epoch
    pub before_epoch_full: &'a str,
    pub after_epoch_full: &'a str,
    pub before_epoch_abr: &'a str,
    pub after_epoch_abr: &'a str,
}

#[derive(Debug)]
pub struct TranquilityDictionary<'a> {
    //Months
    pub archimedes: &'a str,
    pub brahe: &'a str,
    pub copernicus: &'a str,
    pub darwin: &'a str,
    pub einstein: &'a str,
    pub faraday: &'a str,
    pub galileo: &'a str,
    pub hippocrates: &'a str,
    pub imhotep: &'a str,
    pub jung: &'a str,
    pub kepler: &'a str,
    pub lavoisier: &'a str,
    pub mendel: &'a str,
    //Epoch
    pub before_tranquility_full: &'a str,
    pub after_tranquility_full: &'a str,
    pub before_tranquility_abr: &'a str,
    pub after_tranquility_abr: &'a str,
    //Intercalary Days
    pub moon_landing_day: &'a str,
    pub armstrong_day: &'a str,
    pub aldrin_day: &'a str,
}

#[derive(Debug)]
pub struct CommonWeekdayDictionary<'a> {
    pub sunday: &'a str,
    pub monday: &'a str,
    pub tuesday: &'a str,
    pub wednesday: &'a str,
    pub thursday: &'a str,
    pub friday: &'a str,
    pub saturday: &'a str,
}

#[derive(Debug)]
pub struct Dictionary<'a> {
    pub akan_cycle: Option<AkanCycleDictionary<'a>>,
    pub armenian: Option<ArmenianDictionary<'a>>,
    pub common_clock: Option<CommonClockDictionary<'a>>,
    pub coptic: Option<CopticDictionary<'a>>,
    pub cotsworth: Option<CotsworthDictionary<'a>>,
    pub egyptian: Option<EgyptianDictionary<'a>>,
    pub ethiopic: Option<EthiopicDictionary<'a>>,
    pub french_rev: Option<FrenchRevolutionaryDictionary<'a>>,
    pub gregorian: Option<GregorianDictionary<'a>>,
    pub holocene: Option<HoloceneDictionary<'a>>,
    pub iso: Option<ISODictionary<'a>>,
    pub julian: Option<JulianDictionary<'a>>,
    pub positivist: Option<PositivistDictionary<'a>>,
    pub roman: Option<RomanDictionary<'a>>,
    pub symmetry: Option<SymmetryDictionary<'a>>,
    pub tranquility: Option<TranquilityDictionary<'a>>,
    pub common_weekday: Option<CommonWeekdayDictionary<'a>>,
}

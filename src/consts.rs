impl From<char> for Number {
    fn from(c: char) -> Self {
        match c {
            '0' => Number::Zero,
            '1' => Number::One,
            '2' => Number::Two,
            '3' => Number::Three,
            '4' => Number::Four,
            '5' => Number::Five,
            '6' => Number::Six,
            '7' => Number::Seven,
            '8' => Number::Eight,
            '9' => Number::Nine,
            _ => unreachable!("UNREACHABLE: not a number!"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

pub enum PluralType {
    One,
    TwoFour,
    FiveNine,
}

pub const MAX_NUMBER_LEN: usize = (GROUP_NAMES.len() + 1) * 3;

pub const UNITS: &[&str] = &[
    "jedan", "dva", "tri", "četiri", "pet", "šest", "sedam", "osam", "devet",
];

pub const ALT_UNITS: &[&str] = &["jedna", "dvije"];

pub const NAEST: &[&str] = &[
    "jedanaest",
    "dvanaest",
    "trinaest",
    "četrnaest",
    "petnaest",
    "šesnaest",
    "sedamnaest",
    "osamnaest",
    "devetnaest",
];

pub const TENS: &[&str] = &[
    "deset",
    "dvadeset",
    "trideset",
    "četrdeset",
    "pedeset",
    "šezdeset",
    "sedamdeset",
    "osamdeset",
    "devedeset",
];

pub const HUNDREDS: &[&str] = &[
    "sto",
    "dvjesto",
    "tristo",
    "četiristo",
    "petsto",
    "šesto",
    "sedamsto",
    "osamsto",
    "devetsto",
];

pub const THOUSANDS: &[&str] = &["tisuća", "tisuće"];

pub const MILIJUN: &[&str] = &["milijun", "milijuna"];

pub const MILIJARDA: &[&str] = &["milijarda", "milijarde", "milijardi"];

pub const BILIJUN: &[&str] = &["bilijun", "bilijuna"];

pub const BILIJARDA: &[&str] = &["bilijarda", "bilijarde", "bilijardi"];

pub const TRILIJUN: &[&str] = &["trilijun", "trilijuna"];

pub const TRILIJARDA: &[&str] = &["trilijarda", "trilijarde", "trilijardi"];

pub const KVADRILIJUN: &[&str] = &["kvadrilijun", "kvadrilijuna"];

pub const KVADRILIJARDA: &[&str] =
    &["kvadrilijarda", "kvadrilijarde", "kvadrilijardi"];

pub const KVINTILIJUN: &[&str] = &["kvintilijun", "kvintilijuna"];

pub const KVINTILIJARDA: &[&str] =
    &["kvintilijarda", "kvintilijarde", "kvintilijardi"];

pub const SEKSTILIJUN: &[&str] = &["sekstilijun", "sekstilijuna"];

pub const SEKSTILIJARDA: &[&str] =
    &["sekstilijarda", "sekstilijarde", "sekstilijardi"];

pub const SEPTILIJUN: &[&str] = &["septilijun", "septilijuna"];

pub const SEPTILIJARDA: &[&str] = &["septilijarda", "septilijarde", "septilijardi"];

pub const OKTILIJUN: &[&str] = &["oktilijun", "oktilijuna"];

pub const OKTILIJARDA: &[&str] = &["oktilijarda", "oktilijarde", "oktilijardi"];

pub const NONILIJUN: &[&str] = &["nonilijun", "nonilijuna"];

pub const NONILIJARDA: &[&str] = &["nonilijarda", "nonilijarde", "nonilijardi"];

pub const DECILIJUN: &[&str] = &["decilijun", "decilijuna"];

pub const DECILIJARDA: &[&str] = &["decilijarda", "decilijarde", "decilijardi"];

pub const GROUP_NAMES: &[&[&str]] = &[
    THOUSANDS,
    MILIJUN,
    MILIJARDA,
    BILIJUN,
    BILIJARDA,
    TRILIJUN,
    TRILIJARDA,
    KVADRILIJUN,
    KVADRILIJARDA,
    KVINTILIJUN,
    KVINTILIJARDA,
    SEKSTILIJUN,
    SEKSTILIJARDA,
    SEPTILIJUN,
    SEPTILIJARDA,
    OKTILIJUN,
    OKTILIJARDA,
    NONILIJUN,
    NONILIJARDA,
    DECILIJUN,
    DECILIJARDA,
];

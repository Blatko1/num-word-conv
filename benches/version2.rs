#[allow(clippy::result_unit_err)]
pub fn num_word_conv<S: AsRef<str>>(input: S) -> Result<Vec<String>, ()> {
    let mut result: Vec<String> = Vec::new();
    let nums = parse_input(input.as_ref())?;

    if nums.len() > MAX_NUMBER_LEN {
        return Err(());
    }

    let group_count = (nums.len() as f32 / 3.0).ceil() as usize;

    for g in 0..group_count {
        let offset = 3 * g;

        let group = [nums[offset], nums[offset + 1], nums[offset + 2]];

        if is_blank(group) {
            continue;
        }

        let g_inverse = group_count - g - 1;
        let is_last_group = g == (group_count - 1);
        let is_special_unit_name = (g_inverse % 2 != 0) && !is_last_group;

        let (converted, plural_type) = convert(group, is_special_unit_name);
        result.push(converted);

        if !is_last_group {
            let group_name = GROUP_NAMES[g_inverse - 1];

            let name = match plural_type {
                PluralType::One => group_name[0],
                PluralType::TwoFour => group_name[1],
                PluralType::FiveNine => {
                    if g_inverse == 1 {
                        group_name[0]
                    } else {
                        group_name[group_name.len() - 1]
                    }
                }
            };
            result.push(name.to_owned());
            result.push(" ".to_owned());
        }
    }
    Ok(result)
}

pub fn convert(
    group: [Number; 3],
    is_special_unit_name: bool,
) -> (String, PluralType) {
    let mut converted = String::new();
    let mut plural_type = PluralType::FiveNine;

    let mut add_digit = |digit: &str| {
        converted.push_str(digit);
        converted.push(' ');
    };

    let hundreds_val = *group.first().unwrap();
    match hundreds_val {
        Number::Zero => (),
        v => {
            add_digit(HUNDREDS[v as usize - 1]);
        }
    }

    let tens_val = *group.get(1).unwrap();
    match tens_val {
        Number::Zero => (),
        Number::One => {
            let units_val = *group.last().unwrap();
            if units_val == Number::Zero {
                add_digit(TENS[0]);
            } else {
                add_digit(NAEST[units_val as usize - 1]);
            }
            return (converted, plural_type);
        }
        v => add_digit(TENS[v as usize - 1]),
    }

    let units_val = *group.last().unwrap();
    let digit = units_val as usize;
    match units_val {
        Number::Zero => (),
        Number::One => {
            plural_type = PluralType::One;
            if is_special_unit_name {
                add_digit(ALT_UNITS[0]);
            } else {
                add_digit(UNITS[digit - 1]);
            }
        }
        Number::Two => {
            plural_type = PluralType::TwoFour;
            if is_special_unit_name {
                add_digit(ALT_UNITS[1]);
            } else {
                add_digit(UNITS[digit - 1]);
            }
        }
        Number::Four | Number::Three => {
            plural_type = PluralType::TwoFour;
            add_digit(UNITS[digit - 1]);
        }
        _ => {
            add_digit(UNITS[digit - 1]);
        }
    }

    (converted, plural_type)
}

fn is_blank(group: [Number; 3]) -> bool {
    group.iter().all(|&n| n == Number::Zero)
}

fn parse_input(input: &str) -> Result<Vec<Number>, ()> {
    let mut result = Vec::new();
    let input = input.trim();
    let chars = input.chars();

    if !chars.clone().all(|x| x.is_ascii_digit()) {
        return Err(());
    }

    let number_count = input.len();
    let unfilled = 3 - number_count % 3;
    if unfilled != 3 {
        for _ in 0..unfilled {
            result.push(Number::Zero);
        }
    }

    for c in chars {
        result.push(c.into());
    }

    Ok(result)
}

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

const MAX_NUMBER_LEN: usize = (GROUP_NAMES.len() + 1) * 3;

const UNITS: &[&str] = &[
    "jedan", "dva", "tri", "četiri", "pet", "šest", "sedam", "osam", "devet",
];

const ALT_UNITS: &[&str] = &["jedna", "dvije"];

const NAEST: &[&str] = &[
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

const TENS: &[&str] = &[
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

const HUNDREDS: &[&str] = &[
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

const THOUSANDS: &[&str] = &["tisuća", "tisuće"];

const MILIJUN: &[&str] = &["milijun", "milijuna"];

const MILIJARDA: &[&str] = &["milijarda", "milijarde", "milijardi"];

const BILIJUN: &[&str] = &["bilijun", "bilijuna"];

const BILIJARDA: &[&str] = &["bilijarda", "bilijarde", "bilijardi"];

const TRILIJUN: &[&str] = &["trilijun", "trilijuna"];

const TRILIJARDA: &[&str] = &["trilijarda", "trilijarde", "trilijardi"];

const GROUP_NAMES: &[&[&str]] = &[
    THOUSANDS, MILIJUN, MILIJARDA, BILIJUN, BILIJARDA, TRILIJUN, TRILIJARDA,
];

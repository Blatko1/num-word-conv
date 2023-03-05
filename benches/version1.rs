pub fn num_word_conv<S: AsRef<str>>(input: S) -> Result<Vec<String>, ()> {
    let mut result: Vec<String> = Vec::new();
    let (nums, in_currency) = match load_input(input.as_ref()) {
        Ok(n) => n,
        Err(_) => return Err(()),
    };

    let length = nums.len();
    let group_count = (length as f32 / 3.0).ceil() as usize;

    for g in 0..group_count {
        let offset = 3 * g;

        let hundreds = nums[offset];
        let tens = nums[1 + offset];
        let units = nums[2 + offset];

        let g_inv = group_count - g - 1;
        let is_last_group = g == group_count - 1;
        let is_special_unit_name = ((g_inv % 2 != 0) && !is_last_group)
            || (in_currency && is_last_group);

        let group = Group {
            hundreds,
            tens,
            units,
        };
        if group.is_blank() {
            continue;
        }
        let group_info = group.to_str(is_special_unit_name);
        let str = group_info.converted;
        result.push(str);

        if !is_last_group {
            let group_name = match GROUP_NAMES.get(group_count - g - 2) {
                Some(name) => name,
                None => {
                    return Err(());
                }
            };
            let index = match group_info.plural_type {
                PluralType::One => 0,
                PluralType::TwoFour => 1,
                PluralType::FiveNine => {
                    if g + 2 == group_count {
                        0
                    } else {
                        group_name.len() - 1
                    }
                }
            };
            result.push(group_name[index].to_owned());
            result.push(" ".to_owned());
        }
    }
    return Ok(result);
}

struct Group {
    hundreds: Number,
    tens: Number,
    units: Number,
}

impl Group {
    fn to_str(&self, is_special_unit_name: bool) -> GroupInfo {
        let mut converted = String::new();
        let mut plural_type = PluralType::FiveNine;
        let group = vec![self.hundreds, self.tens, self.units];

        let last_digit = self.units;
        let mut digit_position = 3;

        for digit in group {
            let mut digit_value = digit as usize;

            match digit {
                Number::Zero => {
                    plural_type = PluralType::FiveNine;
                    digit_position -= 1;
                    continue;
                }
                Number::One => {
                    plural_type = PluralType::One;

                    // Check if the number ends with -naest (jedanaest, dvanaest, ...)
                    // Also check if current digit is at 2nd position (3, *1*, 4).
                    if digit_position == 2 && last_digit != Number::Zero {
                        plural_type = PluralType::FiveNine;
                        digit_value = last_digit as usize;

                        // special naest group
                        let primary_digits = PRIMARIES[0];
                        let digit = primary_digits[digit_value - 1];
                        converted.push_str(digit);
                        converted.push(' ');

                        // Skip to the end
                        break;
                    }

                    // In Croatian language, there are also used special
                    // word numbers in rare cases before thousand name.
                    if is_special_unit_name && digit_position == 1 {
                        let primary_digits = PRIMARIES[1]; // units
                        let digit = primary_digits[9]; // special word type
                        converted.push_str(digit);
                        converted.push(' ');

                        // Skip to the end
                        break;
                    }
                }
                Number::Two => {
                    plural_type = PluralType::TwoFour;
                    if is_special_unit_name && digit_position == 1 {
                        let primary_digits = PRIMARIES[1];
                        let digit = primary_digits[10];
                        converted.push_str(digit);
                        converted.push(' ');
                        break;
                    }
                }
                Number::Three => plural_type = PluralType::TwoFour,
                Number::Four => plural_type = PluralType::TwoFour,
                _ => plural_type = PluralType::FiveNine,
            }
            let primary_digits = PRIMARIES[digit_position];
            let digit = primary_digits[digit_value - 1];
            converted.push_str(digit);
            converted.push(' ');

            digit_position -= 1;
        }

        GroupInfo {
            converted,
            plural_type,
        }
    }

    fn is_blank(&self) -> bool {
        self.units == Number::Zero
            && self.tens == Number::Zero
            && self.hundreds == Number::Zero
    }
}

struct GroupInfo {
    converted: String,
    plural_type: PluralType,
}

fn load_input(input: &str) -> Result<(Vec<Number>, bool), ()> {
    let mut result = Vec::new();
    let mut in_currency = false;

    let mut input = input.trim().to_owned();
    let length = input.len();

    if length > 2 {
        let last_two = &input[length - 2..length];
        if last_two.eq("kn") {
            in_currency = true;
            input.pop();
            input.pop();
        }
    }

    let mut chars = input.chars();

    if !chars.all(|x| x.is_ascii_digit()) {
        return Err(());
    }

    let number_len = input.trim().len();
    let padding_count = 3 - (number_len % 3);
    if padding_count != 3 {
        for _ in 0..padding_count {
            result.push(Number::Zero);
        }
    }

    for c in input.trim().chars() {
        result.push(c.into());
    }

    Ok((result, in_currency))
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
            _ => panic!("Not a number!"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Number {
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

enum PluralType {
    One,
    TwoFour,
    FiveNine,
}

const PRIMARIES: &[&[&str]] = &[NAEST, UNITS, TENS, HUNDREDS];

const UNITS: &[&str] = &[
    "jedan", "dva", "tri", "četiri", "pet", "šest", "sedam", "osam", "devet",
    "jedna", "dvije",
];

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

const GROUP_NAMES: &[&[&str]] = &[
    THOUSANDS, MILIJUNI, MILIJARDE, BILIJUNI, BILIJARDE, TRILIJUNI, TRILIJARDE,
];

const THOUSANDS: &[&str] = &["tisuća", "tisuće"];

const MILIJUNI: &[&str] = &["milijun", "milijuna"];

const MILIJARDE: &[&str] = &["milijarda", "milijarde", "milijardi"];

const BILIJUNI: &[&str] = &["bilijun", "bilijuna"];

const BILIJARDE: &[&str] = &["bilijarda", "bilijarde", "bilijardi"];

const TRILIJUNI: &[&str] = &["trilijun", "trilijuna"];

const TRILIJARDE: &[&str] = &["trilijarda", "trilijarde", "trilijardi"];

const PRIMARY_UNITS_COUNT: usize = 9;

const PRIMARIES: &[&[&str]] = &[NAEST, UNITS, TENS, HUNDREDS];

const UNITS: &[&str] = &[
    "jedan", "dva", "tri", "četiri", "pet", "šest", "sedam", "osam", "devet", "jedna", "dvije",
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

fn main() {
    println!("Upisi broj!");
    let nums = match get_input() {
        Ok(n) => n,
        Err(_) => return,
    };

    let length = nums.len();
    let mut num_iter = nums.iter().peekable();
    let groups = (length as f32 / 3.0).ceil() as usize;
    let mut group = groups;
    let mut weight = if length % 3 == 0 { 3 } else { length % 3 };

    // The provided number is split into groups with 3 members starting
    // from the smallest number weight. Last group, which has the highest
    // weight can have less than 3 members.
    // Loop starts from the group with the highest the group with the
    // lowest weight. Every time a group is finished

    while let Some(&num) = num_iter.next() {
        let mut number = num as usize;
        let plural_type;

        match num {
            Number::Zero => continue,
            Number::One => {
                plural_type = PluralType::One;
                // Check if the number ends with -naest (jedanaest, dvanaest, ...)
                if weight != 1 && (weight + 1) % 3 == 0 {
                    if let Some(&&peek_num) = num_iter.peek() {
                        if peek_num != Number::Zero {
                            weight = 0;
                            number = peek_num as usize;
                            // Skip the next number
                            num_iter.next().unwrap();
                        }
                    }
                }
            }
            Number::Two => {
                plural_type = PluralType::TwoFour;
            }
            Number::Three => {
                plural_type = PluralType::TwoFour;
            }
            Number::Four => {
                plural_type = PluralType::TwoFour;
            }
            _ => {
                plural_type = PluralType::FiveNine;
            }
        }
        // Print primary digits (units, tens, thousands)
        let primary_group = PRIMARIES[weight];
        let digit = primary_group[number - 1];
        print!("{} ", digit);

        weight -= 1;

        if weight == 0 {
            // If it's not the last group, print the group name
            // (thousands, millions, ...)
            if group > 1 {
                let g = GROUP_NAMES[group - 2];
                let group_name = match plural_type {
                    PluralType::One => g.first().unwrap(),
                    PluralType::TwoFour => g.last().unwrap(),
                    PluralType::FiveNine => g.last().unwrap(),
                };
                print!("{} ", group_name);
                group -= 1;
            }

            weight = 3;
        }
    }
}

fn get_input() -> Result<Vec<Number>, ()> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => {
            println!("Greska pri upisu!");
            return Err(());
        }
    }

    let mut result = Vec::new();
    let mut chars = input.trim().chars();
    // Check if all chars are digits
    if !chars.all(|x| x.is_ascii_digit()) {
        println!("Upisan je znak koji nije znamenka!");
        return Err(());
    }

    for c in input.trim().chars() {
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
            _ => panic!("Not a number!"),
        }
    }
}

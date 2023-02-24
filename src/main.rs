use colored::Colorize;

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
    let input = match get_input() {
        Ok(i) => i,
        Err(_) => return,
    };

    let nums = match parse_input(input) {
        Ok(n) => n,
        Err(_) => return,
    };

    if nums.len() > MAX_NUMBER_LEN {
        println!(
            "Veličina broja s više od {} znamenki još nije uprogramirana!",
            MAX_NUMBER_LEN
        );
        return;
    }

    // The provided number is split into groups with 3 members starting
    // from the highest number weight. Every time a group is finished it
    // checks if the group name should be printed.

    print!("Konvertirano: \n\t\t");

    let length = nums.len();
    let group_count = (length as f32 / 3.0).ceil() as usize;

    for g in 0..group_count {
        let offset = 3 * g;

        // Group of 3 elements
        let hundreds = nums[offset];
        let tens = nums[1 + offset];
        let units = nums[2 + offset];
        let group = [hundreds, tens, units];

        // Check if group's name will be tisuća, milijarda, bilijarda, trilijarda, ...
        // Used for checking special word cases
        // Second part of the check filters out the last group.
        let g_inverse = group_count - g - 1;
        let is_last_group = g == group_count - 1;
        let is_special_unit_name = (g_inverse % 2 != 0) && !is_last_group;

        let (converted, plural_type) = convert(group, is_special_unit_name);
        print!("{}", converted.yellow());

        // Check if group name should be printed. Checks if a group has any numbers other
        // than zero and if it is not the last group.
        if !is_blank(group) && !is_last_group {
            // Get the belonging group. Exits the program if the number is too high.
            let group_name = GROUP_NAMES.get(g_inverse - 1).unwrap();
            // Indexing the right wording for the group that depends on the unit of a group.
            let index = match plural_type {
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
            print!("{} ", group_name[index].green());
        }
    }
    println!("\n");
}

fn convert(
    group: [Number; 3],
    is_special_unit_name: bool,
) -> (String, PluralType) {
    let mut converted = String::new();
    let mut plural_type = PluralType::FiveNine;

    let last_digit = group[2];

    for (i, digit) in group.iter().enumerate() {
        let digit_position = 2 - i;
        let mut digit_value = *digit as usize;

        match digit {
            Number::Zero => {
                plural_type = PluralType::FiveNine;
                continue;
            }
            Number::One => {
                plural_type = PluralType::One;

                // Check if the number ends with -naest (jedanaest, dvanaest, ...)
                // Also check if current digit is at 2nd position (3, *1*, 4).
                if digit_position == 1 && last_digit != Number::Zero {
                    plural_type = PluralType::FiveNine;
                    digit_value = last_digit as usize;

                    // special naest group
                    let primary_digits = PRIMARIES[0];
                    let digit = primary_digits[digit_value - 1];
                    converted.push_str(digit);
                    converted.push(' ');

                    break;
                }

                // In Croatian language, there are also used special
                // word numbers in rare cases before thousand name.
                if is_special_unit_name && digit_position == 0 {
                    let units = PRIMARIES[1];
                    let special_digit = units[9];
                    converted.push_str(special_digit);
                    converted.push(' ');

                    break;
                }
            }
            Number::Two => {
                plural_type = PluralType::TwoFour;
                // In Croatian language, there are also used special
                // word numbers in rare cases before thousand name.
                if is_special_unit_name && digit_position == 0 {
                    let units = PRIMARIES[1];
                    let special_digit = units[10];
                    converted.push_str(special_digit);
                    converted.push(' ');

                    // Skip to the end
                    break;
                }
            }
            Number::Three => plural_type = PluralType::TwoFour,
            Number::Four => plural_type = PluralType::TwoFour,
            _ => plural_type = PluralType::FiveNine,
        }
        // Print primary digits (units, tens, hundreds)
        let primary_digits = PRIMARIES[digit_position + 1];
        let digit = primary_digits[digit_value - 1];
        converted.push_str(digit);
        converted.push(' ');
    }

    (converted, plural_type)
}

fn is_blank(group: [Number; 3]) -> bool {
    group.iter().all(|&n| n == Number::Zero)
}

fn parse_input(input: String) -> Result<Vec<Number>, ()> {
    let mut result = Vec::new();

    let input = input.trim();
    let mut chars = input.chars();

    // Check if all chars are digits
    if !chars.all(|x| x.is_ascii_digit()) {
        println!("Upisan je znak koji nije znamenka!");
        return Err(());
    }

    // Fill padding with zeroes. Converts: [1|4,7,9] to [0,0,1|4,7,9]
    let number_count = input.len();
    let unfilled = 3 - number_count % 3;
    if unfilled != 3 {
        for _ in 0..unfilled {
            result.push(Number::Zero);
        }
    }

    for c in input.chars() {
        result.push(c.into());
    }

    Ok(result)
}

fn get_input() -> Result<String, ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        return Ok(args.get(1).unwrap().to_owned());
    }

    println!("Invalid arguments!");
    Err(())
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

const MAX_NUMBER_LEN: usize = (GROUP_NAMES.len() + 1) * 3;

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

#[test]
fn test1() {
    let hundreds = Number::Four;
    let tens = Number::One;
    let units = Number::Three;
    let thousands = false;

    let g = [hundreds, tens, units];
    assert_eq!(convert(g, thousands).0, "četiristo trinaest ");
}

#[test]
fn test2() {
    let hundreds = Number::Zero;
    let tens = Number::Zero;
    let units = Number::Three;
    let thousands = false;

    let g = [hundreds, tens, units];
    assert_eq!(convert(g, thousands).0, "tri ");
}

#[test]
fn test3() {
    let hundreds = Number::Zero;
    let tens = Number::Three;
    let units = Number::Three;
    let thousands = false;

    let g = [hundreds, tens, units];
    assert_eq!(convert(g, thousands).0, "trideset tri ");
}

#[test]
fn test4() {
    let hundreds = Number::Zero;
    let tens = Number::One;
    let units = Number::Six;
    let thousands = false;

    let g = [hundreds, tens, units];
    assert_eq!(convert(g, thousands).0, "šesnaest ");
}

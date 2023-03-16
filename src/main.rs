#[cfg(test)]
mod tests;
mod consts;

use colored::{ColoredString, Colorize};
use consts::{PluralType, GROUP_NAMES, Number, HUNDREDS, TENS, NAEST, ALT_UNITS, UNITS};

fn main() -> Result<(), ()> {
    let input = get_input()?;
    println!("Konvertirano: \n\t\t");
    for str in num_word_conv(input)? {
        print!("{}", str);
    }
    Ok(())
}

#[allow(clippy::result_unit_err)]
pub fn num_word_conv<S: AsRef<str>>(
    input: S,
) -> Result<Vec<ColoredString>, ()> {
    let mut result: Vec<ColoredString> = Vec::new();
    let nums = parse_input(input.as_ref())?;

    if nums.len() > consts::MAX_NUMBER_LEN {
        println!(
            "Veličina broja s više od {} znamenki još nije uprogramirana!",
            consts::MAX_NUMBER_LEN
        );
        return Err(());
    }

    // The provided number is split into groups with 3 members starting
    // from the highest number weight. Every time a group is finished it
    // checks if the group name should be printed.

    let group_count = (nums.len() as f32 / 3.0).ceil() as usize;

    for g in 0..group_count {
        let offset = 3 * g;

        // Group of 3 elements: ['hundreds', 'tens', 'units'].
        let group = [nums[offset], nums[offset + 1], nums[offset + 2]];

        if is_blank(group) {
            continue;
        }

        // Used for checking special word cases.
        // Every second group starting from the right can have
        // a special, alternate, unit name. [242,*345*,576,*345*,243]
        let g_inverse = group_count - g - 1;
        let is_last_group = g == (group_count - 1);
        let is_special_unit_name = (g_inverse % 2 != 0) && !is_last_group;

        let (converted, plural_type) = convert(group, is_special_unit_name);
        result.push(converted.yellow());

        // Check if group name should be printed. Checks if a group has any numbers other
        // than zero and if it is not the last group.
        if !is_last_group {
            // Get the belonging group.
            let group_name = GROUP_NAMES[g_inverse - 1];

            let name = match plural_type {
                // Appropriate group plural name for unit '1' is positioned
                // at the index '0' of a 'group_name'.
                PluralType::One => group_name[0],
                // Appropriate group plural name for units '2', '3' and '4' is positioned
                // at the index of '1' of a 'group_name'.
                PluralType::TwoFour => group_name[1],
                // Appropriate group plural name for units '5', '6', '7', '8', '9'
                // is positioned at the last index of a 'group_name'.
                // If the group name should be 'thousand', in this case, the appropriate
                // group plural name is placed at the index of '0'.
                PluralType::FiveNine => {
                    if g_inverse == 1 {
                        group_name[0]
                    } else {
                        group_name[group_name.len() - 1]
                    }
                }
            }
            .green();
            result.push(name);
            result.push(" ".clear());
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

    // Convert the hundreds:
    let hundreds_val = *group.first().unwrap();
    match hundreds_val {
        Number::Zero => (),
        v => {
            add_digit(HUNDREDS[v as usize - 1]);
        }
    }

    // Convert the tens:
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
            return (converted, plural_type); // returns immediately skipping the units
        }
        v => add_digit(TENS[v as usize - 1]),
    }

    // Convert the units (if the tens digit is not '1'):
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

    // Check if all chars are digits
    if !chars.clone().all(|x| x.is_ascii_digit()) {
        println!("Upisan je znak koji nije znamenka!");
        return Err(());
    }

    // Fill padding with zeroes. Converts: [1,4,7,9] to [0,0,1|4,7,9]
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

fn get_input() -> Result<String, ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        return Ok(args.get(1).unwrap().to_owned());
    }

    println!("Invalid arguments!");
    Err(())
}
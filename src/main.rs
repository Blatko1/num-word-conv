mod consts;
#[cfg(test)]
mod tests;

use colored::{ColoredString, Colorize};
use consts::{
    Number, PluralType, ALT_UNITS, GROUP_NAMES, HUNDREDS, NAEST, TENS, UNITS,
};

fn main() {
    let input = get_input().unwrap();
    print!("Konvertirano: \n\t cijeli dio: ");
    let (whole, decimals) = num_word_conv(input).unwrap();
    for str in whole {
        print!("{}", str);
    }
    print!("\n\t decimalni dio: ");
    if let Some(str) = decimals {
        print!("{}", str);
    }
    println!();
}

#[allow(clippy::result_unit_err)]
pub fn num_word_conv<S: AsRef<str>>(
    input: S,
) -> Result<(Vec<ColoredString>, Option<ColoredString>), &'static str> {
    let mut result: Vec<ColoredString> = Vec::new();
    let mut decimal_result = None;
    let (whole, decimal) = parse_input(input.as_ref())?;

    if whole.len() > consts::MAX_NUMBER_LEN {
        return Err(
            "Veličina broja s više od 66 znamenki još nije uprogramirana!",
        );
    }

    // The provided number is split into groups with 3 members starting
    // from the highest number weight. Every time a group is finished it
    // checks if the group name should be printed.

    let group_count = (whole.len() as f32 / 3.0).ceil() as usize;

    for g in 0..group_count {
        let offset = 3 * g;

        // Group of 3 elements: ['hundreds', 'tens', 'units'].
        let group = [whole[offset], whole[offset + 1], whole[offset + 2]];

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
    if !decimal.is_empty() {
        let group = [Number::Zero, decimal[0], decimal[1]];
        let (converted, _) = convert(group, false);
        decimal_result = Some(converted.yellow());
    }
    Ok((result, decimal_result))
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

fn parse_input(input: &str) -> Result<(Vec<Number>, Vec<Number>), &'static str> {
    let mut whole = Vec::new();
    let mut decimal = Vec::new();
    let mut input = input.trim().to_owned();

    // Check if all characters are digits
    if !input
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, ' ' | '_' | '.' | ','))
    {
        return Err("Upisan je znak koji nije znamenka ili '_', '.' i ','!");
    }

    // Remove spaces, underscores and points.
    input.retain(|c| !matches!(c, ' ' | '_' | '.'));
    if input.is_empty() {
        return Err("Nije upisan nikakav broj!");
    }

    // Split the number into the whole and the decimal part. If the number is not
    // decimal take the number as whole.
    let whole_part: &str;
    let decimal_part: &str;
    let mut chars = input.chars();
    if let Some(p) = chars.position(|c| c == ',') {
        if chars.any(|c| c == ',') {
            return Err("Upisano je više decimalnih zareza!");
        }
        let split = input.split_at(p);
        (whole_part, decimal_part) = (split.0, split.1.split_at(1).1);
    } else {
        whole_part = input.as_ref();
        decimal_part = "";
    }

    // Convert the whole and the decimal part into Number types.
    if !whole_part.is_empty() {
        // Fill padding with zeroes. Converts: [1,4,7,9] to [0,0,1|4,7,9]
        let number_count = whole_part.len();
        let unfilled = 3 - number_count % 3;
        if unfilled != 3 {
            for _ in 0..unfilled {
                whole.push(Number::Zero);
            }
        }

        for c in whole_part.chars() {
            whole.push(c.into());
        }
    }
    if !decimal_part.is_empty() {
        // The decimal part can only have two decimals
        // Decimal number 1,1 has the decimal part [1,0]
        if decimal_part.len() > 2 {
            return Err(
                "Decimalni dio broja ne smije biti duži od dvije znamenke",
            );
        }
        let mut decimal_chars = decimal_part.chars();
        decimal.push(decimal_chars.next().unwrap().into());
        if decimal_part.len() == 1 {
            decimal.push(Number::Zero);
        }else {
        decimal.push(decimal_chars.next().unwrap().into());
        }
    }

    Ok((whole, decimal))
}

fn get_input() -> Result<String, ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        return Ok(args.get(1).unwrap().to_owned());
    }

    println!("Invalid arguments!");
    Err(())
}

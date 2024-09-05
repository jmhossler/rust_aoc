advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let calibration_values = parse_input(input, &["1", "2", "3", "4", "5", "6", "7", "8", "9"]);

    // Return the sum of all the calibration values
    return calibration_values.map(|values| values.iter().sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let calibration_values = parse_input(
        input,
        &[
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ],
    );

    // Return the sum of all the calibration values
    return calibration_values.map(|values| values.iter().sum());
}

// extract_calibration_value parses the integer from the string. The calibration
// value is the first and last integer in the string combined in that order.
fn extract_calibration_value(input: &str, substrings: &[&str]) -> Option<u32> {
    // find the first substring that occurs in the input
    let first_substring = find_first_substring(input, substrings);
    // find the last substring that occurs in the input
    let last_substring = find_last_substring(input, substrings);

    // if both substrings are found, combine them into a single string and parse it as an integer
    if first_substring.is_some() && last_substring.is_some() {
        let numbers = format!(
            "{}{}",
            replace_with_digit(&first_substring.unwrap()).unwrap(),
            replace_with_digit(&last_substring.unwrap()).unwrap()
        );
        let calibration_value = numbers.parse::<u32>().ok();
        return calibration_value;
    }
    None
}

// find first occurence of any of the provided substrings in the input and return that substring.
// Multiple substrings might be in the input, but only the one that occurs first in the input
// should be returned.
fn find_first_substring(input: &str, substrings: &[&str]) -> Option<String> {
    // iterate from the start of the input to the end, at each point looking if any of the substrings are found
    for i in 0..input.len() {
        // iterate over each substring
        for substring in substrings {
            // check if the substring is found at the current index
            if input[i..].starts_with(substring) {
                return Some(substring.to_string());
            }
        }
    }

    None
}

// similar to find_first_substring, but returns the last occurence of any of the substrings
fn find_last_substring(input: &str, substrings: &[&str]) -> Option<String> {
    // iterate from the end of the input to the start, at each point looking if any of the substrings are found
    for i in (0..input.len()).rev() {
        // iterate over each substring
        for substring in substrings {
            // check if the substring is found at the current index
            if input[i..].starts_with(substring) {
                return Some(substring.to_string());
            }
        }
    }

    None
}

fn replace_with_digit(input: &str) -> Option<String> {
    let digit_conversions = vec![
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for (word, digit) in digit_conversions {
        if input.contains(word) || input.contains(digit) {
            return Some(digit.to_string());
        }
    }

    None
}

// should take both the input and a function that turns string into a calibration value
fn parse_input(input: &str, substrings: &[&str]) -> Option<Vec<u32>> {
    let mut calibration_values = Vec::new();

    for line in input.lines() {
        let calibration_value = extract_calibration_value(line, substrings);
        if let Some(value) = calibration_value {
            calibration_values.push(value);
        }
    }

    Some(calibration_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, "pt1"));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, "pt2"));
        assert_eq!(result, Some(281));
    }
}

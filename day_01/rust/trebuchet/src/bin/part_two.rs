use regex::Regex;

fn main() {
    let puzzle_input = include_str!("puzzle_input.txt");
    let total: u32 = puzzle_input.lines().map(extract_calibration_number).sum();

    println!("The total is {}", total);
}

fn extract_calibration_number(line: &str) -> u32 {
    let pattern = Regex::new(r"[0-9]|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let mut digits = pattern.find_iter(line).map(|m| m.as_str());

    fn digit_to_number(d: &str) -> u32 {
        match d {
            "zero" | "0" => 0,
            "one" | "1" => 1,
            "two" | "2" => 2,
            "three" | "3" => 3,
            "four" | "4" => 4,
            "five" | "5" => 5,
            "six" | "6" => 6,
            "seven" | "7" => 7,
            "eight" | "8" => 8,
            "nine" | "9" => 9,
            _ => panic!("not a digit"),
        }
    }

    let first_digit = digits
        .next()
        .map(digit_to_number)
        .expect("No digits in the line");

    let last_digit = digits.last().map(digit_to_number).unwrap_or(first_digit);

    first_digit * 10 + last_digit
}

#[cfg(test)]
mod tests {
    use super::extract_calibration_number;

    #[test]
    fn rules_from_part_1_apply() {
        assert_eq!(extract_calibration_number("1abc2"), 12);
        assert_eq!(extract_calibration_number("pqr3stu8vwx"), 38);
        assert_eq!(extract_calibration_number("a1b2c3d4e5f"), 15);
        assert_eq!(extract_calibration_number("treb7uchet"), 77);
        assert_eq!(extract_calibration_number("l1ne"), 11);
    }

    #[test]
    fn digits_can_be_spelled() {
        assert_eq!(extract_calibration_number("two1nine"), 29);
        assert_eq!(extract_calibration_number("eighttwothree"), 83);
        assert_eq!(extract_calibration_number("abcone2threexyz"), 13);
        assert_eq!(extract_calibration_number("zoneight234"), 14);
    }
}

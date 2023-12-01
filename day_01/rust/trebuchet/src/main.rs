fn main() {
    let puzzle_input = include_str!("puzzle_input.txt");
    let total: u32 = puzzle_input.lines().map(extract_calibration_number).sum();

    println!("The answer is: {}", total);
}

fn extract_calibration_number(line: &str) -> u32 {
    let mut digits = line.chars().filter(char::is_ascii_digit);

    let first_digit = digits
        .next()
        .map(|c| c.to_digit(10).unwrap())
        .expect("No digits in the string");

    let last_digit = digits
        .last()
        .map(|c| c.to_digit(10).unwrap())
        .unwrap_or(first_digit);

    first_digit * 10 + last_digit
}

#[cfg(test)]
mod tests {
    use crate::extract_calibration_number;

    #[test]
    fn calibration_numbers_are_made_from_the_first_and_last_numbers() {
        assert_eq!(extract_calibration_number("1abc2"), 12);
        assert_eq!(extract_calibration_number("pqr3stu8vwx"), 38);
        assert_eq!(extract_calibration_number("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn calibration_numbers_use_the_same_digit_if_line_contains_only_one_digit() {
        assert_eq!(extract_calibration_number("treb7uchet"), 77);
        assert_eq!(extract_calibration_number("l1ne"), 11);
    }
}

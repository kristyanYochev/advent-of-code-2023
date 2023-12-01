fn main() {}

fn extract_calibration_number(line: &str) -> u32 {
    todo!();
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
}

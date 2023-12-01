pub fn compute_calibration_value(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let d1c = first_digit(line.chars());
            let d2c = first_digit(line.chars().rev());
            format!("{}{}", d1c, d2c).parse().unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

fn first_digit<T: Iterator<Item = char>>(mut chars: T) -> char {
    chars.find(|c| c.is_digit(10)).unwrap()
}

pub fn compute_calibration_value_2(input: String) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use crate::day_1;

    #[test]
    fn example() {
        let input = String::from(
            "1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet",
        );
        let actual = day_1::compute_calibration_value(input);
        let expected = 142;
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_example() {
        let input = String::from(
            "two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen",
        );
        let actual = day_1::compute_calibration_value_2(input);
        let expected = 281;
        assert_eq!(actual, expected);
    }
}

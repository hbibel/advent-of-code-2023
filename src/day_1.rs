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
    input
        .lines()
        .map(|line| {
            let line = line.to_string();
            let d1 = first_digit_2(&line);
            let d2 = last_digit(&line);
            format!("{}{}", d1, d2).parse().unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

const TOKEN_TO_VALUE: &[(&str, i32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn first_digit_2(line: &String) -> i32 {
    let digit_tokens = TOKEN_TO_VALUE.iter().map(|t| t.0);

    let (t, _) = digit_tokens
        .filter_map(|t| line.find(t).map(|i| (t, i)))
        .min_by(|t1, t2| t1.1.cmp(&t2.1))
        .unwrap();
    TOKEN_TO_VALUE.iter().find(|(k, _)| *k == t).unwrap().1
}

fn last_digit(line: &String) -> i32 {
    let digit_tokens = TOKEN_TO_VALUE.iter().map(|t| t.0);

    let (t, _) = digit_tokens
        .filter_map(|t| line.rfind(t).map(|i| (t, i)))
        .max_by(|t1, t2| t1.1.cmp(&t2.1))
        .unwrap();
    TOKEN_TO_VALUE.iter().find(|(k, _)| *k == t).unwrap().1
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

pub fn ways_to_win(input: String) -> u64 {
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());
    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());
    let records: Vec<(u64, u64)> = times.zip(distances).collect();
    acc_ways_to_win(records.iter())
}

pub fn ways_to_win_2(input: String) -> u64 {
    // Sure, there's a nicer way with a formula, but brute force works fast
    // enough for the given input.
    let time = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = input
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let records: Vec<(u64, u64)> = Vec::from([(time, distance)]);
    acc_ways_to_win(records.iter())
}

fn acc_ways_to_win<'a, T>(records: T) -> u64
where
    T: Iterator<Item = &'a (u64, u64)>,
{
    records.fold(1, |acc, (time, record_distance)| {
        let time = time.clone();
        let record_distance = record_distance.clone();

        let ways_to_beat = (0..=time)
            .filter(|button_time| {
                let distance = (time - button_time) * button_time;
                distance > record_distance
            })
            .count() as u64;
        acc * ways_to_beat
    })
}

#[cfg(test)]
mod tests {
    use super::ways_to_win;
    use super::ways_to_win_2;

    #[test]
    fn example_1() {
        let input = String::from(
            "Time:      7  15   30
             Distance:  9  40  200",
        );
        let expected = 288;
        let actual = ways_to_win(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let input = String::from(
            "Time:      7  15   30
             Distance:  9  40  200",
        );
        let expected = 71503;
        let actual = ways_to_win_2(input);
        assert_eq!(actual, expected);
    }
}

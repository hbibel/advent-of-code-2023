pub fn ways_to_win(input: String) -> u32 {
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());
    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());
    let records: Vec<(u32, u32)> = times.zip(distances).collect();
    records.iter().fold(1, |acc, (time, record_distance)| {
        let time = time.clone();
        let record_distance = record_distance.clone();

        let ways_to_beat = (0..=time)
            .filter(|button_time| {
                let distance = (time - button_time) * button_time;
                distance > record_distance
            })
            .count() as u32;
        acc * ways_to_beat
    })
}

#[cfg(test)]
mod tests {
    use super::ways_to_win;

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
}

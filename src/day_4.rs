use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn pile_worth(input: String) -> i32 {
    input.lines().fold(0, |sum, line| {
        let re = Regex::new(r".*: ([\d ]+) \| ([\d ]+)").unwrap();
        let (_, [winning_sec, own_sec]) = re
            .captures_iter(line)
            .map(|cap| cap.extract())
            .next()
            .unwrap();

        let winning: HashSet<i32> = HashSet::from_iter(
            winning_sec
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()),
        );
        let own: HashSet<i32> = HashSet::from_iter(
            own_sec
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()),
        );

        let won = winning.intersection(&own).count();
        if won == 0 {
            sum
        } else {
            let base: i32 = 2;
            base.pow(won as u32 - 1) + sum
        }
    })
}

pub fn num_cards(input: String) -> i32 {
    let mut card_num_to_count: HashMap<i32, i32> = HashMap::new();
    input.lines().for_each(|line| {
        let re = Regex::new(r".* (\d+): ([\d ]+) \| ([\d ]+)").unwrap();
        let (_, [card_no_str, winning_sec, own_sec]) = re
            .captures_iter(line)
            .map(|cap| cap.extract())
            .next()
            .unwrap();

        let card_no = card_no_str.parse::<i32>().unwrap();
        let winning: HashSet<i32> = HashSet::from_iter(
            winning_sec
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()),
        );
        let own: HashSet<i32> = HashSet::from_iter(
            own_sec
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()),
        );

        // If card has not been won before, insert 1 here
        card_num_to_count
            .entry(card_no)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        let won = winning.intersection(&own).count() as i32;
        let mult = card_num_to_count.get(&card_no).unwrap_or(&1).clone();
        (card_no + 1..=card_no + won).for_each(|won_card_no| {
            card_num_to_count
                .entry(won_card_no)
                .and_modify(|count| *count += mult)
                .or_insert(mult);
        });
    });
    card_num_to_count.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::day_4::num_cards;

    use super::pile_worth;

    #[test]
    fn example_1() {
        let input = String::from(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
             Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
             Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
             Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
             Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
             Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        let expected = 13;
        let actual = pile_worth(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_2() {
        let input = String::from(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
             Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
             Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
             Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
             Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
             Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        let expected = 30;
        let actual = num_cards(input);
        assert_eq!(expected, actual);
    }
}

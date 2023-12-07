use std::collections::HashMap;

#[derive(Eq, PartialEq, Ord, Clone, Copy, Debug)]
struct Hand {
    hand_type: HandType,
    cards: [char; 5],
}

impl Hand {
    fn from(cards: [char; 5]) -> Hand {
        let hand_type = {
            let mut card_to_count: HashMap<char, i32> = HashMap::new();
            for c in cards {
                card_to_count
                    .entry(c)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            let jokers: i32 = *card_to_count.get(&'J').unwrap_or(&0);
            card_to_count.remove(&'J');

            if jokers == 5 || card_to_count.values().any(|count| (*count + jokers) == 5) {
                HandType::FiveOak
            } else if card_to_count.values().any(|count| (*count + jokers) == 4) {
                HandType::FourOak
            } else if card_to_count.values().any(|count| *count == 3)
                && card_to_count.values().any(|count| *count == 2)
                || card_to_count.values().filter(|count| **count == 2).count() == 2 && jokers == 1
            {
                HandType::FullHouse
            } else if card_to_count.values().any(|count| (*count + jokers) == 3) {
                HandType::ThreeOak
            } else if card_to_count.values().filter(|count| **count == 2).count() == 2
                || card_to_count.values().any(|count| *count == 2) && jokers == 1
            {
                HandType::TwoPair
            } else if card_to_count.values().any(|count| *count == 2) || jokers == 1 {
                HandType::OnePair
            } else {
                assert_eq!(jokers, 0, "Hand {:?} not parsed correctly", cards);
                HandType::HighCard
            }
        };
        Hand { hand_type, cards }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        for i in 0..5 {
            match card_to_val(self.cards[i]).partial_cmp(&card_to_val(other.cards[i])) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }
        Some(core::cmp::Ordering::Equal)
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
enum HandType {
    // sorted from lowest to highest value
    HighCard,
    OnePair,
    TwoPair,
    ThreeOak,
    FullHouse,
    FourOak,
    FiveOak,
}

pub fn total_winnings(input: String) -> u32 {
    let mut games: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            let cards = &parts.get(0).unwrap()[..5];
            let cards: [char; 5] = cards.chars().collect::<Vec<char>>().try_into().unwrap();
            let hand = Hand::from(cards);

            let bid = parts.get(1).unwrap().parse::<u32>().unwrap();
            (hand, bid)
        })
        .collect();
    games.sort_by_key(|tup| tup.0.clone());
    games
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as u32 + 1) * bid)
}

fn card_to_val(card: char) -> u32 {
    if card.is_numeric() {
        card.to_digit(10).unwrap()
    } else {
        match card {
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            o => panic!("Unexpected card {}", o),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_7_2::total_winnings;

    #[test]
    fn example_1() {
        let input = String::from(
            "32T3K 765
             T55J5 684
             KK677 28
             KTJJT 220
             QQQJA 483",
        );
        let expected = 5905;
        let actual = total_winnings(input);
        assert_eq!(actual, expected);
    }
}

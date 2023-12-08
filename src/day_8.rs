use std::collections::HashMap;

pub fn steps_to_zzz(input: String) -> usize {
    let steps: Vec<char> = input.lines().nth(0).unwrap().chars().collect();
    let num_steps = steps.len();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let from = line.chars().take(3).collect::<String>();
        let to = (
            line.chars().skip(7).take(3).collect::<String>(),
            line.chars().skip(12).take(3).collect::<String>(),
        );
        map.insert(from, to);
    });

    let mut count = 0;
    let mut curr = &String::from("AAA");
    while curr != &String::from("ZZZ") {
        curr = map
            .get(curr)
            .map(|(l, r)| {
                if steps[count % num_steps] == 'R' {
                    r
                } else {
                    l
                }
            })
            .expect(format!("No next step from {}", curr).as_str());
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::day_8::steps_to_zzz;

    #[test]
    fn example_1() {
        let input = String::from(
            "RL\n\
             \n\
             AAA = (BBB, CCC)\n\
             BBB = (DDD, EEE)\n\
             CCC = (ZZZ, GGG)\n\
             DDD = (DDD, DDD)\n\
             EEE = (EEE, EEE)\n\
             GGG = (GGG, GGG)\n\
             ZZZ = (ZZZ, ZZZ)",
        );
        let expected = 2;
        let actual = steps_to_zzz(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let input = String::from(
            "LLR\n\
             \n\
             AAA = (BBB, BBB)\n\
             BBB = (AAA, ZZZ)\n\
             ZZZ = (ZZZ, ZZZ)",
        );
        let expected = 6;
        let actual = steps_to_zzz(input);
        assert_eq!(actual, expected);
    }
}

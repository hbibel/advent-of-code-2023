use std::collections::HashMap;

pub fn steps_to_zzz(input: String) -> usize {
    walk(
        input,
        |s| s == &&String::from("AAA"),
        |s| s == &&String::from("ZZZ"),
    )
}

pub fn steps_to_z(input: String) -> usize {
    walk(input, |s| s.ends_with("A"), |s| s.ends_with("Z"))
}

fn walk(input: String, start_cond: fn(&&String) -> bool, end_cond: fn(&&String) -> bool) -> usize {
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

    let counts: Vec<usize> = map
        .keys()
        .filter(start_cond)
        .map(|mut curr| {
            let mut count = 0;
            while !end_cond(&curr) {
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
        })
        .collect();
    counts.iter().fold(1, |x, y| lcm(x, *y))

    // Brute force is a tad too slow:
    // let mut count = 0;
    // let mut curr: Vec<&String> = map.keys().filter(start_cond).collect();
    //
    // while !curr.iter().all(end_cond) {
    //     curr = curr
    //         .iter()
    //         .map(|c| {
    //             map.get(*c)
    //                 .map(|(l, r)| {
    //                     if steps[count % num_steps] == 'R' {
    //                         r
    //                     } else {
    //                         l
    //                     }
    //                 })
    //                 .expect(format!("No next step from {}", c).as_str())
    //         })
    //         .collect();
    //     count += 1;
    // }
    // count
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while x != 0 {
        (x, y) = (y % x, x);
    }
    y
}

#[cfg(test)]
mod tests {
    use crate::day_8::steps_to_z;
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

    #[test]
    fn example_3() {
        let input = String::from(
            "LR\n\
             \n\
             11A = (11B, XXX)\n\
             11B = (XXX, 11Z)\n\
             11Z = (11B, XXX)\n\
             22A = (22B, XXX)\n\
             22B = (22C, 22C)\n\
             22C = (22Z, 22Z)\n\
             22Z = (22B, 22B)\n\
             XXX = (XXX, XXX)",
        );
        let expected = 6;
        let actual = steps_to_z(input);
        assert_eq!(actual, expected);
    }
}

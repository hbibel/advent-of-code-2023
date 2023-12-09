pub fn sum_interpolations(input: String) -> (i32, i32) {
    let mut sum_appended = 0;
    let mut sum_prepended = 0;

    input.lines().for_each(|line| {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut numss: Vec<Vec<i32>> = vec![nums];
        while numss.last().unwrap().iter().any(|n| n != &0) {
            let last = numss.last().unwrap();
            let der = last
                .iter()
                .zip(last.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>();
            numss.push(der);
        }
        // last element in numss is all 0 now
        let (appended, prepended) =
            numss
                .iter()
                .enumerate()
                .rev()
                .fold((0, 0), |acc, (level, ns)| {
                    if level == 0 {
                        acc
                    } else {
                        let prev_nums = numss.get(level - 1).unwrap();
                        (
                            acc.0 + ns.last().unwrap() + prev_nums[ns.len() - 1],
                            prev_nums[0] - acc.1,
                        )
                    }
                });
        sum_appended += appended;
        sum_prepended += prepended;
    });

    (sum_appended, sum_prepended)
}

#[cfg(test)]
mod tests {
    use crate::day_9::sum_interpolations;

    #[test]
    fn example_1() {
        let input = String::from(
            "0 3 6 9 12 15\n\
             1 3 6 10 15 21\n\
             10 13 16 21 30 45",
        );
        let expected = (114, 2);
        let actual = sum_interpolations(input);
        assert_eq!(actual, expected);
    }
}

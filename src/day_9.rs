pub fn sum_interpolations(input: String) -> i32 {
    let mut sum = 0;

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
        let next_num = numss.iter().enumerate().fold(0, |acc, (level, ns)| {
            if level == 0 {
                acc
            } else {
                let prev_nums = numss.get(level - 1).unwrap();
                acc + ns.last().unwrap() + prev_nums[ns.len() - 1]
            }
        });
        sum += next_num;
    });

    sum
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
        let expected = 114;
        let actual = sum_interpolations(input);
        assert_eq!(actual, expected);
    }
}

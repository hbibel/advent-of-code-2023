#[derive(Debug)]
struct Number {
    value: i32,
    y: i32,
    x_from: i32,
    x_to: i32, // inclusive
}

#[derive(Debug)]
struct Symbol {
    x: i32,
    y: i32,
}

pub fn sum_part_nums(input: String) -> i32 {
    let numbers = find_numbers(&input);
    let symbols = find_symbols(&input);

    let mut sum = 0;
    for number in numbers {
        match symbols.iter().find(|sym| {
            sym.x >= number.x_from - 1
                && sym.x <= number.x_to + 1
                && sym.y >= number.y - 1
                && sym.y <= number.y + 1
        }) {
            Some(_) => sum += number.value,
            _ => (),
        };
    }
    sum
}

pub fn sum_gear_ratios(input: String) -> i32 {
    let numbers = find_numbers(&input);
    let gears = find_gears(&input);

    let mut sum = 0;
    for gear in gears {
        let adjacent_numbers: Vec<&Number> = numbers
            .iter()
            .filter(|num| {
                gear.x >= num.x_from - 1
                    && gear.x <= num.x_to + 1
                    && gear.y >= num.y - 1
                    && gear.y <= num.y + 1
            })
            .collect();

        // check if adjacent_numbers has exactly 2 elems
        if adjacent_numbers.len() == 2 {
            let g1 = adjacent_numbers.get(0).map(|n| n.value).unwrap();
            let g2 = adjacent_numbers.get(1).map(|n| n.value).unwrap();
            sum += g1 * g2;
        }
    }
    sum
}

fn find_numbers(input: &String) -> Vec<Number> {
    input
        .lines()
        .enumerate()
        .flat_map(|(line_no, line)| numbers_in_line(line_no.try_into().unwrap(), line))
        .collect()
}

fn numbers_in_line(line_no: i32, line: &str) -> Vec<Number> {
    let mut nums_in_line: Vec<Number> = Vec::new();
    let mut maybe_x_from: Option<usize> = None;
    let mut x_to = 0;
    for (x, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if maybe_x_from.is_none() {
                maybe_x_from = Some(x);
            }
            x_to = x;
        } else if maybe_x_from.is_some() {
            let x_from = maybe_x_from.unwrap();
            // end of a number
            nums_in_line.push(Number {
                value: line[x_from..=x_to].parse::<i32>().unwrap(),
                y: line_no,
                x_from: x_from.try_into().unwrap(),
                x_to: x_to.try_into().unwrap(),
            });
            maybe_x_from = None;
        }
    }

    match maybe_x_from {
        Some(x_from) => nums_in_line.push(Number {
            value: line[x_from..].parse::<i32>().unwrap(),
            y: line_no,
            x_from: x_from.try_into().unwrap(),
            x_to: line.len().try_into().unwrap(),
        }),
        _ => (),
    }
    nums_in_line
}

fn find_symbols(input: &String) -> Vec<Symbol> {
    let mut syms = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' && !c.is_digit(10) {
                syms.push(Symbol {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                })
            }
        })
    });
    syms
}

fn find_gears(input: &String) -> Vec<Symbol> {
    let mut gears: Vec<Symbol> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '*' {
                gears.push(Symbol {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
        });
    });
    gears
}

#[cfg(test)]
mod tests {
    use crate::day_3::{sum_gear_ratios, sum_part_nums};

    #[test]
    fn example_1() {
        let input = String::from(
            "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..",
        );
        let expected = 4361;
        let actual = sum_part_nums(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_2() {
        let input = String::from(
            "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..",
        );
        let expected = 467835;
        let actual = sum_gear_ratios(input);
        assert_eq!(actual, expected)
    }
}

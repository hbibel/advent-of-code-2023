use std::iter;

#[derive(Debug)]
enum Orientation {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    ANY,
}

#[derive(Debug)]
struct Position {
    p_x: usize,
    p_y: usize,
    orientation: Orientation,
    p_char: char,
}

pub fn farthest_distance(input: &String) -> u32 {
    println!("{}", to_readable(input));

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect();
    let mut next = starting_pos(&grid);
    let mut steps = 1;
    while next.p_char != 'S' {
        next = match step(&next.orientation, &grid, &next.p_x, &next.p_y) {
            Some(p) => p,
            None => {
                panic!("Step from {:?}", next);
            }
        };
        steps += 1;
    }
    steps / 2
}

fn starting_pos(grid: &Vec<Vec<char>>) -> Position {
    let (ax, ay) = grid
        .iter()
        .enumerate()
        .find_map(
            |(y, cs)| match cs.iter().enumerate().find(|(_, c)| c == &&'S').map(|t| t.0) {
                None => None,
                Some(x) => Some((x, y)),
            },
        )
        .unwrap();

    vec![
        Orientation::LEFT,
        Orientation::RIGHT,
        Orientation::UP,
        Orientation::DOWN,
    ]
    .iter()
    .filter_map(|ori| step(ori, grid, &ax, &ay))
    .nth(0)
    .unwrap()
}

fn step(
    from_orientation: &Orientation,
    grid: &Vec<Vec<char>>,
    p_x: &usize,
    p_y: &usize,
) -> Option<Position> {
    match (p_x, p_y, from_orientation) {
        (_, 0, Orientation::UP) => None,
        (_, _, Orientation::UP) => {
            let orientation = match grid[*p_y - 1][*p_x] {
                '|' => Some(Orientation::UP),
                'F' => Some(Orientation::RIGHT),
                '7' => Some(Orientation::LEFT),
                'S' => Some(Orientation::ANY),
                _ => None,
            }?;
            Some(Position {
                p_x: *p_x,
                p_y: p_y - 1,
                orientation,
                p_char: grid[*p_y - 1][*p_x],
            })
        }
        (_, _, Orientation::DOWN) if *p_y >= grid.len() - 1 => None,
        (_, _, Orientation::DOWN) => {
            let orientation = match grid[*p_y + 1][*p_x] {
                '|' => Some(Orientation::DOWN),
                'J' => Some(Orientation::LEFT),
                'L' => Some(Orientation::RIGHT),
                'S' => Some(Orientation::ANY),
                _ => None,
            }?;
            Some(Position {
                p_x: *p_x,
                p_y: p_y + 1,
                orientation,
                p_char: grid[*p_y + 1][*p_x],
            })
        }
        (0, _, Orientation::LEFT) => None,
        (_, _, Orientation::LEFT) => {
            let orientation = match grid[*p_y][*p_x - 1] {
                '-' => Some(Orientation::LEFT),
                'L' => Some(Orientation::UP),
                'F' => Some(Orientation::DOWN),
                'S' => Some(Orientation::ANY),
                _ => None,
            }?;
            Some(Position {
                p_x: p_x - 1,
                p_y: *p_y,
                orientation,
                p_char: grid[*p_y][*p_x - 1],
            })
        }
        (_, _, Orientation::RIGHT) if *p_x >= grid[0].len() - 1 => None,
        (_, _, Orientation::RIGHT) => {
            let orientation = match grid[*p_y][*p_x + 1] {
                '-' => Some(Orientation::RIGHT),
                '7' => Some(Orientation::DOWN),
                'J' => Some(Orientation::UP),
                'S' => Some(Orientation::ANY),
                _ => None,
            }?;
            Some(Position {
                p_x: p_x + 1,
                p_y: *p_y,
                orientation,
                p_char: grid[*p_y][*p_x + 1],
            })
        }
        (_, _, Orientation::ANY) => None,
    }
}

fn to_readable(input: &String) -> String {
    return input
        .chars()
        .map(|c| match c {
            '.' => ' ',
            '-' => '─',
            '|' => '│',
            'F' => '┌',
            '7' => '┐',
            'L' => '└',
            'J' => '┘',
            'S' => '╳',
            other => other,
        })
        .collect();
}

pub fn enclosed_tiles(input: &String) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect();
    let grid = only_loop(grid);

    let new_input = grid
        .iter()
        .map(|l| l.iter().collect::<String>() + "\n")
        .collect::<String>();
    println!("{}", to_readable(&new_input));

    grid.iter().fold(0, |grid_acc, line| {
        let mut enclosed = false;
        let mut in_turn = None;
        line.iter()
            .fold(0, |line_acc, c| match (enclosed, in_turn, *c) {
                (false, _, '.') => line_acc,
                (true, _, '.') => line_acc + 1,
                (_, _, '-') => line_acc,
                (_, _, '|') => {
                    enclosed = !enclosed;
                    line_acc
                }
                (_, _, 'F') => {
                    in_turn = Some('F');
                    line_acc
                }
                (_, Some('F'), '7') => line_acc,
                (_, Some('L'), '7') => {
                    enclosed = !enclosed;
                    in_turn = None;
                    line_acc
                }
                (_, _, 'L') => {
                    in_turn = Some('L');
                    line_acc
                }
                (_, Some('L'), 'J') => line_acc,
                (_, Some('F'), 'J') => {
                    enclosed = !enclosed;
                    in_turn = None;
                    line_acc
                }
                _ => unreachable!("!"),
            })
            + grid_acc
    })
}

fn only_loop(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let start = Position {
        p_x: 43,
        p_y: 92,
        orientation: Orientation::RIGHT,
        p_char: '-',
    };
    let mut new_grid: Vec<Vec<char>> =
        iter::repeat(iter::repeat('.').take(grid[0].len()).collect())
            .take(grid.len())
            .collect();
    new_grid[92][43] = '-';
    let mut next = step(&start.orientation, &grid, &start.p_x, &start.p_y).unwrap();
    while next.p_char != 'S' {
        new_grid[next.p_y][next.p_x] = next.p_char;
        next = step(&next.orientation, &grid, &next.p_x, &next.p_y).unwrap();
    }
    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = String::from(
            "-L|F7\n\
             7S-7|\n\
             L|7||\n\
             -L-J|\n\
             L|-JF",
        );
        let expected = 4;
        let actual = farthest_distance(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let input = String::from(
            "7-F7-\n\
             .FJ|7\n\
             SJLL7\n\
             |F--J\n\
             LJ.LJ",
        );
        let expected = 8;
        let actual = farthest_distance(&input);
        assert_eq!(actual, expected);
    }
}

pub fn sum_shortest_paths(input: &String) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let empty_rows: Vec<usize> = (0..grid.len())
        .filter(|r| grid[*r].iter().all(|c| *c == '.'))
        .collect();
    let empty_cols: Vec<usize> = (0..grid[0].len())
        .filter(|c| grid.iter().map(|r| r[*c]).all(|c| c == '.'))
        .collect();

    let galaxy_coords: Vec<(usize, usize)> = (0..grid.len())
        .flat_map(|r| (0..grid[0].len()).map(move |c| (r, c)))
        .filter(|(r, c)| grid[*r][*c] == '#')
        .collect();

    let mut sum = 0;
    for i in 0..galaxy_coords.len() {
        for j in i + 1..galaxy_coords.len() {
            let (r1, c1) = galaxy_coords[i];
            let (r2, c2) = galaxy_coords[j];
            sum += (r1 as i64 - r2 as i64).abs();
            sum += (c1 as i64 - c2 as i64).abs();
            sum += empty_rows
                .iter()
                .filter(|er| er > &&r1 && er < &&r2 || er > &&r2 && er < &&r1)
                .count() as i64;
            sum += empty_cols
                .iter()
                .filter(|ec| ec > &&c1 && ec < &&c2 || ec > &&c2 && ec < &&c1)
                .count() as i64;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = String::from(
            "...#......\n\
             .......#..\n\
             #.........\n\
             ..........\n\
             ......#...\n\
             .#........\n\
             .........#\n\
             ..........\n\
             .......#..\n\
             #...#.....",
        );
        let expected = 374;
        let actual = sum_shortest_paths(&input);
        assert_eq!(actual, expected);
    }
}

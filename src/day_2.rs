use regex::Regex;

struct Game {
    id: i32,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
}

pub fn possible_game_id_sum(input: String) -> i32 {
    let games = input.lines().map(|l| Game::from(String::from(l)));
    games
        .filter_map(|g| {
            let is_possible = g.max_red <= 12 && g.max_green <= 13 && g.max_blue <= 14;
            if is_possible {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

impl Game {
    fn from(line: String) -> Game {
        let max_red = color_count("red", &line);
        let max_green = color_count("green", &line);
        let max_blue = color_count("blue", &line);

        let id = line
            .chars()
            .skip(5)
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        Game {
            id,
            max_red,
            max_green,
            max_blue,
        }
    }
}

fn color_count(color: &str, line: &String) -> i32 {
    let re = Regex::new((String::from(r"(\d+) ") + color).as_str()).unwrap();
    re.captures_iter(line.as_str())
        .map(|c| {
            let (_, [count]) = c.extract();
            count.parse::<i32>().unwrap()
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::day_2::possible_game_id_sum;

    #[test]
    fn example() {
        let input = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        let expected = 88;
        let actual = possible_game_id_sum(input);
        assert_eq!(actual, expected);
    }
}

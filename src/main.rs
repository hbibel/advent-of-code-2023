use std::{env::args, fs};

mod day_1;
mod day_2;

fn main() {
    let day = args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap()
        .clone()
        .parse::<i32>()
        .unwrap();
    let filename = args().collect::<Vec<String>>().get(2).unwrap().clone();
    let content = fs::read_to_string(filename).unwrap();

    if day == 1 {
        println!("{}", day_1::compute_calibration_value(content.clone()));
        println!("{}", day_1::compute_calibration_value_2(content.clone()));
    } else if day == 2 {
        println!("{}", day_2::possible_game_id_sum(content.clone()));
    }
}

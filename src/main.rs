use std::{env::args, fs};

mod day_1;
mod day_2;
mod day_3;

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
        println!("{}", day_2::game_power_sum(content.clone()));
    } else if day == 3 {
        println!("{}", day_3::sum_part_nums(content.clone()));
        println!("{}", day_3::sum_gear_ratios(content.clone()));
    }
}

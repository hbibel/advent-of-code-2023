use std::{env::args, fs};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

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
    } else if day == 4 {
        println!("{}", day_4::pile_worth(content.clone()));
        println!("{}", day_4::num_cards(content.clone()));
    } else if day == 5 {
        println!("{}", day_5::lowest_location(content.clone()));
        println!("{}", day_5::lowest_location_ranges(content.clone()));
    } else if day == 6 {
        println!("{}", day_6::ways_to_win(content.clone()));
        println!("{}", day_6::ways_to_win_2(content.clone()));
    } else if day == 7 {
        println!("{}", day_7::total_winnings(content.clone()));
    }
}

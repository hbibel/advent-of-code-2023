use std::{env::args, fs};

mod day_1;

fn main() {
    let filename = args().collect::<Vec<String>>().get(1).unwrap().clone();
    let content = fs::read_to_string(filename).unwrap();

    println!("{}", day_1::compute_calibration_value(content.clone()));
    println!("{}", day_1::compute_calibration_value_2(content));
}

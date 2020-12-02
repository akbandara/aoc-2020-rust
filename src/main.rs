extern crate csv;

use std::env;
use aoc_2020_rust::aoc_2020_solutions::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data_dir = &args[1];
    let aoc_day = &args[2].parse().unwrap_or(0);
    let aoc_day_part = &args[3].parse::<i32>().unwrap_or(0);
    

    match aoc_day {
        1 => aoc_day1::solve(data_dir.to_string(), *aoc_day_part).ok(),
        _ => aoc_no_solution::solve(data_dir.to_string(), *aoc_day_part).ok(),
    };
    
}
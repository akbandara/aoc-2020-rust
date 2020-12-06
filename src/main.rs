use std::env;
use aoc_2020_rust::aoc_2020_solutions::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data_dir = &args[1];
    let aoc_day = &args[2].parse::<usize>().unwrap_or(0);
    let aoc_day_part = &args[3].parse::<usize>().unwrap_or(0);
    

    match aoc_day {
        1 => aoc_day1::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
        2 => aoc_day2::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
        3 => aoc_day3::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
        4 => aoc_day4::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
        5 => aoc_day5::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
        6 => aoc_day6::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
        _ => aoc_no_solution::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
    };
    
}
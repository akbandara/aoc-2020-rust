use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


pub mod aoc_day1;
pub mod aoc_day2;
pub mod aoc_day3;
pub mod aoc_day4;
pub mod aoc_day5;
pub mod aoc_no_solution;

pub fn load_data(filename:String, day:usize, part:usize, data:&mut Vec<String>) -> io::Result<()> {
    println!("Reading input file {} to solve Day {} Part {}", filename, day, part);

    let file = File::open(filename)?;
    let file = BufReader::new(file);

    // read file into vector, password_entries
    let mut _data: Vec<String> = file
        .lines()
        .map(|line| line.unwrap())
        .collect();
    
    data.append(&mut _data);
    Ok(())
}

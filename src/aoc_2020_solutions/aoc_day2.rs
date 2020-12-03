use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn solve(data_dir: String, part : i32) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day 2 / Part {}", part);
    let filename = data_dir + "day2-input.txt";
    println!("Reading input file {} to solve Part {}", filename, part);

    let file = File::open(filename)?;
    let file = BufReader::new(file);

    // read file into vector, password_entries
    let password_entries: Vec<String> = file
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut valid_password_count = 0;
    for password_entry in password_entries {
        //println!("Password entry = {}", password_entry.to_string());
        let pw_fields : Vec<&str> = password_entry.split(" ").collect();
        let policy_bounds : Vec<&str>  = pw_fields[0].split("-").collect();
        let policy_1 = policy_bounds[0].parse::<usize>().unwrap_or(0);
        let policy_2 = policy_bounds[1].parse::<usize>().unwrap_or(0);

        let policy_char = pw_fields[1].chars().nth(0).unwrap();
        let password = pw_fields[2];

        if part == 1 {
            let password_policy_chars = password.matches(policy_char).count();
            if (policy_1 <= password_policy_chars) & (password_policy_chars <= policy_2) {
                valid_password_count = valid_password_count + 1
            }
        }

        if part == 2 {
            if (password.chars().nth(policy_1-1).unwrap() == policy_char) || (password.chars().nth(policy_2-1).unwrap() == policy_char) {
                if !((password.chars().nth(policy_1-1).unwrap() == policy_char) && (password.chars().nth(policy_2-1).unwrap() == policy_char)) {
                    valid_password_count = valid_password_count + 1
                }
            }    
        }

    }

    println!("Number of valid passwords = {}", valid_password_count);

    Ok(())
}
use crate::aoc_2020_solutions::load_data;
use std::io;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day {} / Part {}", day, part);

    // read file into vector, password_entries
    let mut password_entries: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut password_entries)?;

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
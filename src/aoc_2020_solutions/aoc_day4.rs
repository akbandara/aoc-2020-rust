extern crate regex;

use crate::aoc_2020_solutions::load_data;
use std::io;
use regex::Regex;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day {} / Part {}", day, part);

    let mut valid_password_count = 0;
    let mut passport_data : Vec<String> = Vec::new();
    load_data(filename, day, part, &mut passport_data)?;

    let valid_field_ids = ["ecl:", "pid:", "eyr:", "hcl:", "byr:", "iyr:", "hgt:"];

    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();

    let passports = passport_data.split(|line| line.len() == 0);
    let mut passport_num = 0;
    for passport in passports {
        passport_num = passport_num + 1;
        let mut found_fields = 0;
        let mut all_fields_correct = true;

        'check_passport: for data_line in passport {
            for valid_field_id in valid_field_ids.iter() {
                if data_line.contains(valid_field_id) {
                    found_fields = found_fields + 1;

                    if part == 2 {
                        //Check if field matches expected value ranges
                        let field_values: Vec<&str>  = data_line.split(" ").collect();
                        let value_suffix = field_values.iter().find(|field| field.starts_with(valid_field_id)).unwrap();
                        let field_value = value_suffix.strip_prefix(valid_field_id).unwrap();

                        // Check ecl: field
                        if *valid_field_id == "ecl:" {
                            all_fields_correct = all_fields_correct && "amb blu brn gry grn hzl oth".contains(field_value);
                        }
                        // Check byr: field
                        if *valid_field_id == "byr:" {
                            let byr = field_value.parse::<usize>().unwrap_or(0);
                            all_fields_correct = all_fields_correct && (1920 <= byr && byr <= 2002);
                        }

                        // Check iyr: field
                        if *valid_field_id == "iyr:" {
                            let iyr = field_value.parse::<usize>().unwrap_or(0);
                            all_fields_correct = all_fields_correct && (2010 <= iyr && iyr <= 2020);
                        }
                        
                        // Check eyr: field
                        if *valid_field_id == "eyr:" {
                            let eyr = field_value.parse::<usize>().unwrap_or(0);
                            all_fields_correct = all_fields_correct && (2020 <= eyr && eyr <= 2030);
                        }

                        // Check hgt: field
                        if *valid_field_id == "hgt:" {
                            let (value, suffix) = field_value.split_at(field_value.len()-2);
                            let hgt = value.parse::<usize>().unwrap_or(0);
                            if suffix == "cm" {
                                all_fields_correct = all_fields_correct && (150 <= hgt && hgt <= 193);
                            } else if suffix == "in" {
                                all_fields_correct = all_fields_correct && (59 <= hgt && hgt <= 76);
                            } else {
                                all_fields_correct = all_fields_correct && false;
                            }
                        }

                        // Check hcl: field
                        if *valid_field_id == "hcl:" {
                            all_fields_correct = all_fields_correct && hcl_re.is_match(field_value);

                        }

                        // Check hcl: field
                        if *valid_field_id == "pid:" {
                            all_fields_correct = all_fields_correct && pid_re.is_match(field_value);

                        }

                        if !all_fields_correct {
                            //println!("Passport {}, {}, {}, Invalid", passport_num, valid_field_id, field_value);
                            break 'check_passport;
                        } else {
                            //println!("Passport {}, {}, {}, Valid", passport_num, valid_field_id, field_value);
                        }
                    }
                    
                }
            }
        }

        //If all the fields are found, this is a valid passport
        if found_fields==valid_field_ids.len() && all_fields_correct {
            println!("Passport {}, Valid", passport_num);
            valid_password_count = valid_password_count + 1;
        }

    }
    println!("Count of valid passports = {}", valid_password_count);

    Ok(())
}
use crate::aoc_2020_solutions::load_data;
use std::io;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day {} / Part {}", day, part);

    // read file into vector, boarding_cards
    let mut customs_forms: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut customs_forms)?;

    let group_customs_forms = customs_forms.split(|line| line.len() == 0);

    let mut group_response_count = 0;

    for group_customs_form in group_customs_forms {
        
        if part == 1 {
            let mut group_customs_decl = "".to_owned();
            for line in group_customs_form {
                group_customs_decl.push_str(&line);
            }
            let mut group_customs_response : Vec<char> = group_customs_decl.chars().collect();
            group_customs_response.sort();
            group_customs_response.dedup();
            group_response_count = group_response_count + group_customs_response.len();
        }

        if part == 2 {
            let mut response_count = [0;26];
            // process all the lines and count number of char responses
            for line in group_customs_form {
                for c in line.chars() {
                    let char_index = ((c as u32)-('a' as u32)) as usize;
                    response_count[char_index] = response_count[char_index] + 1;
                }   
            }

            // valid response if value of response_count = number of lines
            for response in response_count.iter() {

                if *response == group_customs_form.len() {
                    group_response_count = group_response_count + 1;
                }
            }

        }
    }

    println!("Group Customs Counts: {}", group_response_count);

    Ok(())
}
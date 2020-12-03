use crate::aoc_2020_solutions::load_data;
use std::io;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day {} / Part {}", day, part);

    let mut data: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut data)?;

    let claims: Vec<i64> = data.iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    if part == 1 {
        //solving part 1
        for claim_value1 in &claims {
            for claim_value2 in &claims {
                if (claim_value1+claim_value2) == 2020 {
                    println!("Claim 1 ({}) + Claim 2 ({}) = 2020; Claim 1 x Claim 2 = {}",claim_value1, claim_value2, claim_value1*claim_value2);
                }
            }
        }

    }

    if part == 2 {
        // solving part 2
        for claim_value1 in &claims {
            for claim_value2 in &claims {
                for claim_value3 in &claims {
                    if ((claim_value1+claim_value2+claim_value3) == 2020) & ((claim_value1*claim_value2*claim_value3) > 0) {
                        println!("Claim 1 ({}) + Claim 2 ({}) + Claim 3 ({}) = 2020; Claim 1 x Claim 2 x Claim 3 = {}",claim_value1, claim_value2, claim_value3, claim_value1*claim_value2*claim_value3);
                    }
                }
            }
        }
    }
    

    Ok(())

}
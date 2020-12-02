use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn solve(data_dir: String, part : i32) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day 1 / Part {}", part);
    let filename = data_dir + "day1-input.txt";
    println!("Reading input file {} to solve Part {}", filename, part);

    let file = File::open(filename)?;
    let file = BufReader::new(file);

    // read file into vector, claims
    let claims: Vec<i64> = file
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
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
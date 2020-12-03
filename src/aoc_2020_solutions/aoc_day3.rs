use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn get_tree_count(hillside: &Vec<String>, down_slope: usize, cross_slope: usize) -> usize {
    let mut tree_count = 0;
    let mut h_pos = 0;
    let local_hillside = hillside.iter().skip(down_slope);
    let hill_width = hillside[0].len();

    for hill_contour in local_hillside.step_by(down_slope) {
        h_pos = (h_pos+cross_slope)%hill_width;
        let hill_pos = hill_contour.chars().nth(h_pos).unwrap();
        if hill_pos == '#' {
            tree_count += 1;
        }
    }   
    tree_count
}

pub fn solve(data_dir: String, part : i32) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day 3 / Part {}", part);
    let filename = data_dir + "day3-input.txt";
    //let filename = data_dir + "day3-test-input.txt";
    println!("Reading input file {} to solve Part {}", filename, part);

    let file = File::open(filename)?;
    let file = BufReader::new(file);

    // read file into vector, password_entries
    let hillside: Vec<String> = file
        .lines()
        .map(|line| line.unwrap())
        .collect();

    if part == 1{
        let slopes = vec![(1,3)];
        let mut tree_count = 0;
        for (down_slope, cross_slope) in slopes{
            tree_count = tree_count + get_tree_count(&hillside, down_slope, cross_slope);
        }
        println!("Number of trees = {}", tree_count);
    }

    if part ==2 {
        let slopes = vec![(1,1), (1,3), (1,5), (1,7), (2,1)];
        let mut tree_product = 1;

        for (down_slope, cross_slope) in slopes{
            let tree_count = get_tree_count(&hillside, down_slope, cross_slope);
            tree_product = tree_product * tree_count;
        }
        println!("Product of trees = {}", tree_product);
    }



    Ok(())
}
use crate::aoc_2020_solutions::load_data;
use std::io;

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

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day {} / Part {}", day, part);

    let mut hillside: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut hillside)?;

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
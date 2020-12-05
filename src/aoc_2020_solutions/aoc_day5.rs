use crate::aoc_2020_solutions::load_data;
use std::io;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2020 - Day {} / Part {}", day, part);

    // read file into vector, boarding_cards
    let mut boarding_cards: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut boarding_cards)?;

    let mut highest_seat_id = 0;
    let mut seat_ids: Vec<i32> = Vec::new();

    for boarding_card in boarding_cards {

        let (row_section, col_section) = boarding_card.split_at(7);
        let mut row_number_ub : i32 = 127;
        let mut row_number_lb : i32  = 0;
        let mut col_number_ub : i32  = 7;
        let mut col_number_lb : i32  = 0;

        for (_, row_code) in row_section.chars().enumerate() {
            let row_number_mid_point = row_number_lb+(row_number_ub-row_number_lb)/2;
            match row_code {
                'B' => row_number_lb = row_number_mid_point+1,
                'F' => row_number_ub = row_number_mid_point,
                _ => continue,
            };
        }

        for (_, col_code) in col_section.chars().enumerate() {
            let col_number_mid_point = col_number_lb+(col_number_ub-col_number_lb)/2;
            match col_code {
                'R' => col_number_lb = col_number_mid_point+1,
                'L' => col_number_ub = col_number_mid_point,
                _ => continue,
            };
        }
        
        let seat_id = (8*row_number_lb) + col_number_lb;
        seat_ids.push(seat_id);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    if part == 1 {
        println!{"Highest Seat ID = {}", highest_seat_id};
    }

    if part == 2 {
        // sort the seat_ids
        seat_ids.sort();

        // iterate through and find gap in seat ids
        let mut prev_seat_id = seat_ids[0];
        seat_ids.remove(0);
        for curr_seat_id in seat_ids {

            if curr_seat_id-prev_seat_id == 1 {
                prev_seat_id = curr_seat_id;
            } else {
                println!{"My Seat ID is {}", prev_seat_id+1};
                break;
            }

        }
    }

    Ok(())
}
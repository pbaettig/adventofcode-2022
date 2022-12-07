extern crate day_2;
use day_2::duel::{read_duelresults_from_file};
use std::process::exit;
use std::env;

fn main() {
    let fname = env::args().nth(1).unwrap_or("input.txt".to_string());
  
    let Ok(duel_results) = read_duelresults_from_file(&fname) else {
        println!("could not open {fname}");
        exit(1);
    };

    let mut my_score = 0;
    for dr in duel_results {
        let dr = dr.unwrap();
        let dr = dr.points() as u32;
        my_score += dr;
    }
    println!("The result is: {}", my_score);
}
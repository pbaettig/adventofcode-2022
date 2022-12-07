extern crate day_2;
use day_2::duel::{read_duels_from_file};
use std::process::exit;
use std::env;

fn main() {
    // let cmd = env::args().nth(0).unwrap();
    // let Some(fname) = env::args().nth(1) else {
    //     println!("USAGE: {cmd} input-file");
    //     exit(2)
    // };

    let fname = env::args().nth(1).unwrap_or("input.txt".to_string());
  
    let Ok(duels) = read_duels_from_file(&fname) else {
        println!("could not open {fname}");
        exit(1);
    };
   
    let mut my_score: u32 = 0;
    for d in duels {
        let d = d.unwrap();
        let dr = d.result().points() as u32;
        my_score += dr;
    }

    println!("The result is: {}", my_score);
}

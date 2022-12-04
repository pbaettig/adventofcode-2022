use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    println!("{}", process_input())
}


fn process_input() -> u32 {
    let file = File::open("input.txt").unwrap();
    
    let iter = io::BufReader::new(file).lines().into_iter();
    let iter2 = iter.map(|lr| ->Option<u32> {
        let l = lr.unwrap();
        
        match l.parse() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    });

    let mut elf_idx = 1;
    let mut elf_with_most_calories = 0;
    let mut current_sum: u32 = 0;
    let mut maximum_sum:u32 = 0;
    for n in iter2 {
        match n {
            Some(v) => current_sum += v,
            None => {
                if current_sum > maximum_sum {
                    maximum_sum = current_sum;
                    elf_with_most_calories = elf_idx;
                }

                // println!("Elf {} carries {} calories", elf_idx, current_sum);
                elf_idx += 1;
                current_sum = 0;
            }
        }
    }
    println!("Elf {} carries the most calories ({})", elf_with_most_calories, maximum_sum);
    maximum_sum
}
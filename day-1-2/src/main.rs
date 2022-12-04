use std::fs::File;
use std::io::{self, BufRead};
use std::net::UdpSocket;

fn main() {
    
    let mut arr:[u32;3] = [3,2,1];




    dbg!(arr);
    
    arr[0..].rotate_right(1);
    arr[0] = 10;
    dbg!(arr);

    println!("===========");

    arr[1..].rotate_right(1);
    arr[1] = 7;
    dbg!(arr);

    println!("===========");

    arr[2..].rotate_right(1);
    arr[2] = 5;
    dbg!(arr);


    update_top_3(12, &mut arr);
    dbg!(arr);

    update_top_3(11, &mut arr);
    dbg!(arr);

    update_top_3(21, &mut arr);
    dbg!(arr);

    println!("The answer is: {}", process_input());


}

// 0: 1
// 1: 1
// 2: 1

// v = 5
// 0: 5 > 1 => 5; return
// 0: 5
// 1: 1
// 2: 1

// v = 4
// 0: 4 < 5
// 1: 4 > 1 => 4; return

fn update_top_3(v:u32, arr:&mut[u32;3]) {
    let mut max_idx: Option<usize> = None;
    for i in 0..3 {
        if v > arr[i] {
            max_idx = Some(i);
            break;
        }
    }

    match max_idx {
        Some(i) => {
            println!("value {} is bigger than value at index {}", v, i);
            arr[i..].rotate_right(1);
            arr[i] = v;
            dbg!(arr);
        },
        None => {return;}
    }

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

    let mut calories_top_3: [u32; 3] = [0;3];
    let mut current_elf_calories: u32 = 0;
   
    for n in iter2 {
        match n {
            Some(v) => current_elf_calories += v,
            None => {
                println!("{}", current_elf_calories);

                update_top_3(current_elf_calories, &mut calories_top_3);
                current_elf_calories = 0
            }
        }
    }
    println!("");
    println!("");
    let mut sum = 0;
    for cals in calories_top_3 {
        println!("{}", cals);
        sum += cals;
    }

    sum

}
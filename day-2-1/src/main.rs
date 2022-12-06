mod duel;
mod shapes;
use duel::{assert, read_duels_from_file};

fn main() {
    assert();

    let duels = read_duels_from_file("input2.txt").unwrap();
    let mut my_score: u32 = 0;
    for d in duels {
        let d = d.unwrap();
        let dr = d.result().points() as u32;
        my_score += dr;
    }

    println!("The result is: {}", my_score);
}

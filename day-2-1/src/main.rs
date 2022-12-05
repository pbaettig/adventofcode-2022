mod shapes;

use shapes::{Shape, DuelResult};
use std::error::Error;
use std::slice::Windows;
use std::str::FromStr;





fn main() {
    shapes::assert();
    let r = Shape::Paper.duel(Shape::Rock);
    println!("{}", r.points())

}

fn duel(a: Shape, b:Shape) {
    match a.duel(b) {
        DuelResult::Win(v) => println!("{a} wins against {b}, +{v} points"),
        DuelResult::Lose(v) => println!("{a} loses against {b}, +{v} points"),
        DuelResult::Draw(v) => println!("{a} draws against {b}, +{v} points"),
    }
}
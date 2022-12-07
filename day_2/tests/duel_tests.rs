use std::str::FromStr;

use day_2::duel::{Duel, DuelResult};
use day_2::shapes::Shape;

#[test]
pub fn test_duel_results() {
    assert_eq!(Duel::new(Shape::Rock, Shape::Paper).result(), DuelResult::Lose(1));
    assert_eq!(Duel::new(Shape::Rock, Shape::Scissor).result(), DuelResult::Win(7));
    assert_eq!(Duel::new(Shape::Rock, Shape::Rock).result(), DuelResult::Draw(4));

    assert_eq!(Duel::new(Shape::Paper, Shape::Rock).result(), DuelResult::Win(8));
    assert_eq!(Duel::new(Shape::Paper, Shape::Scissor).result(), DuelResult::Lose(2));
    assert_eq!(Duel::new(Shape::Paper, Shape::Paper).result(), DuelResult::Draw(5));

    assert_eq!(Duel::new(Shape::Scissor, Shape::Rock).result(), DuelResult::Lose(3));
    assert_eq!(Duel::new(Shape::Scissor, Shape::Paper).result(), DuelResult::Win(9));
    assert_eq!(Duel::new(Shape::Scissor, Shape::Scissor).result(), DuelResult::Draw(6));
}

#[test]
pub fn test_shape_from_str() {
    assert!(matches!(Shape::from_str("A"), Ok(Shape::Rock)));
    assert!(matches!(Shape::from_str("b"), Ok(Shape::Paper)));
    assert!(matches!(Shape::from_str("C"), Ok(Shape::Scissor)));
    assert!(matches!(Shape::from_str("x"), Ok(Shape::Rock)));
    assert!(matches!(Shape::from_str("Y"), Ok(Shape::Paper)));
    assert!(matches!(Shape::from_str("z"), Ok(Shape::Scissor)));
    assert!(matches!(Shape::from_str("v"), Err(_)));
    assert!(matches!(Shape::from_str(""), Err(_)));
}

#[test]
pub fn test_duelresult_from_str() {
    assert!(matches!(DuelResult::from_str("C Z"), Ok(DuelResult::Win(7))));
    assert!(matches!(DuelResult::from_str("A Z"), Ok(DuelResult::Win(8))));
    assert!(matches!(DuelResult::from_str("B X"), Ok(DuelResult::Lose(1))));
    assert!(matches!(DuelResult::from_str("C Y"), Ok(DuelResult::Draw(6))));
    assert!(matches!(DuelResult::from_str(" Y"), Err(_)));
    assert!(matches!(DuelResult::from_str("y M"), Err(_)));
    assert!(matches!(DuelResult::from_str("a"), Err(_)));
}

#[test]
pub fn test_duel_from_str() {
    let d = Duel::from_str("A X").unwrap();
    assert_eq!((d.me, d.opponent), (Shape::Rock, Shape::Rock));

    let d = Duel::from_str("c y").unwrap();
    assert_eq!((d.me, d.opponent), (Shape::Paper, Shape::Scissor));

    let d = Duel::from_str("B y").unwrap();
    assert_eq!((d.me, d.opponent), (Shape::Paper, Shape::Paper));
    assert_eq!(d.result().points(), 5);


    assert!(matches!(Duel::from_str("B "), Err(_)));
}
use std::fmt;


#[derive(Debug)]
pub struct UnrecognizedShapeError;
impl fmt::Display for UnrecognizedShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "shape is not recognized")
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Debug)]
pub enum DuelResult {
    Win(u8),
    Lose(u8),
    Draw(u8),
}
const _DUEL_WIN: u8 = 6;
const _DUEL_DRAW: u8 = 3;
const _DUEL_LOSE: u8 = 0;

impl DuelResult {
    pub fn points(&self) -> u8 {
        match self {
            Self::Draw(p) | Self::Lose(p) | Self::Win(p) => *p
        }
    }
}


pub fn assert() {
    match Shape::Rock.duel(Shape::Paper) {
        DuelResult::Lose(1) => println!("Rock loses against paper and grants 1 point"),
        _ => panic!("Assertion failed")
    }

    match Shape::Paper.duel(Shape::Rock) {
        DuelResult::Win(8) => println!("Rock wins against paper and grants 8 points"),
        _ => panic!("Assertion failed")
    }
    match Shape::Scissor.duel(Shape::Rock) {
        DuelResult::Lose(3) => println!("Scissors lose against rock and grants 3 points"),
        _ => panic!("Assertion failed")
    }
    match Shape::Scissor.duel(Shape::Paper) {
        DuelResult::Win(9) => println!("Scissors win against paper and grants 9 points"),
        _ => panic!("Assertion failed")
    }
}


impl Shape {
    pub fn duel(&self, other: Self) -> DuelResult {
        if *self == other {
            return DuelResult::Draw(_DUEL_DRAW)
        }
    
        match (self, other) {
            (Self::Rock, Self::Paper) | (Self::Paper, Self::Scissor) | (Self::Scissor, Self::Rock) =>  {
                DuelResult::Lose(_DUEL_LOSE + *self as u8)
            },
            (Self::Rock, Self::Scissor) | (Self::Paper, Self::Rock) | (Self::Scissor, Self::Paper) =>  {
                DuelResult::Win(_DUEL_WIN + *self as u8)
            },
            _ => panic!("shouldn't ever happen")
        }
    }
}

impl std::str::FromStr for Shape {
    type Err = UnrecognizedShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => Err(UnrecognizedShapeError)
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Rock => write!(f, "Rock"),
            Shape::Paper => write!(f, "Paper"),
            Shape::Scissor => write!(f, "Scissor"),
        }
    }
}

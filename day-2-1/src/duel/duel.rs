use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::fmt;
use std::str::FromStr;

use super::super::shapes::Shape;

#[derive(Debug)]
pub struct DuelParseError;
impl fmt::Display for DuelParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse Duel")
    }
}

pub struct Duel {
    me: Shape,
    opponent: Shape,
}

impl Duel {
    pub fn new(me: Shape, opponent: Shape) -> Self {
        Duel { me, opponent }
    }

    pub fn result(&self) -> DuelResult {
        if self.me == self.opponent {
            return DuelResult::Draw(_DUEL_DRAW + self.me as u8);
        }

        match (self.me, self.opponent) {
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissor)
            | (Shape::Scissor, Shape::Rock) => DuelResult::Lose(_DUEL_LOSE + self.me as u8),
            (Shape::Rock, Shape::Scissor)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissor, Shape::Paper) => DuelResult::Win(_DUEL_WIN + self.me as u8),
            _ => panic!("shouldn't ever happen"),
        }
    }
}

impl fmt::Display for Duel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "me:{} vs opponent:{}", self.me, self.opponent)
    }
}

impl std::str::FromStr for Duel {
    type Err = DuelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shapes = s.trim().split_whitespace();
        let s1 = shapes.next();
        let s2 = shapes.next();

        match (s1, s2) {
            (Some(s1_str), Some(s2_str)) => {
                let Ok(s1) = Shape::from_str(s1_str) else {
                    return Err(DuelParseError);
                };
                let Ok(s2) = Shape::from_str(s2_str) else {
                    return Err(DuelParseError);
                };

                Ok(Duel {
                    opponent: s1,
                    me: s2,
                })
            }
            _ => Err(DuelParseError),
        }
    }
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
            Self::Draw(p) | Self::Lose(p) | Self::Win(p) => *p,
        }
    }
}

pub fn assert() {
    match Duel::new(Shape::Rock, Shape::Paper).result() {
        DuelResult::Lose(1) => (), //println!("Rock vs. Paper: ✅"),
        _ => {
            println!("Rock vs. Paper: ❌");
            panic!()
        }
    }

    match Duel::new(Shape::Rock, Shape::Rock).result() {
        DuelResult::Draw(4) => (), //println!("Rock vs. Rock: ✅"),
        _ => {
            println!("Rock vs. Rock: ❌");
            panic!()
        }
    }
    match Duel::new(Shape::Rock, Shape::Scissor).result() {
        DuelResult::Win(7) => (), //println!("Rock vs. Scissor: ✅"),
        _ => {
            println!("Rock vs. Scissor: ❌");
            panic!()
        }
    }

    match Duel::new(Shape::Paper, Shape::Rock).result() {
        DuelResult::Win(8) => (), //println!("Paper vs. Rock: ✅"),
        _ => {
            println!("Paper vs. Rock: ❌");
            panic!()
        }
    }
    match Duel::new(Shape::Paper, Shape::Paper).result() {
        DuelResult::Draw(5) => (), //println!("Paper vs. Rock: ✅"),
        _ => {
            println!("Paper vs. Rock: ❌");
            panic!()
        }
    }
    match Duel::new(Shape::Paper, Shape::Scissor).result() {
        DuelResult::Lose(2) => (), //println!("Paper vs. Scissor: ✅"),
        _ => {
            println!("Paper vs. Scissor: ❌");
            panic!()
        }
    }

    match Duel::new(Shape::Scissor, Shape::Rock).result() {
        DuelResult::Lose(3) => (), //println!("Scissor vs. Rock: ✅"),
        _ => {
            println!("Scissor vs. Rock: ❌");
            panic!()
        }
    }
    match Duel::new(Shape::Scissor, Shape::Paper).result() {
        DuelResult::Win(9) => (), //println!("Scissor vs. Paper: ✅"),
        _ => {
            println!("Scissor vs. Paper: ❌");
            panic!()
        }
    }
    match Duel::new(Shape::Scissor, Shape::Scissor).result() {
        DuelResult::Draw(6) => (), //println!("Scissor vs. Scissor: ✅"),
        _ => {
            println!("Scissor vs. Scissor: ❌");
            panic!()
        }
    }
}

pub fn read_duels_from_file<P>(
    filename: P,
) -> io::Result<impl Iterator<Item = Result<Duel, DuelParseError>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mapped = lines.map(|lr| -> Result<Duel, DuelParseError> { Duel::from_str(&lr.unwrap()) });

    io::Result::Ok(mapped)
}

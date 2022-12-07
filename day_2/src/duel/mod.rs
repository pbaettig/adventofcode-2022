use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::fmt;
use std::str::FromStr;

use super::shapes::Shape;

#[derive(Debug)]
pub struct DuelParseError;
impl fmt::Display for DuelParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse Duel")
    }
}

pub struct Duel {
    pub me: Shape,
    pub opponent: Shape,
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
#[derive(PartialEq)]
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

#[derive(Debug)]
pub struct DuelResultParseError;
impl fmt::Display for DuelResultParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse DuelResult")
    }
}

fn winning_shape(s: Shape) -> Shape {
    match s {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissor,
        Shape::Scissor => Shape::Rock,
    }
}

fn losing_shape(s: Shape) -> Shape {
    match s {
        Shape::Rock => Shape::Scissor,
        Shape::Paper => Shape::Rock,
        Shape::Scissor => Shape::Paper,
    }
}

impl std::str::FromStr for DuelResult {
    type Err = DuelResultParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut token_iter = s.trim().split_whitespace();
        let tokens = (token_iter.next(), token_iter.next());

        match tokens {
            (Some(opponent_s), Some(outcome_s)) => {
                let Ok(opponent_shape) = Shape::from_str(opponent_s) else {
                    return Err(DuelResultParseError)
                };

                match outcome_s.to_uppercase().as_str() {
                    "X" => {
                        return Ok(DuelResult::Lose(
                            _DUEL_LOSE + losing_shape(opponent_shape) as u8,
                        ))
                    }
                    "Y" => return Ok(DuelResult::Draw(_DUEL_DRAW + opponent_shape as u8)),
                    "Z" => {
                        return Ok(DuelResult::Win(
                            _DUEL_WIN + winning_shape(opponent_shape) as u8,
                        ))
                    }
                    _ => return Err(DuelResultParseError),
                };
            }
            _ => Err(DuelResultParseError),
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

pub fn read_duelresults_from_file<P>(
    filename: P,
) -> io::Result<impl Iterator<Item = Result<DuelResult, DuelResultParseError>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mapped = lines.map(|lr| -> Result<DuelResult, DuelResultParseError> {
        DuelResult::from_str(&lr.unwrap())
    });

    io::Result::Ok(mapped)
}

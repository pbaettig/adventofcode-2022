use std::fmt;

#[derive(Debug)]
pub struct UnrecognizedShapeError;
impl fmt::Display for UnrecognizedShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "shape is not recognized")
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl std::str::FromStr for Shape {
    type Err = UnrecognizedShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => Err(UnrecognizedShapeError),
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

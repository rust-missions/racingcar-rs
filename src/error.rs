use std::{fmt, io::Error};

#[derive(Debug, PartialEq)]
pub enum RacingCarError {
    InvalidCarName,
    ParseIntFailed,
    ReadLineFailed,
}

impl std::error::Error for RacingCarError {}

impl fmt::Display for RacingCarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RacingCarError::InvalidCarName => {
                write!(f, "A car's name should be 5 letters or fewer")
            }
            RacingCarError::ParseIntFailed => write!(f, "Please type a number!"),
            RacingCarError::ReadLineFailed => write!(f, "Failed to read line"),
        }
    }
}

impl From<std::num::ParseIntError> for RacingCarError {
    fn from(_: std::num::ParseIntError) -> Self {
        RacingCarError::ParseIntFailed
    }
}

impl From<Error> for RacingCarError {
    fn from(_: Error) -> Self {
        RacingCarError::ReadLineFailed
    }
}

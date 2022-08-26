use std::fmt;

pub enum Error {
    BlankCarName,
    CarNameOverFiveCharacters,
    ZeroTotalRounds,
    InvalidTotalRoundsRange,
    NoWinner,
    ReadLineFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BlankCarName => write!(f, "[ERROR] 자동차 이름은 공백일 수 없습니다."),
            Error::CarNameOverFiveCharacters => write!(f, "[ERROR] 자동차 이름은 5글자 이내여야 합니다."),
            Error::ZeroTotalRounds => write!(f, "[ERROR] 횟수는 0이 될 수 없습니다."),
            Error::InvalidTotalRoundsRange => write!(f, "[ERROR] 1 이상의 정수를 입력해야 합니다. (허용 범위: 1 ~ 4294967296)"),
            Error::NoWinner => write!(f, "[ERROR] 우승자가 존재하지 않습니다."),
            Error::ReadLineFailed => write!(f, "[ERROR] 다시 입력하시오."),
        }
    }
}

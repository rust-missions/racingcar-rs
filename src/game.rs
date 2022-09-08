use crate::Car;

pub struct Game {
    round: i32,
    cars: Vec<Car>,
}

impl Game {
    pub fn new(round: i32, cars: Vec<Car>) -> Self {
        Game { round, cars }
    }
}

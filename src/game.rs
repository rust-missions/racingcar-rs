use crate::Car;

pub struct Game {
    round: i32,
    cars: Vec<Car>,
}

impl Game {
    pub fn new(round: i32, cars: Vec<Car>) -> Self {
        Game { round, cars }
    }

    pub fn run(&mut self) {}

    pub fn winners(&self) -> Vec<Car> {
        vec![Car::new("tmp")]
    }
}

#[cfg(test)]
mod test {
    use {super::*, crate::Car};

    #[test]
    fn winners() {
        let ding = Car {
            name: "ding".to_string(),
            distance: 2,
        };
        let young = Car {
            name: "young".to_string(),
            distance: 0,
        };
        let cars = vec![ding, young];
        let game = Game::new(0, cars);
        assert_eq!(game.winners(), vec![ding]); // does this test compares two vec properly?

        let hi = Car {
            name: "hi".to_string(),
            distance: 2,
        };
        let three_cars = vec![ding, hi, young];
        let game = Game::new(0, three_cars);
        assert_eq!(game.winner())
    }
}

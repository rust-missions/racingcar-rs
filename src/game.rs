use crate::cars::Cars;

pub struct Game {
    pub cars: Cars,
    pub round: u32,
    pub total_rounds: u32,
}

pub struct RoundResult {
    pub cars: Cars,
}

impl Game {
    pub fn new(cars: Cars, total_rounds: u32) -> Game {
        Game {
            cars,
            round: 0,
            total_rounds,
        }
    }

    pub fn play_all_rounds(&mut self) -> Vec<RoundResult> {
        let mut game_results = Vec::new();
        loop {
            if self.is_over() {
                break;
            }
            game_results.push(self.play());
        }
        game_results
    }

    fn play(&mut self) -> RoundResult {
        self.cars = self.cars.play_round();
        self.round += 1;
        RoundResult {
            cars: self.cars.clone(),
        }
    }

    fn is_over(&self) -> bool {
        self.round == self.total_rounds
    }

    pub fn get_winners(&self) -> Result<Vec<&str>, &'static str> {
        let max_distance = &self.cars.calculate_max_distance()?;
        let mut winners = Vec::new();
        for car in &self.cars.value {
            if max_distance > &car.distance {
                continue;
            }
            winners.push(car.name.as_str());
        }

        Ok(winners)
    }
}

#[cfg(test)]
mod tests {
    use super::Cars;
    use crate::{Car, Game};

    #[test]
    fn get_winners() {
        let mut cars = Vec::new();
        cars.push(Car {
            name: String::from("w1"),
            distance: 3,
        });
        cars.push(Car {
            name: String::from("w2"),
            distance: 3,
        });
        cars.push(Car {
            name: String::from("loser"),
            distance: 2,
        });
        let game = Game {
            cars: Cars { value: cars },
            round: 5,
            total_rounds: 10,
        };

        assert_eq!(game.get_winners(), Ok(vec!["w1", "w2"]));
    }
}

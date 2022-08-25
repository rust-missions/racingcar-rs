use crate::cars::Cars;

pub struct Game {
    pub cars: Cars,
    pub round: u32,
}

pub struct RoundResult {
    pub cars: Cars,
}

impl Game {
    pub fn new(cars: Cars) -> Game {
        Game { cars, round: 0 }
    }

    pub fn play_all_rounds(&self, total_rounds: u32) -> Vec<RoundResult> {
        let mut game = Game { cars: self.cars.clone(), round: self.round };
        let mut game_results = Vec::new();
        loop {
            if game.is_over(total_rounds) {
                break;
            }
            game = game.play();
            game_results.push(RoundResult { cars: game.cars.clone() });
        }
        game_results
    }

    fn play(&self) -> Game {
        Game { cars: self.cars.play_round(), round: self.round + 1 }
    }

    fn is_over(&self, total_rounds: u32) -> bool {
        self.round == total_rounds
    }

    pub fn get_winners(&self) -> Vec<String> {
        let max_distance = &self.cars.calculate_max_distance();
        let mut winners = Vec::new();
        for car in &self.cars.value {
            if max_distance > &car.distance {
                continue;
            }
            winners.push(car.name.clone());
        }
        winners
    }
}

#[cfg(test)]
mod tests {
    use crate::{Car, Game};
    use super::Cars;

    #[test]
    fn get_winners() {
        let mut cars = Vec::new();
        cars.push(Car { name: String::from("w1"), distance: 3 });
        cars.push(Car { name: String::from("w2"), distance: 3 });
        cars.push(Car { name: String::from("loser"), distance: 2 });
        let game = Game { cars: Cars { value: cars }, round: 5 };

        assert_eq!(game.get_winners(), vec!["w1", "w2"]);
    }
}

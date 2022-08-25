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

    pub fn play_all_rounds(&mut self, total_rounds: u32) -> Vec<RoundResult> {
        let mut game_results = Vec::new();
        loop {
            if self.is_over(total_rounds) {
                break;
            }
            game_results.push(self.play());
        }
        game_results
    }

    fn play(&mut self) -> RoundResult {
        self.cars = self.cars.play_round();
        self.round += 1;
        RoundResult { cars: self.cars.clone() }
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

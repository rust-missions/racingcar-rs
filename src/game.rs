use crate::Car;
use rand::Rng;

pub struct Game {
    pub cars: Vec<Car>,
    pub round: u32,
}

impl Game {
    pub fn new(car_names: Vec<String>) -> Game {
        let mut cars = Vec::new();
        for car_name in car_names {
            cars.push(Car::new(car_name.as_str()).unwrap());
        }
        Game { cars, round: 0 }
    }

    pub fn play(self) -> Game {
        let mut new_cars = Vec::new();
        for car in self.cars {
            let random_number = rand::thread_rng().gen_range(0..=9);
            new_cars.push(car.play(random_number));
        }
        Game { cars: new_cars, round: self.round + 1 }
    }

    pub fn is_over(&self, total_rounds: u32) -> bool {
        self.round == total_rounds
    }

    pub fn get_winners(&self) -> Vec<String> {
        let max_distance = &self.cars.iter()
            .max_by_key(|car| car.distance)
            .unwrap()
            .distance;

        let mut winners = Vec::new();
        for car in &self.cars {
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
    use crate::Game;
    use super::Car;

    #[test]
    fn get_winners() {
        let mut cars = Vec::new();
        cars.push(Car { name: String::from("w1"), distance: 3 });
        cars.push(Car { name: String::from("w2"), distance: 3 });
        cars.push(Car { name: String::from("loser"), distance: 2 });
        let game = Game { cars, round: 5 };

        assert_eq!(game.get_winners(), vec!["w1", "w2"]);
    }
}

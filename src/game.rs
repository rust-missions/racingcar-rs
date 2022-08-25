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
}

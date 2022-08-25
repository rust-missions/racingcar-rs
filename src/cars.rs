use crate::Car;
use rand::Rng;

pub struct Cars {
    pub value: Vec<Car>,
}

impl Cars {
    pub fn new(car_names: Vec<String>) -> Result<Cars, &'static str> {
        let mut cars = Vec::new();
        for car_name in car_names {
            cars.push(Car::new(car_name.as_str()).unwrap());
        }
        Ok(Cars { value: cars })
    }

    pub fn clone(&self) -> Cars {
        let mut cars = Vec::new();
        for car in &self.value {
            cars.push(Car { name: car.name.clone(), distance: car.distance });
        }
        Cars { value: cars }
    }

    pub fn play_round(&self) -> Cars {
        let mut new_cars = Vec::new();
        for car in &self.value {
            let random_number = rand::thread_rng().gen_range(0..=9);
            new_cars.push(car.play(random_number));
        }
        Cars { value: new_cars }
    }

    pub fn calculate_max_distance(&self) -> u32 {
        self.value.iter()
            .max_by_key(|car| car.distance)
            .unwrap()
            .distance
    }
}

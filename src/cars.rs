use crate::Car;
use rand::Rng;

pub struct Cars {
    pub value: Vec<Car>,
}

impl Cars {
    pub fn new(car_names: Vec<String>) -> Result<Cars, &'static str> {
        let mut cars = Vec::new();
        for car_name in car_names {
            let car = match Car::new(car_name.as_str()) {
                Ok(car) => car,
                Err(e) => return Err(e),
            };
            cars.push(car);
        }
        Ok(Cars { value: cars })
    }

    pub fn clone(&self) -> Cars {
        let mut cars = Vec::new();
        for car in &self.value {
            cars.push(Car {
                name: car.name.clone(),
                distance: car.distance,
            });
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

    pub fn calculate_max_distance(&self) -> Result<u32, &'static str> {
        let max_distance = self
            .value
            .iter()
            .max_by_key(|car| car.distance)
            .ok_or_else(|| "[ERROR] 우승자가 존재하지 않습니다.")?
            .distance;

        Ok(max_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::Cars;
    use crate::Car;

    #[test]
    fn calculate_max_distance() {
        let mut cars = Vec::new();
        cars.push(Car {
            name: String::from("winner"),
            distance: 10,
        });
        cars.push(Car {
            name: String::from("loser"),
            distance: 5,
        });
        let cars = Cars { value: cars };

        assert_eq!(cars.calculate_max_distance(), Ok(10));
    }

    #[test]
    fn err_on_empty_cars() {
        let cars = Cars { value: Vec::new() };
        assert!(cars.calculate_max_distance().is_err());
    }
}

use {crate::Car, std::cmp::Ordering};

#[derive(Debug)]
pub struct Game {
    round: i32,
    cars: Vec<Car>,
}

impl Game {
    pub fn new(round: i32, cars: Vec<Car>) -> Self {
        Game { round, cars }
    }

    pub fn run(&mut self) {
        for _ in 0..self.round {
            for car in self.cars.iter_mut() {
                car.move_forward();
                println!("{}", car);
            }
            println!();
        }
    }

    pub fn winners(&self) -> Vec<&Car> {
        let mut longest_distance = 0;
        let mut winners: Vec<&Car> = Vec::new();

        for car in self.cars.iter() {
            match car.distance.cmp(&longest_distance) {
                Ordering::Less => {}
                Ordering::Equal => {
                    winners.push(car);
                }
                Ordering::Greater => {
                    longest_distance = car.distance;
                    winners.pop();
                    winners.push(car);
                }
            }
            // if car.distance > longest_distance {
            //     longest_distance = car.distance;
            //     winners.pop();
            //     winners.push(car);
            // } else if car.distance == longest_distance {
            //     winners.push(car);
            // } else {
            // }
        }
        winners
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        let hi = Car {
            name: "hi".to_string(),
            distance: 2,
        };

        let cars = vec![ding, young, hi];
        let game = Game::new(0, cars);
        println!("{:?}", game.winners());
    }
}

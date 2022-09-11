use {
    crate::{car::Car, error::RacingCarError, game::Game},
    std::io,
};

mod car;
mod error;
mod game;

pub type Result<T> = std::result::Result<T, RacingCarError>;

fn main() {
    // 1. read & parse input
    let cars: Vec<Car> = valid_cars_from_input();
    let round = valid_round_from_input();

    // 2. make a game & run
    let mut game = Game::new(round, cars);
    println!("실행 결과");
    game.run();

    // 3. find & print winners
    let winners = game.winners();
    let output = format_winners(winners);
    println!("{}", output);
}

fn valid_cars_from_input() -> Vec<Car> {
    loop {
        match read_input_cars() {
            Ok(cars) => {
                return cars;
            }
            Err(e) => eprintln!("[ERROR] {}", e),
        }
    }
}

fn valid_round_from_input() -> i32 {
    loop {
        match read_input_round() {
            Ok(round) => return round,
            Err(e) => eprintln!("[ERROR] {}", e),
        }
    }
}

fn read_input_cars() -> Result<Vec<Car>> {
    let mut cars = String::new();

    println!("경주할 자동차 이름을 입력하세요. (이름은 쉼표(,) 기준으로 구분)");

    io::stdin().read_line(&mut cars)?;

    println!();

    let car_names: Vec<&str> = cars.trim().split(',').collect();
    let cars: std::result::Result<Vec<_>, _> = car_names.iter().map(|car| Car::new(car)).collect();

    cars
}

fn read_input_round() -> Result<i32> {
    let mut round = String::new();

    println!("시도할 횟수는 몇회인가요?");

    io::stdin().read_line(&mut round)?;

    println!();

    let round: i32 = round.trim().parse()?;
    Ok(round)
}

fn format_winners(winners: Vec<&Car>) -> String {
    let winner_names = winners
        .iter()
        .map(|&car| car.name.to_owned())
        .collect::<Vec<String>>()
        .join(", ");
    format!("최종 우승자 : {}", winner_names)
}

#[cfg(test)]
mod tests {
    use crate::{format_winners, Car};

    #[test]
    fn test_format_winners() {
        let ding = Car::new("ding").unwrap();
        let young = Car::new("young").unwrap();
        let hi = Car::new("hi").unwrap();

        let winners = vec![&ding, &young, &hi];
        let output = format_winners(winners);
        assert_eq!("최종 우승자 : ding, young, hi", output);

        let winners = vec![&ding];
        let output = format_winners(winners);
        assert_eq!("최종 우승자 : ding", output);
    }
}

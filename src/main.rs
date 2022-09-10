extern crate core;

use {
    crate::{car::Car, game::Game},
    std::io,
};

mod car;
mod game;

fn main() {
    // implement your logic

    // 1. parse input
    let cars = parse_cars();
    let round = parse_round();

    // 2. make a game & run
    let mut game = Game::new(round, cars);
    println!("실행 결과");
    game.run();

    // 3. find & print winners
    let winners = game.winners();
    let output = format_winners(winners);
    println!("{}", output);
}

fn parse_cars() -> Vec<Car> {
    let mut cars = String::new();

    println!("경주할 자동차 이름을 입력하세요. (이름은 쉼표(,) 기준으로 구분)");

    io::stdin()
        .read_line(&mut cars)
        .expect("[ERROR] Failed to read line");

    println!();

    let car_names: Vec<&str> = cars.trim().split(',').collect();
    let cars = car_names.iter().map(|name| Car::new(name)).collect();

    cars
}

fn parse_round() -> i32 {
    let mut round = String::new();

    println!("시도할 횟수는 몇회인가요?");

    io::stdin()
        .read_line(&mut round)
        .expect("[ERROR] Failed to read line");

    println!();

    let round: i32 = round.trim().parse().expect("[ERROR] Please type a number!");

    round
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
        let ding = Car::new("ding");
        let young = Car::new("young");
        let hi = Car::new("hi");

        let winners = vec![&ding, &young, &hi];
        let output = format_winners(winners);
        assert_eq!("최종 우승자 : ding, young, hi", output);

        let winners = vec![&ding];
        let output = format_winners(winners);
        assert_eq!("최종 우승자 : ding", output);
    }
}

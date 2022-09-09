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
    game.run();

    // 3. find & print winners
    let winners = game.winners();
    let output = format_winners(winners);
    println!("{}", output);
}

fn parse_cars() -> Vec<Car> {
    let mut cars = String::new();

    io::stdin()
        .read_line(&mut cars)
        .expect("Failed to read line");

    let car_names: Vec<&str> = cars.trim().split(',').collect();
    let cars = car_names.iter().map(|name| Car::new(name)).collect();

    cars
}

fn parse_round() -> i32 {
    let mut round = String::new();

    io::stdin()
        .read_line(&mut round)
        .expect("Failed to read line");

    let round: i32 = round.trim().parse().expect("Please type a number!");

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
        assert_eq!("최종 우승자 : ding, young, hi", winner_names);

        let winners = vec![&ding];
        let output = format_winners(winners);
        assert_eq!("최종 우승자 : ding", winner_names);
    }
}

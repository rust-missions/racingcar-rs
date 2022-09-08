use {
    crate::{car::Car, game::Game},
    std::io,
};

mod car;
mod game;

fn main() {
    // implement your logic
    // 1. parse input
    let mut cars = String::new();

    io::stdin()
        .read_line(&mut cars)
        .expect("Failed to read line");

    let car_names: Vec<&str> = cars.trim().split(',').collect();
    println!("{:?}", car_names);

    let cars = car_names.iter().map(|name| Car::new(name)).collect();
    println!("{:?}", cars);

    let mut round = String::new();

    io::stdin()
        .read_line(&mut round)
        .expect("Failed to read line");

    let round: i32 = round.trim().parse().expect("Please type a number!");
    println!("{}", round);

    // 2. make a game & run
    let game = Game::new(round, cars);
}

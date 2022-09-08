use {crate::model::Car, std::io};

mod model;

fn main() {
    // implement your logic
    // 1. parse input
    let mut cars = String::new();

    io::stdin()
        .read_line(&mut cars)
        .expect("Failed to read line");

    let cars: Vec<&str> = cars.trim().split(',').collect();
    println!("{:?}", cars);

    let mut round = String::new();

    io::stdin()
        .read_line(&mut round)
        .expect("Failed to read line");

    let round: u32 = round.trim().parse().expect("Please type a number!");
    println!("{}", round);
}

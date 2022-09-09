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
    let mut game = Game::new(round, cars);
    game.run();

    // 3. print winners
    let winners = game.winners();
    let winner_names = winners
        .iter()
        .map(|&car| car.name.to_owned())
        .collect::<Vec<String>>()
        .join(", ");
    println!("{}", format!("최종 우승자 : {}", winner_names))
}

#[cfg(test)]
mod tests {
    use crate::Car;

    #[test]
    fn print_output() {
        let ding = Car {
            name: "ding".to_string(),
            distance: 2,
        };
        let young = Car {
            name: "young".to_string(),
            distance: 2,
        };
        let hi = Car {
            name: "hi".to_string(),
            distance: 2,
        };

        let winners = vec![&ding, &young, &hi];
        let winner_names = winners
            .iter()
            .map(|&car| car.name.to_owned())
            .collect::<Vec<String>>()
            .join(", ");
        assert_eq!(
            "최종 우승자 : ding, young, hi",
            format!("최종 우승자 : {}", winner_names)
        );

        let winners = vec![&ding];
        let winner_names = winners
            .iter()
            .map(|&car| car.name.to_owned())
            .collect::<Vec<String>>()
            .join(", ");
        assert_eq!(
            "최종 우승자 : ding",
            format!("최종 우승자 : {}", winner_names)
        );
    }
}

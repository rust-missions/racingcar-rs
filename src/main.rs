use crate::car::Car;
use crate::cars::Cars;
use crate::game::Game;

mod car;
mod cars;
mod game;
mod view;

fn main() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    let cars = Cars::new(view::read_car_names_input()).unwrap();
    let total_rounds = view::read_total_rounds_input();

    let mut game = Game::new(cars);
    println!("실행결과");
    loop {
        if game.is_over(total_rounds) {
            break;
        }
        game = game.play();
        for car in &game.cars.value {
            println!("{} : {}", car.name, "-".repeat(car.distance as usize));
        }
        println!();
    }
    view::winners_output(game.get_winners());
}

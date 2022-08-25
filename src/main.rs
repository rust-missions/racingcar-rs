use crate::car::Car;
use crate::cars::Cars;
use crate::game::Game;

mod car;
mod cars;
mod game;
mod view;

fn main() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    let cars :Cars = loop {
        break match Cars::new(view::read_car_names_input()) {
            Ok(cars) => cars,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        }
    };
    let total_rounds = view::read_total_rounds_input();

    let mut game = Game::new(cars, total_rounds);
    view::print_game_result(game.play_all_rounds());
    view::print_winners_output(game.get_winners());
}

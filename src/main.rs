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

    let game = Game::new(cars);
    view::game_result(game.play_all_rounds(total_rounds));
    view::winners_output(game.get_winners());
}

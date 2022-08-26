use crate::car::Car;
use crate::cars::Cars;
use crate::game::Game;

mod car;
mod cars;
mod game;
mod view;

fn main() {
    let cars: Cars = view::read_cars_input();
    let total_rounds = view::read_total_rounds_input();

    let mut game = Game::new(cars, total_rounds);
    view::print_game_result(game.play_all_rounds());
    view::print_winners_output(game.get_winners());
}

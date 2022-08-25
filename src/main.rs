use crate::car::Car;

mod car;
mod view;

fn main() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    let car_names: Vec<String> = view::read_car_names_input();
    println!("{}", view::read_total_rounds_input());

    let mut cars = Vec::new();
    for car_name in car_names {
        cars.push(Car::new(car_name.as_str()).unwrap());
    }
}

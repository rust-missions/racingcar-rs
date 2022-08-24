mod car;
mod view;

fn main() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    let car_names: Vec<String> = view::read_car_names_input();
}

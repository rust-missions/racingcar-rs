use std::io;

pub fn read_car_names_input() -> Vec<String> {
    let mut car_names_input = String::new();
    io::stdin().read_line(&mut car_names_input).unwrap();

    let mut names = Vec::new();
    for name in car_names_input.split(",") {
        names.push(name.to_string());
    }
    names
}

use std::io;
use crate::game::RoundResult;

pub fn read_car_names_input() -> Vec<String> {
    let mut car_names_input = String::new();
    io::stdin().read_line(&mut car_names_input).unwrap();

    let mut names = Vec::new();
    for name in car_names_input.split(",") {
        names.push(name.to_string());
    }
    names
}

pub fn read_total_rounds_input() -> u32 {
    println!("시도할 횟수는 몇회인가요?");
    loop {
        let mut total_rounds_input = String::new();
        io::stdin().read_line(&mut total_rounds_input).expect("Failed to read line");

        return match total_rounds_input.trim().parse::<u32>() {
            Ok(num) => {
                if num == 0 {
                    println!("횟수는 0이 될 수 없습니다.");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("1 이상의 정수를 입력해야 합니다. (허용 범위: 1 ~ 4294967296)");
                continue;
            }
        };
    }
}

pub fn game_result(round_results: Vec<RoundResult>) {
    println!("실행결과");
    for round_result in &round_results {
        for car in &round_result.cars.value {
            println!("{} : {}", car.name, "-".repeat(car.distance as usize));
        }
        println!();
    }
}

pub fn winners_output(winners: Vec<String>) {
    println!("최종 우승자 : {}", winners.join(", "))
}

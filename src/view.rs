use crate::game::RoundResult;
use crate::Cars;
use std::io;

pub fn read_cars_input() -> Cars {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    loop {
        let car_names = match read_car_names_input() {
            Ok(car_names) => car_names,
            Err(_) => {
                eprintln!("{}", "[ERROR] 다시 입력하시오.");
                continue;
            }
        };

        break match Cars::new(car_names) {
            Ok(cars) => cars,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
    }
}

fn read_car_names_input() -> Result<Vec<String>, io::Error> {
    let mut car_names_input = String::new();
    io::stdin().read_line(&mut car_names_input)?;

    let mut names = Vec::new();
    for name in car_names_input.split(",") {
        names.push(name.to_string());
    }
    Ok(names)
}

pub fn read_total_rounds_input() -> u32 {
    println!("시도할 횟수는 몇회인가요?");
    loop {
        let mut total_rounds_input = String::new();
        match io::stdin().read_line(&mut total_rounds_input) {
            Ok(rounds_input) => rounds_input,
            Err(_) => {
                eprintln!("{}", "[ERROR] 다시 입력하시오.");
                continue;
            }
        }

        return match total_rounds_input.trim().parse::<u32>() {
            Ok(num) => {
                if num == 0 {
                    println!("[ERROR] 횟수는 0이 될 수 없습니다.");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("[ERROR] 1 이상의 정수를 입력해야 합니다. (허용 범위: 1 ~ 4294967296)");
                continue;
            }
        };
    }
}

pub fn print_game_result(round_results: Vec<RoundResult>) {
    let round_result_announcements = round_results
        .iter()
        .map(|round_result| format!("{}\n", collect_and_join_round_result(round_result)))
        .collect::<Vec<String>>()
        .join("\n");
    println!("실행결과\n{}", round_result_announcements);
}

fn collect_and_join_round_result(round_result: &RoundResult) -> String {
    round_result
        .cars
        .value
        .iter()
        .map(|car| format!("{} : {}", car.name, "-".repeat(car.distance as usize)))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn print_winners_output(winners: Vec<&str>) {
    println!("최종 우승자 : {}", winners.join(", "))
}

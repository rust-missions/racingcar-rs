use {
    rand::Rng,
    std::{cmp, io},
};

pub fn print_car_name_format() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
}

pub fn get_car_names_from_input() -> Vec<String> {
    let mut cars: Vec<String> = Vec::new();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("잘못된 입력입니다.");

    let splited: Vec<&str> = line.trim().split(',').collect();
    for i in 0..splited.len() as i64 {
        cars.push(splited[i as usize].to_owned());
    }
    cars
}

pub fn print_total_round_format() {
    println!("전체 라운드 수는 몇회인가요?");
}

pub fn get_total_round_from_input() -> i64 {
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("잘못된 입력입니다.");
        let total_round = match line.trim().parse::<i64>() {
            Ok(total_round) => total_round,
            Err(_) => {
                eprintln!("[Error] 시도 횟수는 숫자여야 한다.");
                continue;
            }
        };
        return total_round;
    }
}

pub fn print_result_per_round(car_names: Vec<String>, dist_info: Vec<String>) {
    let cars_num: i64 = car_names.len() as i64;
    for car_num in 0..cars_num {
        println!(
            "{} : {}",
            car_names[car_num as usize], dist_info[car_num as usize]
        );
    }
    println!();
}

pub fn run_round(car_names: Vec<String>, dist_info: &mut [String]) {
    let mut rng = rand::thread_rng();
    let cars_num: i64 = car_names.len() as i64;
    for car_num in 0..cars_num {
        let rand_num: i64 = rng.gen_range(0..10);
        if rand_num >= 4 {
            let s = dist_info[car_num as usize].to_owned() + "-";
            dist_info[car_num as usize] = s;
        }
    }
}

pub fn print_result_format() {
    println!("각 라운드 별 경기 결과");
}

pub fn print_result_from(car_names: Vec<String>, dist_info: Vec<String>) {
    let mut scores: Vec<usize> = Vec::new();
    let mut max_scores = 0;
    let mut winner: Vec<&str> = Vec::new();
    for i in 0..dist_info.len() as i64 {
        let score = dist_info[i as usize].len();
        scores.push(score);
        max_scores = cmp::max(max_scores, score);
    }

    for i in 0..scores.len() as i64 {
        let score = dist_info[i as usize].len();
        if score == max_scores {
            winner.push(&car_names[i as usize]);
        }
    }
    print!("최종 우승자 : ");
    for i in 0..winner.len() as i64 {
        print!("{}", winner[i as usize]);
        if i != (winner.len() - 1) as i64 {
            print!(", ");
        }
    }
    println!();
}

fn main() {
    print_car_name_format();
    let car_names: Vec<String> = get_car_names_from_input();
    let cars_num: i64 = car_names.len() as i64;

    print_total_round_format();
    let total_round: i64 = get_total_round_from_input();

    let mut dist_info: Vec<String> = Vec::new();
    for _ in 0..cars_num as i64 {
        dist_info.push("".to_owned());
    }

    print_result_format();

    for _ in 0..total_round {
        run_round(car_names.clone(), &mut dist_info);
        print_result_per_round(car_names.clone(), dist_info.clone());
    }

    print_result_from(car_names, dist_info);
}

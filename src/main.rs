use {
    rand::Rng,
    std::{cmp, fmt::Error, io},
};

pub fn print_car_name_format() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
}

pub fn get_car_names_from_input() -> Result<Vec<String>, Error> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("잘못된 입력입니다.");
    let splited: Vec<&str> = line.trim().split(',').collect();
    let mut cars: Vec<String> = Vec::new();
    for i in 0..splited.len() as i64 {
        cars.push(splited[i as usize].to_owned());
    }
    Ok(cars)
}

pub fn print_total_round_format() {
    println!("전체 라운드 수는 몇회인가요?");
}

pub fn get_total_round_from_input() -> Result<i64, Error> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("잘못된 입력입니다.");
    let total_round = line.trim().parse::<i64>().unwrap();
    Ok(total_round)
}

pub fn print_result_per_round(car_names: Vec<String>, dist_info: Vec<String>) {
    let cars_num: i64 = car_names.len() as i64;
    for car_num in 0..cars_num {
        print!(
            "{} : {}\n",
            car_names[car_num as usize], dist_info[car_num as usize]
        );
    }
    println!();
}

pub fn run_round(car_names: Vec<String>, dist_info: &mut Vec<String>) {
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

fn main() {
    print_car_name_format();
    let car_names: Vec<String> = get_car_names_from_input().unwrap();
    let cars_num: i64 = car_names.len() as i64;

    print_total_round_format();
    let total_round: i64 = get_total_round_from_input().unwrap();

    let mut dist_info: Vec<String> = Vec::new();
    for _ in 0..cars_num as i64 {
        dist_info.push("".to_owned());
    }

    for _ in 0..total_round {
        run_round(car_names.clone(), &mut dist_info);
        print_result_per_round(car_names.clone(), dist_info.clone());
    }
}

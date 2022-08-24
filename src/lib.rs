pub struct Car {
    pub name: String,
    pub distance: i32,
}

impl Car {
    pub fn new(name: &str) -> Result<Car, &'static str> {
        let name_char_length = name.chars().count();
        if name_char_length == 0 {
            return Err("자동차 이름은 공백일 수 없습니다.");
        }
        if name_char_length > 5 {
            return Err("자동차 이름은 5글자 이내여야 합니다.");
        }
        Ok(Car { name: String::from(name), distance: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::Car;

    #[test]
    fn initial_distance_zero() {
        let name = "유효한이름";
        let car = Car::new(name).unwrap();
        assert_eq!(car.distance, 0);
    }

    #[test]
    fn name_min_one_character() {
        let name = "";
        assert!(Car::new(name).is_err())
    }

    #[test]
    fn name_max_five_characters() {
        let name = "여섯글자이름";
        assert!(Car::new(name).is_err())
    }
}

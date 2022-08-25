pub struct Car {
    pub name: String,
    pub distance: u32,
}

impl Car {
    pub fn new(name: &str) -> Result<Car, &'static str> {
        let name = name.trim();
        let name_char_length = name.chars().count();
        if name_char_length == 0 {
            return Err("[ERROR] 자동차 이름은 공백일 수 없습니다.");
        }
        if name_char_length > 5 {
            return Err("[ERROR] 자동차 이름은 5글자 이내여야 합니다.");
        }

        Ok(Car { name: String::from(name), distance: 0 })
    }

    pub fn play(&self, number: i8) -> Car {
        if number >= 4 {
            return Car { name: self.name.clone(), distance: self.distance + 1 };
        }
        Car { name: self.name.clone(), distance: self.distance }
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
    fn name_blank_not_allowed() {
        let name = " ";
        assert!(Car::new(name).is_err())
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

    #[test]
    fn move_forward_on_4_or_more() {
        let car = Car::new("유효한이름").unwrap();
        let car = car.play(4);

        assert_eq!(car.distance, 1);
    }

    #[test]
    fn stop_on_3_or_less() {
        let car = Car::new("유효한이름").unwrap();
        let car = car.play(3);

        assert_eq!(car.distance, 0);
    }
}

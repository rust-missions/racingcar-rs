use {
    rand::Rng,
    std::fmt,
    std::fmt::{Display, Formatter},
};

#[derive(Debug, Eq, PartialEq)]
pub struct Car {
    pub name: String,
    pub distance: i32,
}

impl Car {
    pub fn new(name: &str) -> Self {
        if name.len() > 5 {
            panic!("[ERROR] 이름은 5자 이하만 가능합니다.");
        }
        Car {
            name: name.to_owned(),
            distance: 0,
        }
    }

    pub fn move_forward(&mut self) {
        let rand_num = rand::thread_rng().gen_range(0..=10);
        if rand_num >= 4 {
            self.distance += 1;
        }
    }
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.name, self.distance)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_car() {
        let car = Car::new("ding");
        assert_eq!("ding : 0", format!("{}", car));
    }
}

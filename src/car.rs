use {
    rand::Rng,
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug, Eq, PartialEq)]
pub struct Car {
    pub name: String,
    pub distance: i32,
}

impl Car {
    pub fn new(name: &str) -> Self {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} : {}", self.name, self.distance)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_car() {
        let car = Car::new("ding-young");
        assert_eq!("ding-young : 0", format!("{}", car));
    }
}

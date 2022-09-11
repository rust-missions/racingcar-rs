use {
    super::Result,
    crate::RacingCarError,
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
    pub fn new(name: &str) -> Result<Self> {
        if name.len() > 5 {
            return Err(RacingCarError::InvalidCarName);
        }
        Ok(Car {
            name: name.to_owned(),
            distance: 0,
        })
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
    fn new() {
        let invalid_car = Car::new("tooLong");
        assert_eq!(Err(RacingCarError::InvalidCarName), invalid_car);
    }

    #[test]
    fn print_car() {
        let car = Car::new("ding").unwrap();
        assert_eq!("ding : 0", format!("{}", car));
    }
}

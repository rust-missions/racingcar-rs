use {
    rand::Rng,
    std::fmt::{Display, Formatter},
};

pub struct Car {
    name: String,
    distance: i32,
}

impl Car {
    fn new(name: String) -> Self {
        Car { name, distance: 0 }
    }

    fn move_forward(&mut self) {
        let rand_num = rand::thread_rng().gen_range(0..=10);
        if rand_num >= 4 {
            self.distance += 1;
        }
    }
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.name, self.distance)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_car() {
        let car = Car::new("ding-young".to_string());
        assert_eq!("ding-young : 0", format!("{}", car));
    }
}

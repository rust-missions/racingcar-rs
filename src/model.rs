use ::rand::Rng;

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

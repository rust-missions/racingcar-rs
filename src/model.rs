pub struct Car {
    name: String,
    distance: i32,
}

impl Car {
    fn new(name: String) -> Self {
        Car { name, distance: 0 }
    }
}

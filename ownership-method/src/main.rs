struct Sensor {
    value: i32,
    valid: bool,
}

impl Sensor {
    // Version 1 - takes ownership
    fn is_valid(self) -> bool {
        self.valid
    }
    
    // Version 2 - borrows
    fn is_valid_borrow(&self) -> bool {
        self.valid
    }
}

fn main() {
    let sensor = Sensor { value: 42, valid: true };
    
    println!("Valid: {}", sensor.is_valid_borrow());
    println!("Still have sensor: {}", sensor.value);
}
struct ControlBuilder {
    setpoint: f32,
    gain: f32,
}

impl ControlBuilder {
    fn new() -> Self {
        Self {
            setpoint: 0.0,
            gain: 1.0,
        }
    }

    fn setpoint(mut self, value: f32) -> Self {
        self.setpoint = value;
        self
    }

    fn gain(mut self, value: f32) -> Self {
        self.gain = value;
        self
    }

    fn run(self) -> f32 {
        self.setpoint * self.gain
    }
}

fn main() {
    let result = ControlBuilder::new()
        .setpoint(80.0)
        .gain(1.5)
        .run();
    println!("Result: {}", result);
}
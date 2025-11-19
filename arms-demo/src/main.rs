#[derive(PartialEq, Copy, Clone)]
enum MachineState {
    Idle,
    Running,
    Fault,
}

fn main() {
    let state = MachineState::Running;

    // match with arms
    let net_flow = match state {
        MachineState::Idle => 5.0, // arm 1
        MachineState::Running => -3.0, // arm 2
        MachineState::Fault => 0.0, // arm 3
    };

    println!("Net flow: {}", net_flow); // -3.0

    // If as expression with arms
    let outflow = if state == MachineState::Running { 8.0 } else { 0.0 }; // if arm else arm
    println!("Outflow: {}", outflow); // 8.0
}
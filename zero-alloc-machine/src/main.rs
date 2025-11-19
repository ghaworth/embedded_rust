#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Running,
    Fault,
}

#[entry]
fn main() -> ! {
    // Force compiler to keep code
    unsafe {
        core::ptr::write_volatile(0xDEAD_BEEF as *mut u32, 42);
    }
    
    let mut current_state = State::Idle;
    let sensor_input = true;
    let error_input = false;

    let mut log: [u32; 5] = [0; 5];
    let mut log_index = 0;

    current_state = transition(current_state, sensor_input, error_input);
    log_entry(&mut log, &mut log_index, &current_state);

    loop {}
}

fn transition(current: State, sensor: bool, error: bool) -> State {
    match current {
        State::Idle if sensor => State::Running,
        State::Running if error => State::Fault,
        state => state,
    }
}

fn log_entry(log: &mut [u32; 5], index: &mut usize, state: &State) {
    if *index < log.len() {
        log[*index] = match state {
            State::Idle => 0,
            State::Running => 1,
            State::Fault => 2,
        };
        *index += 1;
    }
}
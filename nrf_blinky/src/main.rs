#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_hal::{
    gpio::{p0::Parts as P0Parts, Level},
    pac::Peripherals,
    prelude::*,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins = P0Parts::new(p.P0);

    let mut led = pins.p0_13.into_push_pull_output(Level::High);

    // Remove the `let mut delay = ...` line
    // Just call delay() directly

    loop {
        led.set_low().unwrap();   // ON
        cortex_m::asm::delay(8_000_000);  // ← FIXED
        led.set_high().unwrap();  // OFF
        cortex_m::asm::delay(8_000_000);  // ← FIXED
    }
}
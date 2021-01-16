#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::{Peripherals, Pins, delay_ms};

const INPUT_REST_DELAY_MS: u16 = 100;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut led_pin = pins.d13.into_output(&mut pins.ddr);
    let switch_pin = pins.d2.into_pull_up_input(&mut pins.ddr);
    
    loop {
        if switch_pin.is_high().void_unwrap() {
            led_pin.set_high().void_unwrap();
            delay_ms(INPUT_REST_DELAY_MS);
        } else {
            led_pin.set_low().void_unwrap();
            delay_ms(INPUT_REST_DELAY_MS);
        }
    }
}

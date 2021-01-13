#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::{Peripherals, Pins, delay_ms};

#[arduino_uno::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut buzzer_pin = pins.d12.into_output(&mut pins.ddr);

    loop {
        for _i in 0..100 {
            buzzer_pin.set_high().void_unwrap();
            delay_ms(10);
            buzzer_pin.set_low().void_unwrap();
            delay_ms(1);
        }
    }
}

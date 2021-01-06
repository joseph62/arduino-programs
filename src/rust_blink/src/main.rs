#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();

    loop {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(800);
    }
}

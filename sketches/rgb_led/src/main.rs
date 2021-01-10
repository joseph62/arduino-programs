#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::delay_ms;
use arduino_uno::prelude::*;
use arduino_uno::pwm::{Prescaler, Timer0Pwm, Timer2Pwm};

const DELAY_TIME: u16 = 10;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let mut blue = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer2);
    let mut green = pins.d5.into_output(&mut pins.ddr).into_pwm(&mut timer0);
    let mut red = pins.d6.into_output(&mut pins.ddr).into_pwm(&mut timer0);

    red.enable();
    green.enable();
    blue.enable();

    let mut red_value: u8;
    let mut green_value: u8;
    let mut blue_value: u8;

    loop {
        red_value = 255;
        green_value = 0;
        blue_value = 0;

        red.set_duty(red_value);
        green.set_duty(green_value);
        blue.set_duty(blue_value);

        while red_value > 0 {
            green_value += 1;
            red_value -= 1;

            red.set_duty(red_value);
            green.set_duty(green_value);

            delay_ms(DELAY_TIME);
        }

        red_value = 0;
        green_value = 255;
        blue_value = 0;

        red.set_duty(red_value);
        green.set_duty(green_value);
        blue.set_duty(blue_value);

        while green_value > 0 {
            blue_value += 1;
            green_value -= 1;

            green.set_duty(green_value);
            blue.set_duty(blue_value);

            delay_ms(DELAY_TIME);
        }

        red_value = 0;
        green_value = 0;
        blue_value = 255;

        red.set_duty(red_value);
        green.set_duty(green_value);
        blue.set_duty(blue_value);

        while blue_value > 0 {
            blue_value -= 1;
            red_value += 1;

            blue.set_duty(blue_value);
            red.set_duty(red_value);

            delay_ms(DELAY_TIME);
        }
    }
}

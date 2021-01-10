#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::delay_ms;
use arduino_uno::prelude::*;
use arduino_uno::pwm::{Prescaler, Timer0Pwm, Timer2Pwm};

const DELAY: u16 = 100;

enum Color {
    Red,
    Green,
    Blue,
}

fn next_color(current: &Color) -> Color {
    match current {
        Color::Red => Color::Green,
        Color::Green => Color::Blue,
        Color::Blue => Color::Red,
    }
}

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let color_change_pin = pins.d8.into_pull_up_input(&mut pins.ddr);

    let mut red_pin = pins.d6.into_output(&mut pins.ddr).into_pwm(&mut timer0);
    let mut green_pin = pins.d5.into_output(&mut pins.ddr).into_pwm(&mut timer0);
    let mut blue_pin = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer2);

    let mut current_color = Color::Red;

    red_pin.set_duty(255);
    green_pin.set_duty(0);
    blue_pin.set_duty(0);

    red_pin.enable();
    green_pin.enable();
    blue_pin.enable();

    loop {
        if color_change_pin.is_low().void_unwrap() {
            current_color = next_color(&current_color);
            match current_color {
                Color::Red => {
                    red_pin.set_duty(255);
                    green_pin.set_duty(0);
                    blue_pin.set_duty(0);
                }
                Color::Green => {
                    red_pin.set_duty(0);
                    green_pin.set_duty(255);
                    blue_pin.set_duty(0);
                }
                Color::Blue => {
                    red_pin.set_duty(0);
                    green_pin.set_duty(0);
                    blue_pin.set_duty(255);
                }
            }
        }
        delay_ms(DELAY);
    }
}

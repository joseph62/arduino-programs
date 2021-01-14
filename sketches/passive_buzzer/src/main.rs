#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::prelude::*;
use arduino_uno::{Peripherals, Pins, delay_us};
use arduino_uno::pwm::{Prescaler, Timer0Pwm};

const NOTE_C5: u16 = 523;
const NOTE_D5: u16 = 587;
const NOTE_E5: u16 = 659;
const NOTE_F5: u16 = 698;
const NOTE_G5: u16 = 784;
const NOTE_A5: u16 = 880;
const NOTE_B5: u16 = 988;
const NOTE_C6: u16 = 1047;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    let mut buzzer_pin = pins.d6.into_output(&mut pins.ddr).into_pwm(&mut timer0);

    buzzer_pin.enable();

    let duration: u32 = 500_000;


    loop {
        let delay = (1_000_000 / NOTE_C5 as u32) as u16;
        let mut passed = 0;
        while passed < duration {
            buzzer_pin.set_duty(128);
            delay_us(delay);
            buzzer_pin.set_duty(0);
            delay_us(delay);
            passed += 2 * delay as u32;
        }

        let delay = (1_000_000 / NOTE_D5 as u32) as u16;
        let mut passed = 0;
        while passed < duration {
            buzzer_pin.set_duty(128);
            delay_us(delay);
            buzzer_pin.set_duty(0);
            delay_us(delay);
            passed += 2 * delay as u32;
        }


        let delay = (1_000_000 / NOTE_E5 as u32) as u16;
        let mut passed = 0;
        while passed < duration {
            buzzer_pin.set_duty(128);
            delay_us(delay);
            buzzer_pin.set_duty(0);
            delay_us(delay);
            passed += 2 * delay as u32;
        }


        let delay = (1_000_000 / NOTE_F5 as u32) as u16;
        let mut passed = 0;
        while passed < duration {
            buzzer_pin.set_duty(128);
            delay_us(delay);
            buzzer_pin.set_duty(0);
            delay_us(delay);
            passed += 2 * delay as u32;
        }


        let delay = (1_000_000 / NOTE_G5 as u32) as u16;
        let mut passed = 0;
        while passed < duration {
            buzzer_pin.set_duty(128);
            delay_us(delay);
            buzzer_pin.set_duty(0);
            delay_us(delay);
            passed += 2 * delay as u32;
        }


        let delay = (1_000_000 / NOTE_A5 as u32) as u16;
        let mut passed = 0;
        while passed < duration {
            buzzer_pin.set_duty(128);
            delay_us(delay);
            buzzer_pin.set_duty(0);
            delay_us(delay);
            passed += 2 * delay as u32;
        }


        let delay = (1_000_000 / NOTE_B5 as u32) as u16;
        let mut passed = 0;
        while passed < duration {
            buzzer_pin.set_duty(128);
            delay_us(delay);
            buzzer_pin.set_duty(0);
            delay_us(delay);
            passed += 2 * delay as u32;
        }


        let delay = (1_000_000 / NOTE_C6 as u32) as u16;
        let mut passed = 0;
        while passed < duration {
            buzzer_pin.set_duty(128);
            delay_us(delay);
            buzzer_pin.set_duty(0);
            delay_us(delay);
            passed += 2 * delay as u32;
        }

    }
}

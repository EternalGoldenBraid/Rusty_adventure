// See https://www.arduino.cc/en/Reference/PortManipulation for port mapping.


#![no_std]
#![no_main]

use panic_halt as _;
//extern crate panic_halt;
//extern crate panic_abort;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;

use crate::moisture::{SensorUnit};

mod moisture;


fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}



#[arduino_uno::entry]
fn main() -> ! {

    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    // For blinking L
    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        stutter_blink(&mut led, 15);
    }
    // End for blinking L
}

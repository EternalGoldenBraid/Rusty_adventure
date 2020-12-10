// See https://www.arduino.cc/en/Reference/PortManipulation for port mapping.


#![no_std]
#![no_main]

use panic_halt as _;
//extern crate panic_halt;
//extern crate panic_abort;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;

use crate::moisture:{SensorUnit};

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

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        stutter_blink(&mut led, 15);
    }

    // Boards serial write to console
    // screen /dev/tty/<your_tty_here> 57600
    // ls /dev/tty* | grep usb --> get the usb connected
    // 57600 is the baud rate
    let mu serial = arduino_uno::Serial::new(
        perihperals.USART0,

        // the values below correspond to :
        // rx: receive pin (hardwired into the MCU)
        // tx : PD1 is the "hardcoded output"
        // the ownership is moved by writing explicitely input, 
        // output is enforced at compile time,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),

        // other well known baud rates are possible (9600)
        //57600,
        9600,
        );
}

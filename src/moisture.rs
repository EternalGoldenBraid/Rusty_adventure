//! ### The moisture sensor module.

use arduino_uno::prelude::*;
use arduino_uno::hal::port::mode::{Floating};


// Struct for the moisture input instantiated in main due pin requirements.
pub struct SensorUnit {
    pub a0in: arduino_uno::hal::port::portc::PC0<arduino_uno::hal::port::mode::Input<Floating>>,
}


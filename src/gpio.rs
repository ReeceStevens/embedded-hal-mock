//! A mock for General Purpose Input/Output (GPIO) peripherals.
//!
//! ```
//! use embedded_hal_mock::gpio::{GpioPin, GpioState};
//! use embedded_hal::digital::{OutputPin, InputPin};
//!
//! let mut pin = GpioPin::new();
//! pin.set_low();
//! assert_eq!(pin.is_low(), true);
//!
//! pin.set_high();
//! assert_eq!(pin.is_high(), true);
//! ```
#[derive(PartialEq,Debug,Copy,Clone)]
pub enum GpioState {
    High,
    Low,
    Floating,
}

pub struct GpioPin {
    state: GpioState,
}

impl GpioPin {
    pub fn new() -> Self {
        GpioPin { state: GpioState::Floating }
    }
}

// TODO: Support both v1 and v2 OutputPin interfaces
// impl hal::digital::v2::OutputPin for GpioPin {
//     type Error = ();

//     fn set_low(&mut self) -> Result<(), Self::Error> {
//         self.state = GpioState::Low;
//         Ok(())
//     }

//     fn set_high(&mut self) -> Result<(), Self::Error> {
//         self.state = GpioState::High;
//         Ok(())
//     }
// }

impl hal::digital::OutputPin for GpioPin {
    fn set_low(&mut self) {
        self.state = GpioState::Low;
    }

    fn set_high(&mut self) {
        self.state = GpioState::High;
    }
}

impl hal::digital::InputPin for GpioPin {
    fn is_low(&self) -> bool {
        self.state == GpioState::Low
    }

    fn is_high(&self) -> bool {
        self.state == GpioState::High
    }
}

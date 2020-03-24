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

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Infallible;

impl hal::digital::v2::OutputPin for GpioPin {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = GpioState::Low;
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.state = GpioState::High;
        Ok(())
    }
}

impl hal::digital::v2::InputPin for GpioPin {
    type Error = Infallible;

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.state == GpioState::Low)
    }

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.state == GpioState::High)
    }
}

impl hal::digital::v2::ToggleableOutputPin for GpioPin {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        if self.state == GpioState::Low {
            self.state = GpioState::High;
        } else {
            self.state = GpioState::Low;
        }
        Ok(())
    }

}

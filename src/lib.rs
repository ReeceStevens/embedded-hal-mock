//! The goal of this crate is to provide a mock embedded device that implements the embedded-hal
//! interface. This is to allow testing of device drivers and other embedded-hal compatible code
//! without necessarily requiring hardware.
//!
//! Note that this explicitly _does_ require the standard library. Since this is intended to be
//! used in integration tests, it should not be an issue if you call a `[no_std]` library.
extern crate embedded_hal as hal;
extern crate nb;

pub mod gpio;
pub mod adc;
pub mod usart;
pub mod spi;
pub mod i2c;

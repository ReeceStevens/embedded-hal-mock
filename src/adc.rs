//! This module implements a OneShot ADC. It is created by passing in a buffer of input data which
//! will be read by subsequent calls to `.read()`.
//!
//! The input buffer will be continuously read in a loop.
use crate::gpio::GpioPin;
use hal::adc::{Channel, OneShot};

pub struct MockAdc;
impl hal::adc::Channel<MockAdc> for GpioPin {
    type ID = u8;
    fn channel() -> u8 { 0 }
}

pub struct VecAdc<WORD> {
    data: Vec<WORD>,
    head: usize,
}

impl<WORD> VecAdc<WORD> {
    pub fn new(data: Vec<WORD>) -> Self {
        if data.is_empty() {
            panic!("Cannot provide an empty backing store for VecAdc!");
        }
        VecAdc { data, head: 0 }
    }
}

impl<PIN, WORD> OneShot<MockAdc, WORD, PIN> for VecAdc<WORD>
where
    WORD: From<u16> + Copy,
    PIN: Channel<MockAdc, ID=u8>
{
    type Error = ();

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        let word = self.data[self.head];
        self.head = (self.head + 1) % self.data.len();
        Ok(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc_read() {
        let mock_data: Vec<u16> = vec![0, 1, 2, 3, 4];
        let num_reads = mock_data.len() * 2;
        let mut mock_pin = GpioPin::new();
        let mut adc = VecAdc::new(mock_data);

        let mut output = vec![0; num_reads];
        for i in 0..num_reads {
            output[i] = adc.read(&mut mock_pin).unwrap();
        }
        assert_eq!(output, vec![0, 1, 2, 3, 4, 0, 1, 2, 3, 4]);
    }

    #[test]
    #[should_panic]
    fn test_adc_empty_input() {
        let mut adc: VecAdc<u16> = VecAdc::new(vec![]);
    }
}

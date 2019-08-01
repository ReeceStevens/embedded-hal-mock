//! This module implements a OneShot ADC. It is created by passing in a buffer of input data which
//! will be read by subsequent calls to `.read()`.
//!
//! The input buffer will be continuously read in a loop.
use std::iter::Iterator;

use crate::gpio::GpioPin;
use hal::adc::{Channel, OneShot};

pub struct MockAdc;
impl hal::adc::Channel<MockAdc> for GpioPin {
    type ID = u8;
    fn channel() -> u8 { 0 }
}

pub struct VecAdc<I> {
    data: std::iter::Cycle<I>,
}

impl<WORD, I> VecAdc<I>
where
    I: Iterator<Item=WORD> + Clone,
{
    pub fn new<T: IntoIterator<Item=WORD,IntoIter=I>>(data: T) -> Self {
        VecAdc { data: data.into_iter().cycle() }
    }
}

impl<PIN, WORD, I> OneShot<MockAdc, WORD, PIN> for VecAdc<I>
where
    I: Iterator<Item=WORD> + Clone,
    WORD: From<u16> + Copy,
    PIN: Channel<MockAdc, ID=u8>
{
    type Error = ();

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        match self.data.next() {
            Some(v) => Ok(v),
            None => panic!("Backing store has no values"),
        }
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
        let input: Vec<u16> = vec![];
        let mut adc = VecAdc::new(input);
        let mut mock_pin = GpioPin::new();
        adc.read(&mut mock_pin).unwrap();
    }
}

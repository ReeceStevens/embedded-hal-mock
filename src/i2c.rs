//! A mock of an I2C interface, using two `Vec`s as the backing buffers.
use hal::blocking::i2c::{Read, Write};

pub struct VecI2c {
    input_buf: Vec<u8>,
    output_buf: Vec<u8>,
    input_head: usize,
}

impl VecI2c {
    fn read_byte(&mut self) -> u8 {
        let result = self.input_buf[self.input_head];
        self.input_head = (self.input_head + 1) % self.input_buf.len();
        result
    }
}

impl Read for VecI2c {
    type Error = ();

    fn read(&mut self, _addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        for i in 0..buffer.len() {
            buffer[i] = self.read_byte();
        }
        Ok(())
    }
}

impl Write for VecI2c {
    type Error = ();

    fn write(&mut self, _addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.output_buf.extend(bytes);
        Ok(())
    }
}

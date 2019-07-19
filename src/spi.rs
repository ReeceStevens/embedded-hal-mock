use hal::spi::FullDuplex;

pub struct VecSpi {
    input_buf: Vec<u8>,
    output_buf: Vec<u8>,
    input_head: usize,
}

impl VecSpi {
    pub fn new(input_buf: Vec<u8>) -> Self {
        let output_buf = vec![];
        VecSpi {
            input_buf,
            output_buf,
            input_head: 0,
        }
    }
}

impl FullDuplex<u8> for VecSpi {
    type Error = ();

    fn send(&mut self, data: u8) -> nb::Result<(), Self::Error> {
        self.output_buf.push(data);
        Ok(())
    }

    fn read(&mut self) ->nb::Result<u8, Self::Error> {
        let result = self.input_buf[self.input_head];
        self.input_head = (self.input_head + 1) % self.input_buf.len();
        Ok(result)
    }
}

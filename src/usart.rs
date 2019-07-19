pub struct VecUsart {
    input_buf: Vec<u8>,
    output_buf: Vec<u8>,
}

impl VecUsart {
    pub fn new(input_buf: Vec<u8>) -> Self {
        let output_buf = vec![];
        VecUsart { input_buf, output_buf }
    }
}

impl std::fmt::Write for VecUsart {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output_buf.extend(s.as_bytes());
        Ok(())
    }
}

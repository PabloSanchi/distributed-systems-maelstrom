use serde_json::to_writer;
use std::io::Write;

pub struct NewLineWriter<W: Write> {
    inner: W,
}

impl<W: Write> NewLineWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    pub fn write<T: serde::Serialize>(&mut self, value: &T) -> std::io::Result<()> {
        to_writer(&mut self.inner, value)?;
        self.inner.write_all(b"\n")?;
        self.inner.flush()
    }
}

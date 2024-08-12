/// The generic display interface trait
pub trait Interface {
    type Error;

    /// Write bytes to the display
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

/// A serial interface that uses [embedded-io](https://docs.rs/embedded-io/latest/embedded_io/)
pub struct SerialInterface<S: embedded_io::Write> {
    serial: S,
}

impl<S: embedded_io::Write> SerialInterface<S> {
    /// Create a new serial interface
    pub fn new(serial: S) -> Self {
        Self { serial }
    }
}

impl<S: embedded_io::Write> Interface for SerialInterface<S> {
    type Error = S::Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.serial.write_all(data)
    }
}

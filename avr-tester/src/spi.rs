use crate::*;
use std::array;

/// Manages a single SPI.
pub struct Spi<'a> {
    sim: &'a mut AvrSimulator,
    id: usize,
}

impl<'a> Spi<'a> {
    pub(crate) fn new(sim: &'a mut AvrSimulator, id: usize) -> Self {
        Self { sim, id }
    }

    /// Transmits a byte to AVR.
    pub fn send_byte(&mut self, value: u8) {
        self.sim.spi_send(self.id, value);
    }

    /// Retrieves a byte from AVR.
    ///
    /// Returns `None` if there are no more bytes in the buffer, in which case
    /// no more bytes will appear at least until the next [`AvrTester::run()`].
    ///
    /// See also: [`Self::recv()`].
    pub fn recv_byte(&mut self) -> Option<u8> {
        self.sim.spi_recv(self.id)
    }
}

use super::*;
use std::{cell::UnsafeCell, collections::VecDeque, ptr::NonNull};

/// Provides access to simavr's SPI.
pub struct Spi {
    ptr: NonNull<SpiInner>,
    id: char,
}

impl Spi {
    pub fn new(id: char) -> Self {
        let ptr = Box::into_raw(Default::default());

        // Unwrap-safety: `Box::into_raw()` doesn't return null pointers
        let ptr = NonNull::new(ptr).unwrap();

        Self { ptr, id }
    }

    pub fn try_init(self, avr: &mut Avr) -> Option<Self> {
        // Now let's finalize everything by attaching to simavr's IRQs, so that
        // we can easily get notified when AVR sends something through SPI.

        let ioctl = IoCtl::SpiGetIrq { spi: self.id };

        let irq_output = avr
            .io_getirq(ioctl, ffi::SPI_IRQ_OUTPUT)
            .unwrap_or_else(|| {
                panic!(
                    "avr_io_getirq() failed (got a null pointer for SPI{}'s output)",
                    0
                )
            });

        // Safety: All of our callbacks match the expected IRQs
        unsafe {
            avr.irq_register_notify(irq_output, Some(Self::on_output), self.ptr.as_ptr());
        }

        Some(self)
    }

    pub fn flush(&mut self, avr: &mut Avr) {
        let this = self.get();
        let mut irq = None;

        loop {
            // Safety: `&mut self` ensures that while we are working, simavr
            //         won't interrupt us
            let byte = if let Some(byte) = unsafe { this.pop_tx() } {
                byte
            } else {
                break;
            };

            let irq = irq.get_or_insert_with(|| {
                // Unwrap-safety: Since we've come this far, then the chosen AVR
                //                certainly supports SPI and there's no
                //                reason for that instruction to panic
                avr.io_getirq(IoCtl::SpiGetIrq{ spi: self.id }, ffi::SPI_IRQ_INPUT)
                    .unwrap()
            });

            // Safety: `SPI_IRQ_INPUT` is meant to send data through SPI and
            //         supports being raised with any byte-parameter
            unsafe {
                avr.raise_irq(*irq, byte as _);
            }
        }
    }

    pub fn send(&mut self, byte: u8) {
        // Safety: `&mut self` ensures that while we are working, simavr won't
        //         interrupt us
        unsafe {
            self.get().push_tx(byte);
        }
    }

    pub fn recv(&mut self) -> Option<u8> {
        // Safety: `&mut self` ensures that while we are working, simavr won't
        //         interrupt us
        unsafe { self.get().pop_rx() }
    }

    pub fn peek(&mut self) -> Option<u8> {
        // Safety: `&mut self` ensures that while we are working, simavr won't
        //         interrupt us
        unsafe { self.get().peek_rx() }
    }

    fn get(&self) -> &SpiInner {
        // Safety: `self.ptr` is alive as long as `self`
        unsafe { self.ptr.as_ref() }
    }

    unsafe extern "C" fn on_output(_: NonNull<ffi::avr_irq_t>, value: u32, uart: *mut SpiInner) {
        SpiInner::from_ptr(uart).push_rx(value as u8);
    }
}

impl Drop for Spi {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}

#[derive(Debug)]
struct SpiInner {
    rx: UnsafeCell<VecDeque<u8>>,
    tx: UnsafeCell<VecDeque<u8>>,
}

impl SpiInner {
    const RX_BUFFER_MAX_BYTES: usize = 128 * 1024;

    unsafe fn from_ptr<'a>(uart: *mut SpiInner) -> &'a Self {
        &*(uart as *mut Self)
    }

    /// Called by simavr when the AVR transmits a byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::pop_rx()`] or
    /// [`Self::peek_rx()`].
    unsafe fn push_rx(&self, value: u8) {
        let rx = &mut *self.rx.get();

        if rx.len() < Self::RX_BUFFER_MAX_BYTES {
            rx.push_back(value);
        }
    }

    /// Called by AvrTester when user wants to retrieve a single byte from the
    /// buffer.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::push_rx()`] or
    /// [`Self::peek_rx()`].
    unsafe fn pop_rx(&self) -> Option<u8> {
        (*self.rx.get()).pop_front()
    }

    /// Called by AvrTester when user wants to peek at the currently-pending
    /// byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::push_rx()`] or
    /// [`Self::pop_rx()`].
    unsafe fn peek_rx(&self) -> Option<u8> {
        (*self.rx.get()).front().copied()
    }

    /// Called by AvrTester when user wants to transmit a byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::pop_tx()`].
    unsafe fn push_tx(&self, value: u8) {
        (*self.tx.get()).push_back(value);
    }

    /// Called by simavr when the AVR is ready to retrieve a byte.
    ///
    /// # Safety
    ///
    /// Cannot be called simultaneously with [`Self::push_tx()`].
    unsafe fn pop_tx(&self) -> Option<u8> {
        (*self.tx.get()).pop_front()
    }
}

impl Default for SpiInner {
    fn default() -> Self {
        Self {
            rx: Default::default(),
            tx: Default::default(),
        }
    }
}

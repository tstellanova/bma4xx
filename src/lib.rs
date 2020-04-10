#![no_std]

mod definitions;

use crate::definitions::{Command, Register, BMA421_DEVICE_ID, BMA423_DEFAULT_DEVICE_ID};
use core::convert::TryFrom;
use core::convert::TryInto;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub use accelerometer::error::{Error, ErrorKind};

pub struct BMA4xx<I2C> {
    port: I2C,
    address: u8,
}

impl<I2C, CommE> BMA4xx<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = CommE>,
    CommE: core::fmt::Debug,
{
    fn new(port: I2C, address: u8) -> Self {
        Self { port, address }
    }

    pub fn default_bma421(port: I2C) -> Self {
        Self::new(port, BMA421_DEVICE_ID)
    }

    pub fn default_bma423(port: I2C) -> Self {
        Self::new(port, BMA423_DEFAULT_DEVICE_ID)
    }

    /// Configures the sensor with a default configuration
    pub fn setup(&mut self, delay: &mut impl DelayUs<u32>) -> Result<(), Error<E>> {
        let device_id = self.read_register(Register::CHIP_ID)?;
        match device_id {
            BMA421_DEVICE_ID
            | BMA421_ALT_DEVICE_ID
            | BMA423_DEFAULT_DEVICE_ID
            | BMA423_ALT_DEVICE_ID => {}
            _ => return Err(Error::new(ErrorKind::Device)),
        }

        self.soft_reset(delay)?;

        //TODO setup configuration such as bandwidth

        Ok(())
    }

    pub fn soft_reset(&mut self, delay: &mut impl DelayUs<u32>) -> Result<(), Error<E>> {
        self.write_register(Register::CMD, Command::SOFT_RESET)?;
        delay.delay_us(50_000);
        Ok(())
    }
}

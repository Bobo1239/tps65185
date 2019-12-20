#![no_std]

mod register;

use core::fmt::Debug;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use enum_repr::EnumRepr;

use crate::register::Register;

pub const I2C_SLAVE_ADDRESS: u8 = 0x68;

#[derive(Debug, Clone)]
pub enum Error<I2cErr> {
    I2c(I2cErr),
    UnknownEnumValue,
}

impl<I2cErr> From<I2cErr> for Error<I2cErr> {
    fn from(err: I2cErr) -> Error<I2cErr> {
        Error::I2c(err)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tps65185<I2c> {
    i2c: I2c,
}

impl<I2c, I2cErr> Tps65185<I2c>
where
    I2c: WriteRead<Error = I2cErr> + Write<Error = I2cErr>,
    I2cErr: Debug,
{
    pub fn new(i2c: I2c) -> Tps65185<I2c> {
        Tps65185 { i2c }
    }

    pub fn device_version(&mut self) -> Result<DeviceVersion, Error<I2cErr>> {
        let value = self.read_register(Register::REVID)?;
        DeviceVersion::from_repr(value).ok_or(Error::UnknownEnumValue)
    }

    fn write_register(&mut self, register: Register, value: u8) -> Result<(), Error<I2cErr>> {
        self.i2c
            .write(I2C_SLAVE_ADDRESS, &[register.addr(), value])?;
        Ok(())
    }

    fn read_register(&mut self, register: Register) -> Result<u8, Error<I2cErr>> {
        let mut buffer = [0];
        self.i2c
            .write_read(I2C_SLAVE_ADDRESS, &[register.addr()], &mut buffer)?;
        Ok(buffer[0])
    }
}

#[EnumRepr(type = "u8")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceVersion {
    Tps65185_1p0 = 0x45,
    Tps65185_1p1 = 0x55,
    Tps65185_1p2 = 0x65,
    Tps651851_1p0 = 0x66,
}

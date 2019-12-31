#![no_std]

mod register;

use core::fmt::Debug;
use core::marker::PhantomData;

use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use embedded_hal::digital::v2::OutputPin;

use crate::register::{DeviceVersion, Register};

pub const I2C_SLAVE_ADDRESS: u8 = 0x68;

/// TODO: doc; Dataasheet specifies 1.8ms delay until I2C is ready
pub const WAKEUP_I2C_DELAY_US: u16 = 1_800;

#[derive(Debug, Clone)]
pub enum Error<I2cErr, PinErr> {
    I2c(I2cErr),
    Pin(PinErr),
    UnknownEnumValue,
}

// impl<I2cErr, PinErr> From<I2cErr> for Error<I2cErr, PinErr> {
//     fn from(err: I2cErr) -> Error<I2cErr, PinErr> {
//         Error::I2c(err)
//     }
// }

impl<I2cErr, PinErr> From<PinErr> for Error<I2cErr, PinErr> {
    fn from(err: PinErr) -> Error<I2cErr, PinErr> {
        Error::Pin(err)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tps65185<M: Mode, I2c, Wakeup, Pwrup, VcomCtrl> {
    i2c: I2c,
    pin_wakeup: Wakeup,
    pin_pwrup: Pwrup,
    pin_vcom_ctrl: VcomCtrl,
    mode: PhantomData<M>,
}

pub trait Mode {}
pub enum Sleep {}
impl Mode for Sleep {}
// Merged into one mode as we only care about I2c availability
pub enum StandbyActive {}
impl Mode for StandbyActive {}

impl<I2c, Wakeup, Pwrup, VcomCtrl, I2cErr, PinErr> Tps65185<Sleep, I2c, Wakeup, Pwrup, VcomCtrl>
where
    I2c: WriteRead<Error = I2cErr> + Write<Error = I2cErr>,
    Wakeup: OutputPin<Error = PinErr>,
    Pwrup: OutputPin<Error = PinErr>,
    VcomCtrl: OutputPin<Error = PinErr>,
{
    pub fn new(
        i2c: I2c,
        mut pin_wakeup: Wakeup,
        pin_pwrup: Pwrup,
        mut pin_vcom_ctrl: VcomCtrl,
    ) -> Result<Self, Error<I2cErr, PinErr>> {
        pin_wakeup.set_low()?;
        pin_vcom_ctrl.set_high()?; // TODO: Decide if this is right
        pin_wakeup.set_low()?;
        Ok(Tps65185 {
            i2c,
            pin_wakeup,
            pin_pwrup,
            pin_vcom_ctrl,
            mode: PhantomData,
        })
    }

    // TODO: Doc optional delay
    #[allow(clippy::type_complexity)]
    pub fn wakeup<D: DelayUs<u16>>(
        mut self,
        delay: Option<&mut D>,
    ) -> Result<Tps65185<StandbyActive, I2c, Wakeup, Pwrup, VcomCtrl>, Error<I2cErr, PinErr>> {
        self.pin_wakeup.set_high()?;
        if let Some(delay) = delay {
            delay.delay_us(WAKEUP_I2C_DELAY_US);
        }
        Ok(Tps65185 {
            i2c: self.i2c,
            pin_wakeup: self.pin_wakeup,
            pin_pwrup: self.pin_pwrup,
            pin_vcom_ctrl: self.pin_vcom_ctrl,
            mode: PhantomData,
        })
    }

    // TODO: Method to release pins again
}

impl<I2c, Wakeup, Pwrup, VcomCtrl, I2cErr, PinErr>
    Tps65185<StandbyActive, I2c, Wakeup, Pwrup, VcomCtrl>
where
    I2c: WriteRead<Error = I2cErr> + Write<Error = I2cErr>,
    Wakeup: OutputPin<Error = PinErr>,
    Pwrup: OutputPin<Error = PinErr>,
    VcomCtrl: OutputPin<Error = PinErr>,
{
    pub fn device_version(&mut self) -> Result<DeviceVersion, Error<I2cErr, PinErr>> {
        let value = self.read_register(Register::REVID)?;
        DeviceVersion::from_repr(value).ok_or(Error::UnknownEnumValue)
    }

    pub fn enable(&mut self) -> Result<(), Error<I2cErr, PinErr>> {
        // TODO: Not sure if this is ideal...
        // Ensure that we get a rising edge
        self.pin_pwrup.set_low()?;
        Ok(self.pin_pwrup.set_high()?)
    }

    pub fn disable(&mut self) -> Result<(), Error<I2cErr, PinErr>> {
        // Ensure that we get a falling edge
        self.pin_pwrup.set_high()?;
        Ok(self.pin_pwrup.set_low()?)
    }

    #[allow(clippy::type_complexity)]
    pub fn sleep(
        mut self,
    ) -> Result<Tps65185<Sleep, I2c, Wakeup, Pwrup, VcomCtrl>, Error<I2cErr, PinErr>> {
        self.pin_wakeup.set_low()?;
        Ok(Tps65185 {
            i2c: self.i2c,
            pin_wakeup: self.pin_wakeup,
            pin_pwrup: self.pin_pwrup,
            pin_vcom_ctrl: self.pin_vcom_ctrl,
            mode: PhantomData,
        })
    }

    fn write_register(
        &mut self,
        register: Register,
        value: u8,
    ) -> Result<(), Error<I2cErr, PinErr>> {
        self.i2c
            .write(I2C_SLAVE_ADDRESS, &[register.addr(), value])
            .map_err(Error::I2c)?;
        Ok(())
    }

    fn read_register(&mut self, register: Register) -> Result<u8, Error<I2cErr, PinErr>> {
        let mut buffer = [0];
        self.i2c
            .write_read(I2C_SLAVE_ADDRESS, &[register.addr()], &mut buffer)
            .map_err(Error::I2c)?;
        Ok(buffer[0])
    }
}

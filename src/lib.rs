#![no_std]

extern crate embedded_hal as hal;

use core::mem;

use hal::blocking::i2c::{Write, WriteRead};

pub trait Register {
    fn addr(&self) -> u8;
}

pub trait SerialWrite<Mode, R>
where
    Self : Write,
    R : Register
{

    fn write_u8(&mut self, addr: u8, reg: R, data: u8) -> Result<(), Self::Error> {
        self.write(addr, &[reg.addr(), data])
    }

    fn write_le_u16(&mut self, addr: u8, reg: R, data: u16) -> Result<(), Self::Error>;
    fn write_be_u16(&mut self, addr: u8, reg: R, data: u16) -> Result<(), Self::Error>;
    fn write_le_u32(&mut self, addr: u8, reg: R, data: u32) -> Result<(), Self::Error>;
    fn write_be_u32(&mut self, addr: u8, reg: R, data: u32) -> Result<(), Self::Error>;
}

pub trait SerialRead<Mode, R>
where
    Self : WriteRead,
    R : Register
{

    fn read_u8(&mut self, addr: u8, reg: R) -> Result<u8, Self::Error> {
        let mut buffer: [u8; 1] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }
    fn read_le_u16(&mut self, addr: u8, reg: R) -> Result<u16, Self::Error>;
    fn read_be_u16(&mut self, addr: u8, reg: R) -> Result<u16, Self::Error>;
    fn read_le_u24(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error>;
    fn read_be_u24(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error>;
    fn read_le_u32(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error>;
    fn read_be_u32(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error>;
}

pub mod autoincrement;
pub mod noincrement;

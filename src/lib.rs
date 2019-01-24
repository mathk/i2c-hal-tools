#![no_std]

extern crate embedded_hal as hal;


pub trait Register {
    fn addr(&self) -> u8;
}

pub trait SerialWrite<Mode, R>
where
    R : Register
{
    type Error;

    fn write_u8(&mut self, addr: u8, reg: R, data: u8) -> Result<(), Self::Error>;
    fn write_le_u16(&mut self, addr: u8, reg: R, data: u16) -> Result<(), Self::Error>;
    fn write_be_u16(&mut self, addr: u8, reg: R, data: u16) -> Result<(), Self::Error>;
    fn write_le_u32(&mut self, addr: u8, reg: R, data: u32) -> Result<(), Self::Error>;
    fn write_be_u32(&mut self, addr: u8, reg: R, data: u32) -> Result<(), Self::Error>;
}

pub trait SerialRead<Mode, R>
where
    R : Register
{
    type Error;

    fn read_u8(&mut self, addr: u8, reg: R) -> Result<u8, Self::Error>;
    fn read_le_u16(&mut self, addr: u8, reg: R) -> Result<u16, Self::Error>;
    fn read_be_u16(&mut self, addr: u8, reg: R) -> Result<u16, Self::Error>;
    fn read_le_u32(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error>;
    fn read_be_u32(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error>;
}

pub mod autoincrement;

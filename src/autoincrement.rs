
extern crate cast;
extern crate embedded_hal as hal;

use core::mem;

use {SerialWrite, SerialRead, Register};
use hal::blocking::i2c::{Write, WriteRead};
use self::cast::{u16, u32};

pub struct AutoIncrementI2c;

impl<T, R, E> SerialWrite<AutoIncrementI2c, R> for T
where
    T : Write<Error = E>,
    R : Register,
{

    type Error = E;

    fn write_u8(&mut self, addr: u8, reg: R, data: u8) -> Result<(), E> {
        self.write(addr, &[reg.addr(), data])
    }

    fn write_le_u16(&mut self, addr: u8, reg: R, data: u16) -> Result<(), E> {
        self.write(addr, &[reg.addr(), (data & 0xFF) as u8, (data >> 8) as u8])
    }

    fn write_be_u16(&mut self, addr: u8, reg: R, data: u16) -> Result<(), E> {
        self.write(addr, &[reg.addr(), (data >> 8) as u8, (data & 0xFF) as u8])
    }

    fn write_le_u32(&mut self, addr: u8, reg: R, data: u32) -> Result<(), E> {
        self.write(addr, &[reg.addr(), ((data >> 24) & 0xFF) as u8, ((data >> 16) & 0xFF) as u8, ((data >> 8) & 0xFF) as u8, (data & 0xFF) as u8])
    }

    fn write_be_u32(&mut self, addr: u8, reg: R, data: u32) -> Result<(), E> {
        self.write(addr, &[reg.addr(), (data & 0xFF) as u8, ((data >> 8) & 0xFF) as u8, ((data >> 16) & 0xFF) as u8, ((data >> 24) & 0xFF) as u8])
    }
}

impl<T, R, E> SerialRead<AutoIncrementI2c, R> for T
where
    T : WriteRead<Error = E>,
    R : Register,
{
    type Error = E;

    fn read_u8(&mut self, addr: u8, reg: R) -> Result<u8, E> {
        let mut buffer: [u8; 1] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }

    fn read_le_u16(&mut self, addr: u8, reg: R) -> Result<u16, E> {
        let mut buffer: [u8; 2] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer)?;
        Ok((u16(buffer[1]) << 8) & u16(buffer[0]))
    }

    fn read_be_u16(&mut self, addr: u8, reg: R) -> Result<u16, E> {
        let mut buffer: [u8; 2] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer)?;
        Ok((u16(buffer[0]) << 8) & u16(buffer[1]))
    }

    fn read_le_u32(&mut self, addr: u8, reg: R) -> Result<u32, E> {
        let mut buffer: [u8; 4] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer)?;
        Ok((u32(buffer[3]) << 24) & (u32(buffer[2]) << 16) & (u32(buffer[1]) << 8) & u32(buffer[0]))
    }

    fn read_be_u32(&mut self, addr: u8, reg: R) -> Result<u32, E> {
        let mut buffer: [u8; 4] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer)?;
        Ok((u32(buffer[0]) << 24) & (u32(buffer[1]) << 16) & (u32(buffer[2]) << 8) & u32(buffer[3]))
    }
}


extern crate cast;
extern crate embedded_hal as hal;

use core::mem;

use {SerialWrite, SerialRead, Register};
use hal::blocking::i2c::{Write, WriteRead};
use self::cast::{u16, u32};

pub struct NoIncrementI2c;


impl<T, R> SerialWrite<NoIncrementI2c, R> for T
where
    T : Write,
    R : Register,
{

    fn write_le_u16(&mut self, addr: u8, reg: R, data: u16) -> Result<(), Self::Error> {
        self.write(addr, &[reg.addr(), (data & 0xFF) as u8])?;
        self.write(addr, &[reg.addr() + 1, (data >> 8) as u8])
    }

    fn write_be_u16(&mut self, addr: u8, reg: R, data: u16) -> Result<(), Self::Error> {
        self.write(addr, &[reg.addr(), (data >> 8) as u8])?;
        self.write(addr, &[reg.addr() + 1, (data & 0xFF) as u8])
    }

    fn write_le_u32(&mut self, addr: u8, reg: R, data: u32) -> Result<(), Self::Error> {
        self.write(addr, &[reg.addr(), ((data >> 24) & 0xFF) as u8])?;
        self.write(addr, &[reg.addr() + 1, ((data >> 16) & 0xFF) as u8])?;
        self.write(addr, &[reg.addr() + 2, ((data >> 8) & 0xFF) as u8])?;
        self.write(addr, &[reg.addr() + 3, (data & 0xFF) as u8])
    }

    fn write_be_u32(&mut self, addr: u8, reg: R, data: u32) -> Result<(), Self::Error> {
        self.write(addr, &[reg.addr(), (data & 0xFF) as u8])?;
        self.write(addr, &[reg.addr() + 1, ((data >> 8) & 0xFF) as u8])?;
        self.write(addr, &[reg.addr() + 2, ((data >> 16) & 0xFF) as u8])?;
        self.write(addr, &[reg.addr() + 3, ((data >> 24) & 0xFF) as u8])
    }
}

impl<T, R> SerialRead<NoIncrementI2c, R> for T
where
    T : WriteRead,
    R : Register,
{
    fn read_le_u16(&mut self, addr: u8, reg: R) -> Result<u16, Self::Error> {
        let mut buffer: [u8; 2] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer[..1])?;
        self.write_read(addr, &[reg.addr() + 1], &mut buffer[1..])?;
        Ok((u16(buffer[1]) << 8) | u16(buffer[0]))
    }

    fn read_be_u16(&mut self, addr: u8, reg: R) -> Result<u16, Self::Error> {
        let mut buffer: [u8; 2] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer[..1])?;
        self.write_read(addr, &[reg.addr() + 1], &mut buffer[1..])?;
        Ok((u16(buffer[0]) << 8) | u16(buffer[1]))
    }

    fn read_le_u24(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error> {
        let mut buffer: [u8; 3] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer[0..1])?;
        self.write_read(addr, &[reg.addr() + 1], &mut buffer[1..2])?;
        self.write_read(addr, &[reg.addr() + 2], &mut buffer[2..3])?;
        Ok((u32(buffer[2]) << 16) | (u32(buffer[1]) << 8) | u32(buffer[0]))
    }

    fn read_be_u24(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error> {
        let mut buffer: [u8; 3] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer[0..1])?;
        self.write_read(addr, &[reg.addr() + 1], &mut buffer[1..2])?;
        self.write_read(addr, &[reg.addr() + 2], &mut buffer[2..3])?;
        Ok((u32(buffer[0]) << 16) | (u32(buffer[1]) << 8) | u32(buffer[2]))
    }

    fn read_le_u32(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error> {
        let mut buffer: [u8; 4] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer[0..1])?;
        self.write_read(addr, &[reg.addr() + 1], &mut buffer[1..2])?;
        self.write_read(addr, &[reg.addr() + 2], &mut buffer[2..3])?;
        self.write_read(addr, &[reg.addr() + 3], &mut buffer[3..])?;
        Ok((u32(buffer[3]) << 24) | (u32(buffer[2]) << 16) | (u32(buffer[1]) << 8) | u32(buffer[0]))
    }

    fn read_be_u32(&mut self, addr: u8, reg: R) -> Result<u32, Self::Error> {
        let mut buffer: [u8; 4] = unsafe { mem::uninitialized() };
        self.write_read(addr, &[reg.addr()], &mut buffer[0..1])?;
        self.write_read(addr, &[reg.addr() + 1], &mut buffer[1..2])?;
        self.write_read(addr, &[reg.addr() + 2], &mut buffer[2..3])?;
        self.write_read(addr, &[reg.addr() + 3], &mut buffer[3..])?;
        Ok((u32(buffer[0]) << 24) | (u32(buffer[1]) << 16) | (u32(buffer[2]) << 8) | u32(buffer[3]))
    }
}

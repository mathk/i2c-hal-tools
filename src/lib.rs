
extern crate embedded-hal as hal;


pub trait SerialWrite<Mode> {
    type Error;

    fn write_u8(&mut self, addr: u8, data: u8) -> Result<(), Error>;
    fn write_le_u16(&mut self, addr: u8, data: u16) -> Result<(), Error>;
    fn write_be_u16(&mut self, addr: u8, data: u16) -> Result<(), Error>;
    fn write_le_u32(&mut self, addr: u8, data: u32) -> Result<(), Error>;
    fn write_be_u32(&mut self, addr: u8, data: u32) -> Result<(), Error>;
    fn read_u8(&mut self, addr: u8) -> Result<u8, Error>;
    fn read_le_u16(&mut self, addr: u8) -> Result<u16, Error>;
    fn read_be_u16(&mut self, addr: u8) -> Result<u16, Error>;
    fn read_le_u32(&mut self, addr: u8) -> Result<u16, Error>;
    fn read_be_u32(&mut self, addr: u8) -> Result<u16, Error>;
}

pub mod autoincrement;


extern crate cast;
extern crate embedded_hal as hal;

use core::mem;

use hal::blocking::i2c::{Write, WriteRead};
use cast::u16;

trait AutoIncrementI2c {
}

impl<>

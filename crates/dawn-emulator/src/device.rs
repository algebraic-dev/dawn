#[derive(Copy, Clone)]
pub struct Address(pub u32);

impl Address {
    pub fn inc(&self, num: u32) -> Address {
        Address(self.0 + num)
    }
}

impl From<Address> for usize {
    fn from(value: Address) -> Self {
        value.0 as usize
    }
}

impl Into<u32> for Address {
    fn into(self) -> u32 {
        self.0
    }
}

pub trait IODevice {
    const SIZE: usize;

    fn read_32(&mut self, address: Address) -> u32 {
        if address.0 as usize > Self::SIZE - 4 {
            0
        } else {
            (self.read_8(address) as u32) << 24
                | (self.read_8(address.inc(1)) as u32) << 16
                | (self.read_8(address.inc(2)) as u32) << 8
                | (self.read_8(address.inc(3)) as u32)
        }
    }

    fn read_16(&mut self, address: Address) -> u16 {
        if address.0 as usize > Self::SIZE - 2 {
            return 0;
        } else {
            (self.read_8(address) as u16) << 8 | (self.read_8(address.inc(1)) as u16)
        }
    }

    fn write_32(&mut self, address: Address, value: u32) {
        if !address.0 as usize > Self::SIZE - 4 {
            self.write_8(address, (value >> 24) as u8 & 0xFF);
            self.write_8(address.inc(1), (value >> 16) as u8 & 0xFF);
            self.write_8(address.inc(2), (value >> 8) as u8 & 0xFF);
            self.write_8(address.inc(3), (value) as u8 & 0xFF);
        }
    }

    fn write_16(&mut self, address: Address, value: u16) {
        if !address.0 as usize > Self::SIZE - 2 {
            self.write_8(address, (value >> 8) as u8 & 0xFF);
            self.write_8(address.inc(1), (value) as u8 & 0xFF);
        }
    }

    fn read_8(&mut self, address: Address) -> u8;
    fn write_8(&mut self, address: Address, value: u8);
}

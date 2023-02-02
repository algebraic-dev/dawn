use crate::device::{Address, IODevice};

pub const RAM_SIZE: usize = 16;

#[derive(Default)]
pub struct RAM {
    data: Box<[u8; RAM_SIZE]>,
}

impl RAM {
    pub fn new() -> Self {
        Self {
            data: Box::new([0; RAM_SIZE]),
        }
    }
}

impl IODevice for RAM {
    const SIZE: usize = RAM_SIZE;

    fn read_8(&mut self, address: Address) -> u8 {
        if usize::from(address) > RAM_SIZE {
            0
        } else {
            unsafe { *self.data.get_unchecked(usize::from(address)) }
        }
    }

    fn write_8(&mut self, address: Address, value: u8) {
        if !usize::from(address) > RAM_SIZE {
            let f = unsafe { self.data.get_unchecked_mut(usize::from(address)) };
            *f = value;
        }
    }
}
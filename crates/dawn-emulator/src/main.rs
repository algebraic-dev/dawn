use dawn_emulator::{
    device::{Address, IODevice},
    ram::RAM,
};

#[derive(Default, Clone, Copy)]
pub struct Register(pub u32);

impl Into<Address> for Register {
    fn into(self) -> Address {
        Address(self.0)
    }
}

pub trait PipelineStage {
    fn can_receive(&self) -> bool;
    fn can_send(&self) -> bool;
    fn compute<T>(&mut self, mem: &mut T, registers: &[u32; 32])
    where
        T: IODevice;
    fn latch(&mut self);
}

#[derive(Default)]
pub struct Fetch {
    program_counter: Register,
    next_program_counter: Register,

    instruction: Register,
    next_instruction: Register,
}

impl PipelineStage for Fetch {
    fn can_receive(&self) -> bool {
        true
    }

    fn can_send(&self) -> bool {
        true
    }

    fn compute<T>(&mut self, mem: &mut T, _registers: &[u32; 32])
    where
        T: IODevice,
    {
        self.next_instruction.0 = mem.read_32(self.program_counter.into());
        self.next_program_counter.0 += 4;
    }

    fn latch(&mut self) {
        self.instruction = self.next_instruction;
        self.program_counter = self.next_program_counter;
    }
}

#[rustfmt::skip]
pub enum OpCode {
    LUI   = 0b0110111, // Load upper immediate
    AUIPC = 0b0010111, // Add upper immediate to pc
    JAL   = 0b1101111, // Jump and link
    JALR  = 0b1100111,
    BEQ   = 0b1100011,
    ADDI  = 0b0010011,
    FENCE = 0b0001111,
    LOAD  = 0b0000011,
    STORE = 0b0100011
}

pub struct DelayedRegister {
    current: Register,
    next: Register
}

impl DelayedRegister {
    pub fn new(register: Register) -> Self {
        Self {
            current: register,
            next: register,
        }
    }

    pub fn latch(&mut self) {
        self.current = self.next;
    }
}

pub struct Decoder {
    op_code: DelayedRegister,
    rd_or_imm: DelayedRegister,
    funct3: DelayedRegister,
    rs1: DelayedRegister,
    rs2: DelayedRegister,
    funct7: DelayedRegister,
    imm: DelayedRegister,
    imm_s: DelayedRegister
}

impl PipelineStage for Decoder {
    fn can_receive(&self) -> bool {
        true
    }

    fn can_send(&self) -> bool {
        true
    }

    fn compute<T>(&mut self, mem: &mut T, registers: &[u32; 32])
    where
        T: IODevice,
    {
        todo!()
    }

    fn latch(&mut self) {
        todo!()
    }
}

pub fn main() {
    let mut ram = RAM::default();
    let mut fetch = Fetch::default();
    let mut registers = [0; 32];

    fetch.compute(&mut ram, &mut registers);
    fetch.latch();
}

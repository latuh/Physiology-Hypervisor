pub enum Register {
    Reg1,
    Reg2,
    Reg3,
    Reg4,
    Reg5,
    Reg6,
    Reg7,
    Reg8
}

pub struct Registers {
    pub reg1: [u8; 1],
    pub reg2: [u8; 1],
    pub reg3: [u8; 1],
    pub reg4: [u8; 1],
    pub reg5: [u8; 1],
    pub reg6: [u8; 1],
    pub reg7: [u8; 1],
    pub reg8: [u8; 1]
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            reg1: [0; 1],
            reg2: [0; 1],
            reg3: [0; 1],
            reg4: [0; 1],
            reg5: [0; 1],
            reg6: [0; 1],
            reg7: [0; 1],
            reg8: [0; 1]
        }
    }
}
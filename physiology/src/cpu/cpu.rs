use super::cache::Cache;
use super::registers::Registers;

pub struct CPU {
    pub cache: Cache,
    pub registry: Registers
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            cache: Cache::new(),
            registry: Registers::new()
        }
    }
}
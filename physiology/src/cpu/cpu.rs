use super::cache::Cache;
use super::registry::Registry;

pub struct CPU {
    pub cache: Cache,
    pub registry: Registry
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            cache: Cache::new(),
            registry: Registry::new()
        }
    }
}
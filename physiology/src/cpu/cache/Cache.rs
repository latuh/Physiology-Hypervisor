pub struct Cache {
    pub l1: [u8; 256],
    pub l2: [u8; 512],
    pub l3: [u8; 1024]
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            l1: [0; 256],
            l2: [0; 512],
            l3: [0; 1024]
        }
    }
}
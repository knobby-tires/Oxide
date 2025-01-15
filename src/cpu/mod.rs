#[derive(Debug)]
pub struct CPU {
    pub registers: [u64; 16],  // 16 general-purpose registers
    pub program_counter: u64,
    pub running: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            program_counter: 0,
            running: false,
        }
    }
}
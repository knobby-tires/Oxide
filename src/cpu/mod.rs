#[derive(Debug)]
pub struct CPU {
    pub registers: [u64; 16],  // 16 general purpose registers
    pub program_counter: u64,
    pub running: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    LoadImm { reg: usize, value: u64 },    // load immediate value into register
    Move { dest: usize, src: usize },      // move between registers
    Add { dest: usize, src: usize },       // add registers
    Jump {address: u64 },                  // jump to address
    Halt,                                  // stop the execution
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            program_counter: 0,
            running: false,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            Instruction::LoadImm { reg, value} => {
                if reg < self.registers.len() {
                    self.registers[reg] = value;
                    self.program_counter += 1;
                    Ok(())
                } else {
                    Err(format!("Invalid register: {}", reg))
                }
            },
            Instruction::Move { dest, src} => {
                if dest < self.registers.len() && src < self.registers.len() {
                    self.registers[dest] = self.registers[src];
                    self.program_counter += 1;
                    Ok(())
                } else {
                    Err(format!("Invalid register: dest={}, src={}", dest, src))
                }
            },
            Instruction::Add {dest, src} => {
                if dest < self.registers.len() && src < self.registers.len() {
                    self.registers[dest] = self.registers[dest].wrapping_add(self.registers[src]);
                    self.program_counter += 1;
                    Ok(())
                } else {
                    Err(format!("Invalid register: dest={}, src={}", dest, src))
                }
            },
            Instruction::Jump { address } => {
                self.program_counter = address;
                Ok(())
            },
            Instruction::Halt => {
                self.running = false;
                Ok(())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_immediate() {
        let mut cpu = CPU::new();
        assert!(cpu.execute(Instruction::LoadImm { reg: 0, value: 42 }).is_ok());
        assert_eq!(cpu.registers[0], 42);
        assert_eq!(cpu.program_counter, 1);
        
        // test invalid register
        assert!(cpu.execute(Instruction::LoadImm { reg: 16, value: 42 }).is_err());
    }

    #[test]
    fn test_move_between_registers() {
        let mut cpu = CPU::new();
        // first load a value
        cpu.execute(Instruction::LoadImm { reg: 0, value: 42 }).unwrap();
        // then move it
        assert!(cpu.execute(Instruction::Move { dest: 1, src: 0 }).is_ok());
        assert_eq!(cpu.registers[1], 42);
        
        // test invalid registers
        assert!(cpu.execute(Instruction::Move { dest: 16, src: 0 }).is_err());
        assert!(cpu.execute(Instruction::Move { dest: 0, src: 16 }).is_err());
    }

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        // load two values
        cpu.execute(Instruction::LoadImm { reg: 0, value: 42 }).unwrap();
        cpu.execute(Instruction::LoadImm { reg: 1, value: 58 }).unwrap();
        // Add them
        assert!(cpu.execute(Instruction::Add { dest: 0, src: 1 }).is_ok());
        assert_eq!(cpu.registers[0], 100);
        
        // test overflow (wrapping add)
        cpu.execute(Instruction::LoadImm { reg: 0, value: u64::MAX }).unwrap();
        cpu.execute(Instruction::LoadImm { reg: 1, value: 1 }).unwrap();
        cpu.execute(Instruction::Add { dest: 0, src: 1 }).unwrap();
        assert_eq!(cpu.registers[0], 0);  // Should wrap around to 0
    }

    #[test]
    fn test_jump() {
        let mut cpu = CPU::new();
        assert!(cpu.execute(Instruction::Jump { address: 42 }).is_ok());
        assert_eq!(cpu.program_counter, 42);
    }

    #[test]
    fn test_halt() {
        let mut cpu = CPU::new();
        cpu.running = true;
        assert!(cpu.execute(Instruction::Halt).is_ok());
        assert!(!cpu.running);
    }

    #[test]
    fn test_program_sequence() {
        let mut cpu = CPU::new();
        // small program: load two numbers, add them, move result
        let program = [
            Instruction::LoadImm { reg: 0, value: 30 },
            Instruction::LoadImm { reg: 1, value: 12 },
            Instruction::Add { dest: 0, src: 1 },
            Instruction::Move { dest: 2, src: 0 },
            Instruction::Halt,
        ];

        for instruction in program {
            assert!(cpu.execute(instruction).is_ok());
        }

        assert_eq!(cpu.registers[2], 42);  // final result
        assert!(!cpu.running);  // should be halted
    }
}
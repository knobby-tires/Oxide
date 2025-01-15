mod cpu;
use cpu::CPU;
use cpu::Instruction;

// all possible states a vm can be in 
#[derive(Debug)] 
enum VMState{
    Created,
    Running,
    Stopped,
    Error,
}

// basic config stuff for VM
struct VMConfig {
    memory_size: usize, // size in bytes
    num_cpus: u32,      // # of virtual cpu's
}

// The main vm structure
struct VirtualMachine {
    state: VMState,
    config: VMConfig,
    pub memory: Vec<u8>, // simple memory impelementation for groundwork
    pub cpus: Vec<CPU>,
}

impl VirtualMachine {
    // constructer for new vm
    fn new(config: VMConfig) -> Self {
        let mut cpus = Vec::with_capacity(config.num_cpus as usize);
        for _ in 0..config.num_cpus {
            cpus.push(CPU::new());
        }
        VirtualMachine {
        state: VMState::Created,
        memory: vec![0; config.memory_size],
        cpus,
        config,
        }
    }


    // start: start vm's
    fn start(&mut self) {
        println!("Starting VM with {} bytes of memory", self.config.memory_size);
        self.state = VMState::Running;
    }

    // stop: stop vm's
    fn stop(&mut self) -> Result<(), String> {
        match self.state {
            VMState::Running => {
                self.state = VMState::Stopped;
                Ok(())
            }
            _ => {
                self.state = VMState::Error;
                Err("Cannot stop: VM is not running".to_string())
            }
        }
    }

    // write_byte: method to write a byte to memory with bounds checkign
    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), String> {
        if address >= self.memory.len() {
            return Err(format!("Memory address {} out of bounds", address));
        }
        self.memory[address] = value;
        Ok(())
    }

    // read_byte: method to read a byte from memory with bounds checkign
    fn read_byte(&self, address: usize) -> Result<u8, String> {
        if address >= self.memory.len() {
            return Err(format!("Memory address {} out of bounds", address));
        }
        Ok(self.memory[address])
    }

    fn load_program(&mut self, program: &[Instruction], start_address: u64) -> Result<(), String> {
        // for now just store program for 1st cpu
        if self.cpus.is_empty() {
            return Err("No CPUs available".to_string());
        }

        let cpu = &mut self.cpus[0];
        cpu.program_counter = start_address;
        cpu.running = true;

        // basic execution loop
        for instruction in program {
            cpu.execute(*instruction)?;
            if !cpu.running {
                break; // stop when hit a halt instruction
            }
        }
        Ok(())
    }
}

fn main() {
    // create a simple vm with 1mb of memory and 1 cpu
    let config = VMConfig {
        memory_size: 1024 * 1024, // 1mb
        num_cpus : 1,
    };

    let mut vm = VirtualMachine::new(config);
    vm.start();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_operations() {
        let config = VMConfig {
            memory_size: 1024,
            num_cpus: 1,
        };
        let mut vm = VirtualMachine::new(config);

        // Test write and read
        assert!(vm.write_byte(0x100, 42).is_ok());
        assert_eq!(vm.read_byte(0x100).unwrap(), 42);

        // Test out of bounds
        assert!(vm.write_byte(2000, 42).is_err());
        assert!(vm.read_byte(2000).is_err());
    }

    #[test]
    fn test_vm_state() {
        let config = VMConfig {
            memory_size: 1024,
            num_cpus: 1,
        };
        let mut vm = VirtualMachine::new(config);
        assert!(matches!(vm.state, VMState::Created));
        
        vm.start();
        assert!(matches!(vm.state, VMState::Running));
    }

    #[test]
    fn test_cpu_initialization() {
        let config = VMConfig {
            memory_size: 1024,
            num_cpus: 2,
        };
        let vm = VirtualMachine::new(config);
        
        assert_eq!(vm.cpus.len(), 2);
        // Check that CPUs are in their initial state
        for cpu in &vm.cpus {
            assert_eq!(cpu.program_counter, 0);
            assert!(!cpu.running);
        }
    }

    #[test]
    fn test_program_execution() {
        let config = VMConfig {
            memory_size: 1024,
            num_cpus: 1,
        };
        let mut vm = VirtualMachine::new(config);
        
        let program = [
            Instruction::LoadImm { reg: 0, value: 5 },
            Instruction::LoadImm { reg: 1, value: 7 },
            Instruction::Add { dest: 2, src: 0 },
            Instruction::Add { dest: 2, src: 1 },
            Instruction::Halt,
        ];

        assert!(vm.load_program(&program, 0).is_ok());
        assert_eq!(vm.cpus[0].registers[2], 12);  // 5 + 7
    }
}
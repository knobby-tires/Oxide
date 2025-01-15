// src/main.rs

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
    memory: Vec<u8>, // simple memory impelementation for groundwork
}

impl VirtualMachine {
    // constructer for new vm
    fn new(config: VMConfig) -> Self {
        VirtualMachine {
        state: VMState::Created,
        memory: vec![0; config.memory_size],
        config,
        }
    }


    // basic start method
    fn start(&mut self) {
        println!("Starting VM with {} bytes of memory", self.config.memory_size);
        self.state = VMState::Running;
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
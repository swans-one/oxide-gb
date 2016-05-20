use super::cpu;
use super::interconnect;

#[derive(Debug)]
pub struct Gb {
    cpu: cpu::Cpu,
}

impl Gb {
    pub fn new() -> Gb {
        Gb {
            cpu: cpu::Cpu::new(interconnect::Interconnect::new()),
        }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}

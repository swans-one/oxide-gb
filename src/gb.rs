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

    pub fn power_on_reset(&mut self) {
        self.cpu.power_on_reset();
    }
}

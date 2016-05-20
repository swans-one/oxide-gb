use super::cpu;

pub struct Gb {
    cpu: cpu::Cpu,
}

impl Gb {
    pub fn new() -> Gb {
        Gb {
            cpu: cpu::Cpu::new(),
        }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}

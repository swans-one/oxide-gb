use super::interconnect;

#[derive(Debug)]
pub struct Cpu {
    register: CpuRegisters,
    interconnect: interconnect::Interconnect,
}

#[derive(Default, Debug)]
#[allow(dead_code)] // TODO: remove when things are used
struct CpuRegisters {
    acc_a: u8,
    acc_b: u8,
    acc_d: u8,
    acc_h: u8,
    flag_f: u8,
    flag_c: u8,
    flag_e: u8,
    flag_l: u8,
    pc: u16,
    sp: u16,
    int_i: u8,
    ref_r: u8,
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu {
        Cpu {
            register: CpuRegisters::default(),
            interconnect: interconnect,
        }
    }

    // TODO: Different interface
    pub fn run(&mut self) {
        let pc = self.register.pc;
        self.interconnect.read_word(pc);
    }

    pub fn power_on_reset(&mut self) {
        self.register.pc = 0x100;
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
        self.interconnect.load_cartridge(rom);
    }
}

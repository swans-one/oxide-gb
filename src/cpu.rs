use super::interconnect;

#[derive(Default, Debug)]
pub struct Cpu {
    registers: CpuRegisters,
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
    pub fn new() -> Cpu {
        Cpu::default()
    }

    // TODO: Different interface
    pub fn run(&mut self) {

    }
}

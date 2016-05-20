#[derive(Default, Debug)]
#[allow(dead_code)] // TODO: remove when things are used
pub struct Cpu {
    reg_acc_a: u8,
    reg_acc_b: u8,
    reg_acc_d: u8,
    reg_acc_h: u8,
    reg_flag_f: u8,
    reg_flag_c: u8,
    reg_flag_e: u8,
    reg_flag_l: u8,
    reg_pc: u16,
    reg_sp: u16,
    reg_int_i: u8,
    reg_ref_r: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }
}

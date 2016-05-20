use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Default, Debug)]
#[allow(dead_code)] // TODO: remove when things are used
struct Cpu {
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
    fn new() -> Cpu {
        Cpu::default()
    }
}

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();

    let rom_bytes = load_bin(rom_file_name);

    let cpu = Cpu::new();

    println!("{:?}", rom_bytes[1]);
    println!("{:#?}", cpu)
}

fn load_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut bin_file = fs::File::open(path).unwrap();
    let mut bin_bytes: Vec<u8> = Vec::new();
    bin_file.read_to_end(&mut bin_bytes).unwrap();
    bin_bytes
}

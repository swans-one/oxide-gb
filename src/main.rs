use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

mod cpu;
mod gb;
mod interconnect;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();

    let rom_bytes = load_bin(rom_file_name);

    let mut gb = gb::Gb::new();
    gb.load_cartridge(rom_bytes);
    gb.power_on_reset();
    gb.run();

    // We moved this for now, may make a copy later
    // println!("{:?}", rom_bytes[1]);
    println!("{:#?}", gb)
}

fn load_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut bin_file = fs::File::open(path).unwrap();
    let mut bin_bytes: Vec<u8> = Vec::new();
    bin_file.read_to_end(&mut bin_bytes).unwrap();
    bin_bytes
}

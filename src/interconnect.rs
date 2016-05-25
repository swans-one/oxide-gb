use std::fmt;

const ADDR_SPACE_SIZE: usize = 16 * 1024;

pub struct Interconnect {
    // TODO: name this address space?
    ram: Box<[u8; ADDR_SPACE_SIZE]>,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        Interconnect::default()
    }

    pub fn read_word(&self, addr: u16) -> u8 {
        // TODO: Test this
        if 0x0000 <= addr && addr <= 0x7fff {
            self.ram[addr as usize]
        } else if 0xC000 <= addr && addr <= 0xDFFF {
            self.ram[addr as usize]
        } else {
            panic!("Unable to read from address {:#x}", addr)
        }
    }
}


impl Default for Interconnect {
    fn default() -> Interconnect {
        Interconnect {
            ram: Box::new([0; ADDR_SPACE_SIZE])
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ram...")
    }
}

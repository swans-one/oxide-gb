use std::fmt;

const ADDR_SPACE_SIZE: usize = 16 * 1024;

pub struct Interconnect {
    // TODO: name this address space?
    ram: Box<[u8; ADDR_SPACE_SIZE]>,
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

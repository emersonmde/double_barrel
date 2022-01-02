use crate::io::InputOutputModule;

/*
0x0000	0000 0000 0000 0000     RAM Start
0x3FFF  0011 1111 1111 1111     RAM End

0x6000  0110 0000 0000 0000     IO Start
0x600F  0110 0000 0000 FFFF     IO End

0x7fff	0111 1111 1111 1111
0x8000	1000 0000 0000 0000	    EEPROM Start

0xfffc  1111 1111 1111 1100	    Code start low byte
0xfffd  1111 1111 1111 1101	    Code start end byte
0xffff	1111 1111 1111 1111	    EEPROM End
 */
pub struct MemoryManagementUnit {
    // TODO: Find a way to change to arrays
    ram: Vec<u8>,
    // io: Vec<u8>,
    io: InputOutputModule,
    rom: Vec<u8>,

    segments: Vec<(u16, u16, fn(&Self, u16) -> u8, fn(&mut Self, u16, u8))>,
    // rom: [u8; 0x8000]
}

impl MemoryManagementUnit {
    pub fn new(rom: Vec<u8>) -> Self {
        if rom.len() != 0x8000 {
            panic!("Invalid ROM size")
        }
        let mut segments: Vec<(u16, u16, fn(&Self, u16) -> u8, fn(&mut Self, u16, u8))> = Vec::new();
        segments.push((0x0000, 0x3FFF, Self::read_ram, Self::write_ram)); // 00
        segments.push((0x6000, 0x600F, Self::read_io, Self::write_io)); // 011
        segments.push((0x8000, 0xFFFF, Self::read_rom, Self::write_rom)); // 1
        MemoryManagementUnit {
            ram: vec![0; 0x4000],
            // io: vec![0; 0xF],
            io: InputOutputModule::new(),
            rom,
            segments,
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn write_ram(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    fn read_io(&self, _: u16) -> u8 {
        panic!("readio_io not implemented")
        // let offset = addr & 0xF;
        // self.io[offset as usize]
    }

    fn write_io(&mut self, addr: u16, value: u8) {
        // let offset = addr & 0xF;
        // self.io[offset as usize] = value;
        self.io.write(addr, value);
    }

    fn read_rom(&self, addr: u16) -> u8 {
        let offset = addr & (0x8000 - 1);
        self.rom[offset as usize]
    }

    fn write_rom(&mut self, _: u16, _: u8) {
        panic!("Unable to write to ROM")
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        // TODO: prefix tree
        for (start, end, read, _) in self.segments.iter() {
            if addr >= *start && addr <= *end {
                return (read)(self, addr);
            }
        }
        panic!("Segmentation Fault");
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        // TODO: Check bounds
        ((self.read_u8(addr + 1) as u16) << 8) | (self.read_u8(addr) as u16)
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        // TODO: prefix tree
        for (start, end, _, write) in self.segments.iter() {
            if addr >= *start && addr <= *end {
                (write)(self, addr, value);
                return;
            }
        }
        panic!("Segmentation Fault");
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        self.write_u8(addr, (value & 0xFF) as u8);
        self.write_u8(addr + 1, (value >> 8) as u8);
    }

    pub fn print_stack(&self) {
        println!("stack: {:?}", &self.ram[0x0100..=0x01FF])
    }
}

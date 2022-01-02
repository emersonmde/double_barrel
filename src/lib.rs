use std::error::Error;
use byteorder::{ByteOrder, LittleEndian};

use cpu::CPU;
use mmu::MemoryManagementUnit;

mod mmu;
mod cpu;
mod io;
mod lcd;

fn print_eeprom(eeprom: &[u8], length: usize) {
    for b in eeprom[0..length].iter() {
        println!("{:#04x}", b);
    }
}

pub struct DoubleBarrel {
    file_name: String,
    cpu: CPU,
}

impl DoubleBarrel {
    pub fn new(file_name: String) -> Option<Self> {
        let rom = std::fs::read(&file_name).unwrap();
        print_eeprom(&rom, 20);
        if rom.len() != 0x8000 {
            println!("Invalid ROM size: ROM must be {} bytes, found {}", 0x8000, rom.len());
            return None;
        }
        let cpu = CPU::new(rom);
        Some(DoubleBarrel { file_name, cpu })
    }

    pub fn start(&mut self) {
        println!("\nStart Program\n");
        loop {
            self.cpu.tick();
        }
    }
}
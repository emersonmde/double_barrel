// use std::time::Duration;
// use tokio::time;

// async fn renew_sitemap_each_day() {
//     let mut interval_day = time::interval(Duration::from_secs(1));
//     for _ in 0..5 {
//         let now = interval_day.tick().await;
//         println!("Renew sitemaps for each day. (Time now = {:?})", now);
//     }
// }

use byteorder::{ByteOrder, LittleEndian};

use cpu::CPU;
use mmu::MemoryManagementUnit;

mod mmu;
mod cpu;
mod io;
mod lcd;

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

fn print_eeprom(eeprom: &[u8], length: usize) {
    for b in eeprom[0..length].iter() {
        println!("{:#04x}", b);
    }
}

fn main() {
    let rom = std::fs::read("a.out").unwrap();
    if rom.len() != 0x8000 {
        panic!("Invalid ROM size: ROM must be {} bytes, found {}", 0x8000, rom.len());
    }
    print_eeprom(&rom, 20);

    println!("\n");
    let code_start_addr = 0x7ffc;
    let code_start = LittleEndian::read_u16(&rom[code_start_addr..(code_start_addr + 2)]);

    println!("Start of code: {:#04x}", code_start);

    if code_start & 0x8000 == 0 {
        println!("ERROR: Start of code must be in ROM (Seg fault)");
        std::process::exit(1);
    }

    // let mut mmu = MemoryManagementUnit::new(rom);
    let mut cpu = CPU::new(rom);
    cpu.print_registers();
    println!("\nStart Program\n");
    loop {
        cpu.tick();
    }
}

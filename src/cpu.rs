use std::fmt;

use crate::MemoryManagementUnit;

const OPCODE_SIZE: usize = 0xFF;
const STACK_SIZE: usize = 0xFF;

pub struct Registers {
    a: u8,
    x: u8,
    stack: u8,
    pc: u16,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(a: {:#04x}, x: {:#04x}, stack: {:#04x}, pc: {:#04x})", self.a, self.x, self.stack, self.pc)
    }
}

pub struct CPU {
    pub registers: Registers,
    pub mmu: MemoryManagementUnit,
    pub opcodes: [fn(&mut CPU); OPCODE_SIZE],
}


impl CPU {
    pub fn new(rom: Vec<u8>) -> Self {
        let mmu = MemoryManagementUnit::new(rom);
        let registers = Registers { a: 0, x: 0, stack: 0, pc: mmu.read_u16(0xFFFC) };
        let opcodes: [fn(&mut CPU); OPCODE_SIZE] = [Self::not_implemented; OPCODE_SIZE];
        let mut cpu = CPU { registers, mmu, opcodes };
        cpu.register_opcodes();
        cpu
    }

    pub fn print_registers(&self) {
        println!("Registers: {}", self.registers)
    }

    fn register_opcodes(&mut self) {
        self.opcodes[0xEA] = CPU::nop;
        self.opcodes[0xA2] = CPU::ldx;
        self.opcodes[0x9A] = CPU::txs;
        self.opcodes[0xA9] = CPU::lda;
        self.opcodes[0x8D] = CPU::sta;
        self.opcodes[0x20] = CPU::jsr;
        self.opcodes[0x60] = CPU::rts;
        self.opcodes[0x4c] = CPU::jmp;
    }

    pub fn pop_u8_operand(&mut self) -> u8 {
        let operand = self.mmu.read_u8(self.registers.pc);
        self.registers.pc += 1;
        operand
    }

    pub fn pop_u16_operand(&mut self) -> u16 {
        let operand = self.mmu.read_u16(self.registers.pc);
        self.registers.pc += 2;
        operand
    }

    pub fn get_instruction(&mut self) -> u8 {
        self.pop_u8_operand()
    }

    pub fn tick(&mut self) {
        if self.registers.pc == 0x8062 {
            panic!("END")
        }
        (self.opcodes[self.get_instruction() as usize])(self);
    }

    // Opcodes
    fn not_implemented(&mut self) {
        panic!("{:#04x} at {:#04x} not Implmented", self.mmu.read_u8(self.registers.pc - 1), self.registers.pc - 1);
    }

    fn nop(&mut self) {
        println!("nop");
    }

    fn lda(&mut self) {
        let operand = self.pop_u8_operand();
        println!("lda #{:#04x}", operand);
        self.registers.a = operand;
    }

    fn sta(&mut self) {
        let operand = self.pop_u16_operand();
        println!("sta #{:#06X}", operand);
        self.mmu.write_u8(operand, self.registers.a);
    }

    fn ldx(&mut self) {
        let operand = self.pop_u8_operand();
        println!("ldx #{:#04x}", operand);
        self.registers.x = operand;
    }

    fn txs(&mut self) {
        println!("tsx");
        self.registers.stack = self.registers.x;
    }

    fn jsr(&mut self) {
        let operand = self.pop_u16_operand();
        // write self.registers.pc - 1 to stack - 1 and stack
        // stack - 2
        // pc = op
        let stack_addr: u16 = 0x0100 | (self.registers.stack - 1) as u16;
        // println!("JSR WRITE {:#04x} TO {:#04x}", operand, stack_addr);
        self.mmu.write_u16(stack_addr, self.registers.pc);
        self.registers.stack -= 2 % STACK_SIZE as u8;
        self.registers.pc = operand;
        println!("jsr {:#04x}", operand);
        // println!("STACK {:#04x}", self.registers.stack);
    }

    fn rts(&mut self) {
        let stack_addr: u16 = 0x0100 | ((self.registers.stack as u16 + 1) & 0xFF);
        let pc = self.mmu.read_u16(stack_addr);
        // println!("rts READ {:#04x} FROM {:#04x}", pc, stack_addr);
        self.registers.pc = pc;
        self.registers.stack = self.registers.stack.checked_add(2).unwrap();
        println!("rts");
        self.registers.stack = self.registers.x;
    }

    fn jmp(&mut self) {
        let operand = self.pop_u16_operand();
        self.registers.pc = operand;
        println!("jmp {:#06x}", operand);
    }
}

use std::fs::File;
use std::io::prelude::*;

enum CpuInstruction {
    IllegalInstruction(u16),
}

#[allow(dead_code)]
struct Cpu {
    pc: usize,
    index_reg: u16,
    registers: Vec<u8>,
    graphics: Vec<u8>,
    stack: Vec<u16>,
    sp: u16,
    memory: Vec<u8>,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            pc: 0x200,
            index_reg: 0,
            registers: vec![0; 16],
            graphics: vec![0; 64*32],
            stack: vec![0; 16],
            sp: 0,
            memory: vec![0; 4096],
        }
    }

    fn load_rom(&mut self, path: &str) -> std::io::Result<()> {
        let mut rom = File::open(path)?;
        let mut buffer = vec![];
        rom.read_to_end(&mut buffer)?;
        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }
        Ok(())
    }

    fn run(&mut self) -> Result<(), CpuInstruction> {
        loop {
            self.step()?;
        }
    }

    fn step(&mut self) -> Result<(), CpuInstruction> {
        // let next_pc = self.pc + 2;

        let opcode_high = self.memory[self.pc];
        let opcode_low = self.memory[self.pc+1];
        let opcode = (opcode_high as u16) << 8 | opcode_low as u16;
        match opcode {
            _ => {
                return Err(CpuInstruction::IllegalInstruction(opcode))
            },
        }

        // self.pc = next_pc;
        // Ok(())
    }
}

fn main() {
    let mut cpu = Cpu::new();
    match cpu.load_rom("./games/pong2.c8") {
        Ok(()) => {},
        Err(err) => { panic!("Error: {}", err) },
    }
    match cpu.run() {
        Ok(()) => {},
        Err(CpuInstruction::IllegalInstruction(opcode)) => { panic!("Illegal CPU instruction: {:x}", opcode) },
    }
}

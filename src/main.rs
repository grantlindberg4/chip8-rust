extern crate rand;

mod cpu;
mod core;

fn main() {
    let mut cpu = cpu::Cpu::new();
    match cpu.load_rom("./roms/pong2.c8") {
        Ok(()) => {},
        Err(err) => { panic!("Error: {}", err) },
    }
    match cpu.run() {
        Ok(()) => {},
        Err(cpu::CpuError::IllegalInstruction(opcode)) => {
            panic!("Illegal CPU instruction: {:x}", opcode)
        },
    }
}

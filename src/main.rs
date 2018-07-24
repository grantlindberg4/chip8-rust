extern crate rand;
extern crate sdl2;

use std::env;

mod cpu;
mod core;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu.load_fontset();

    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let rom = &args[1];
    match cpu.load_rom(rom) {
        Ok(()) => {},
        Err(err) => { panic!("Error: {}", err) },
    }

    let sdl_context = sdl2::init().unwrap();
    match cpu.run(&sdl_context) {
        Ok(()) => {},
        Err(cpu::CpuError::IllegalInstruction(opcode)) => {
            panic!("Illegal CPU instruction: {:x}", opcode)
        },
    }
}

extern crate rand;
extern crate sdl2;

mod cpu;
mod core;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu.load_fontset();
    match cpu.load_rom("./roms/pong2.c8") {
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

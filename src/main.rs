extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut core = core::Core::new(&sdl_context);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    core.handle_key_down(&mut cpu, keycode)
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    core.handle_key_up(&mut cpu, keycode)
                },
                _ => {},
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
        match cpu.step() {
            Ok(()) => {},
            Err(cpu::CpuError::IllegalInstruction(opcode)) => {
                panic!("Illegal CPU instruction: {:x}", opcode)
            },
        }
        if cpu.draw_screen {
            core.draw(&mut cpu);
        }
    }
}

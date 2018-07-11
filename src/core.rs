use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::Sdl;

use cpu::Cpu;

const SCALE: u32 = 12;

pub static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

#[derive(Clone, Copy)]
pub enum Pixel {
    Black,
    White,
}

#[derive(Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
}

pub struct Core {
    canvas: Canvas<Window>,
}

impl Core {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Chip8-Rust", 64*SCALE, 32*SCALE)
                                    .position_centered()
                                    .opengl()
                                    .build()
                                    .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Core {
            canvas,
        }
    }

    pub fn draw(&mut self, cpu: &mut Cpu) {
        for i in 0..64*32 {
            let curr_pixel = cpu.display[i];
            let x = (i % 64) * SCALE as usize;
            let y = (i / 64) * SCALE as usize;

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            match curr_pixel {
                Pixel::White => {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255))
                },
                _ => {},
            }
            let _ = self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));
        }
        self.canvas.present();
    }

    pub fn handle_key_down(&mut self, cpu: &mut Cpu, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => { cpu.keys[0x1] = KeyState::Pressed },
            Keycode::Num2 => { cpu.keys[0x2] = KeyState::Pressed },
            Keycode::Num3 => { cpu.keys[0x3] = KeyState::Pressed },
            Keycode::Num4 => { cpu.keys[0xC] = KeyState::Pressed },
            Keycode::Q => { cpu.keys[0x4] = KeyState::Pressed },
            Keycode::W => { cpu.keys[0x5] = KeyState::Pressed },
            Keycode::E => { cpu.keys[0x6] = KeyState::Pressed },
            Keycode::R => { cpu.keys[0xD] = KeyState::Pressed },
            Keycode::A => { cpu.keys[0x7] = KeyState::Pressed },
            Keycode::S => { cpu.keys[0x8] = KeyState::Pressed },
            Keycode::D => { cpu.keys[0x9] = KeyState::Pressed },
            Keycode::F => { cpu.keys[0xE] = KeyState::Pressed },
            Keycode::Z => { cpu.keys[0xA] = KeyState::Pressed },
            Keycode::X => { cpu.keys[0x0] = KeyState::Pressed },
            Keycode::C => { cpu.keys[0xB] = KeyState::Pressed },
            Keycode::V => { cpu.keys[0xF] = KeyState::Pressed },
            _ => {},
        }
    }

    pub fn handle_key_up(&mut self, cpu: &mut Cpu, keycode: Keycode) {
        match keycode {
            Keycode::Num1 => { cpu.keys[0x1] = KeyState::Released },
            Keycode::Num2 => { cpu.keys[0x2] = KeyState::Released },
            Keycode::Num3 => { cpu.keys[0x3] = KeyState::Released },
            Keycode::Num4 => { cpu.keys[0xC] = KeyState::Released },
            Keycode::Q => { cpu.keys[0x4] = KeyState::Released },
            Keycode::W => { cpu.keys[0x5] = KeyState::Released },
            Keycode::E => { cpu.keys[0x6] = KeyState::Released },
            Keycode::R => { cpu.keys[0xD] = KeyState::Released },
            Keycode::A => { cpu.keys[0x7] = KeyState::Released },
            Keycode::S => { cpu.keys[0x8] = KeyState::Released },
            Keycode::D => { cpu.keys[0x9] = KeyState::Released },
            Keycode::F => { cpu.keys[0xE] = KeyState::Released },
            Keycode::Z => { cpu.keys[0xA] = KeyState::Released },
            Keycode::X => { cpu.keys[0x0] = KeyState::Released },
            Keycode::C => { cpu.keys[0xB] = KeyState::Released },
            Keycode::V => { cpu.keys[0xF] = KeyState::Released },
            _ => {},
        }
    }
}

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};
use sdl2::Sdl;

use cpu::Cpu;

pub const DISPLAY_HEIGHT: u32 = 32;
pub const DISPLAY_WIDTH: u32 = 64;
const SCALE_FACTOR: u32 = 12;

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

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            }
            else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Core {
    canvas: Canvas<Window>,
    audio_device: AudioDevice<SquareWave>,
}

impl Core {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(
            "Chip8-Rust",
            DISPLAY_WIDTH*SCALE_FACTOR,
            DISPLAY_HEIGHT*SCALE_FACTOR
        )
                                    .position_centered()
                                    .opengl()
                                    .build()
                                    .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44000),
            channels: Some(1),
            samples: None,
        };
        let audio_device = audio_subsystem.open_playback(
            None,
            &desired_spec,
            |spec| {
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            }
        ).unwrap();

        Core {
            canvas,
            audio_device,
        }
    }

    pub fn draw(&mut self, cpu: &mut Cpu) {
        for i in 0..(DISPLAY_WIDTH*DISPLAY_HEIGHT) as usize {
            let curr_pixel = cpu.display[i];
            let x = (i % DISPLAY_WIDTH as usize) * SCALE_FACTOR as usize;
            let y = (i / DISPLAY_WIDTH as usize) * SCALE_FACTOR as usize;

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            match curr_pixel {
                Pixel::White => {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255))
                },
                _ => {},
            }
            let _ = self.canvas.fill_rect(Rect::new(
                x as i32,
                y as i32,
                SCALE_FACTOR,
                SCALE_FACTOR
            ));
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

    pub fn play_sound(&mut self) {
        self.audio_device.resume();
    }

    pub fn stop_sound(&mut self) {
        self.audio_device.pause();
    }
}

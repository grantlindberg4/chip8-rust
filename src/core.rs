use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::Sdl;

use cpu::Cpu;

const SCALE: u32 = 12;

#[derive(Clone, Copy)]
pub enum Pixel {
    Black,
    White,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
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
}

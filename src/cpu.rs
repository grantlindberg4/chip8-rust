use std;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use rand::{thread_rng, Rng};
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::Sdl;

use core;
use core::Core;

pub enum Opcode {
    CallRCAProgram(u16),
    ClearDisplay,
    ReturnFromSubroutine,
    JumpToAddr(u16),
    CallSubroutine(u16),
    SkipIfRegisterSet { addr: u16, value: u16 },
    SkipIfRegisterNotSet { addr: u16, value: u16 },
    SkipIfRegistersEqual { first: u16, second: u16 },
    SetRegister { addr: u16, value: u16 },
    AddToRegister { addr: u16, value: u16 },
    AssignRegister { first: u16, second: u16 },
    AssignRegisterBitwiseOr { first: u16, second: u16 },
    AssignRegisterBitwiseAnd { first: u16, second: u16 },
    AssignRegisterBitwiseXor { first: u16, second: u16 },
    AddRegisters { first: u16, second: u16 },
    SubtractRegisters { first: u16, second: u16 },
    AssignRegisterBitshiftRight { first: u16, second: u16 },
    SubtractFirstRegister { first: u16, second: u16 },
    AssignRegistersBitshiftLeft { first: u16, second: u16 },
    SkipIfRegistersNotEqual { first: u16, second: u16 },
    SetIndexRegister(u16),
    SetProgramCounter(u16),
    AssignRandomValue { addr: u16, value: u16 },
    Draw { x: u16, y: u16, height: u16 },
    SkipIfKeyPressed(u16),
    SkipIfKeyNotPressed(u16),
    AssignToDelayTime(u16),
    AssignOnKeyPress(u16),
    SetDelayTimer(u16),
    SetSoundTimer(u16),
    AddToIndexRegister(u16),
    SetIndexRegisterToSpriteAddr(u16),
    SetBCD(u16),
    DumpRegister(u16),
    LoadRegister(u16),
}

pub enum CpuError {
    IllegalInstruction(u16),
}

pub struct Cpu {
    pc: u16,
    index_reg: u16,
    registers: Vec<u8>,
    pub keys: Vec<core::KeyState>,
    delay_timer: u8,
    sound_timer: u8,
    counter: u8,
    pub display: Vec<core::Pixel>,
    pub draw_screen: bool,
    stack: Vec<u16>,
    sp: u16,
    memory: Vec<u8>,
}

impl Cpu {
    /// Creates a new Cpu
    /// The program counter starts at 0x200
    /// Vectors are used instead of arrays for flexibility
    pub fn new() -> Self {
        Cpu {
            pc: 0x200,
            index_reg: 0,
            registers: vec![0; 16],
            keys: vec![core::KeyState::Released; 16],
            delay_timer: 0,
            sound_timer: 0,
            counter: 10,
            display: vec![
                core::Pixel::Black;
                (core::DISPLAY_WIDTH*core::DISPLAY_HEIGHT) as usize
            ],
            draw_screen: false,
            stack: vec![0; 16],
            sp: 0,
            memory: vec![0; 4096],
        }
    }

    /// Loads the Chip-8 fontset into Cpu memory
    pub fn load_fontset(&mut self) {
        let fontset: [u8; 80] = [
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
        for i in 0..80 {
            self.memory[i] = fontset[i];
        }
    }

    /// Loads the contents of the selected Chip-8 rom into Cpu memory
    pub fn load_rom(&mut self, path: &str) -> std::io::Result<()> {
        let mut rom = File::open(path)?;
        let mut buffer = vec![];
        rom.read_to_end(&mut buffer)?;
        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }
        Ok(())
    }

    /// Checks an opcode to see if it exists
    /// If the opcode is legal, the corresponding instruction is returned for
    /// the Cpu to execute
    /// If the opcode is illegal, an error is returned and the program is
    /// aborted
    ///
    /// # Arguments
    ///
    /// * `opcode` - An unsigned 16-bit integer that is to be verified by
    /// the function
    ///
    /// # Example 1: Legal opcode
    ///
    /// ```
    /// let cpu = new Cpu::new();
    /// match cpu.decode(0x00E0) {
    ///     Ok(cpu::Opcode::ClearDisplay) => {
    ///         // 0x00E0 is a legal opcode and corresponds to this function
    ///         // Therefore this code will be executed
    ///     },
    ///     Err(cpu::CpuError::IllegalInstruction(opcode)) => {
    ///         panic!("Illegal CPU instruction: {:x}", opcode)
    ///    },
    /// }
    /// ```
    ///
    /// # Example 2: Illegal opcode
    ///
    /// ```
    /// let cpu = new Cpu::new();
    /// match cpu.decode(0xE100) {
    ///     Ok(cpu::Opcode::ClearDisplay) => {},
    ///     Err(cpu::CpuError::IllegalInstruction(opcode)) => {
    ///         // 0xE100 is an illegal opcode
    ///         // Therefore the program will terminate
    ///         panic!("Illegal CPU instruction: {:x}", opcode)
    ///    },
    /// }
    /// ```
    ///
    /// NOTE: Other opcodes not included in match statement for brevity
    /// NOTE: This example uses the function as a public method, but the
    /// function is in fact private and should be used within the context
    /// of the Cpu object
    fn decode(&mut self, opcode: u16) -> Result<Opcode, CpuError> {
        match opcode {
            0x00E0 => Ok(Opcode::ClearDisplay),
            0x00EE => Ok(Opcode::ReturnFromSubroutine),
            0x0000...0x0FFF => Ok(Opcode::CallRCAProgram(opcode & 0x0FFF)),
            0x1000...0x1FFF => Ok(Opcode::JumpToAddr(opcode & 0x0FFF)),
            0x2000...0x2FFF => Ok(Opcode::CallSubroutine(opcode & 0x0FFF)),
            0x3000...0x3FFF => {
                Ok(Opcode::SkipIfRegisterSet {
                    addr: (opcode & 0x0F00) >> 8,
                    value: opcode & 0x00FF,
                })
            },
            0x4000...0x4FFF => {
                Ok(Opcode::SkipIfRegisterNotSet {
                    addr: (opcode & 0x0F00) >> 8,
                    value: opcode & 0x00FF,
                })
            },
            0x5000...0x5FFF => {
                Ok(Opcode::SkipIfRegistersEqual {
                    first: (opcode & 0x0F00) >> 8,
                    second: (opcode & 0x00F0) >> 4,
                })
            }
            0x6000...0x6FFF => {
                Ok(Opcode::SetRegister {
                    addr: (opcode & 0x0F00) >> 8,
                    value: opcode & 0x00FF,
                })
            },
            0x7000...0x7FFF => {
                Ok(Opcode::AddToRegister {
                    addr: (opcode & 0x0F00) >> 8,
                    value: opcode & 0x00FF,
                })
            },
            0x8000...0x8FFF => {
                match opcode & 0x000F {
                    0x0000 => {
                        Ok(Opcode::AssignRegister {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    0x0001 => {
                        Ok(Opcode::AssignRegisterBitwiseOr {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    0x0002 => {
                        Ok(Opcode::AssignRegisterBitwiseAnd {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    0x0003 => {
                        Ok(Opcode::AssignRegisterBitwiseXor {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    0x0004 => {
                        Ok(Opcode::AddRegisters {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    0x0005 => {
                        Ok(Opcode::SubtractRegisters {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    0x0006 => {
                        Ok(Opcode::AssignRegisterBitshiftRight {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    0x0007 => {
                        Ok(Opcode::SubtractFirstRegister {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    0x000E => {
                        Ok(Opcode::AssignRegistersBitshiftLeft {
                            first: (opcode & 0x0F00) >> 8,
                            second: (opcode & 0x00F0) >> 4,
                        })
                    },
                    _ => Err(CpuError::IllegalInstruction(opcode)),
                }
            },
            0x9000...0x9FFF => {
                Ok(Opcode::SkipIfRegistersNotEqual {
                    first: (opcode & 0x0F00) >> 8,
                    second: (opcode & 0x00F0) >> 4,
                })
            },
            0xA000...0xAFFF => Ok(Opcode::SetIndexRegister(opcode & 0x0FFF)),
            0xB000...0xBFFF => Ok(Opcode::SetProgramCounter(opcode & 0x0FFF)),
            0xC000...0xCFFF => {
                Ok(Opcode::AssignRandomValue {
                    addr: (opcode & 0x0F00) >> 8,
                    value: opcode & 0x00FF,
                })
            },
            0xD000...0xDFFF => {
                Ok(Opcode::Draw {
                    x: (opcode & 0x0F00) >> 8,
                    y: (opcode & 0x00F0) >> 4,
                    height: opcode & 0x000F,
                })
            },
            0xE000...0xEFFF => {
                match opcode & 0x00FF {
                    0x009E => {
                        Ok(Opcode::SkipIfKeyPressed((opcode & 0x0F00) >> 8))
                    },
                    0x00A1 => {
                        Ok(Opcode::SkipIfKeyNotPressed((opcode & 0x0F00) >> 8))
                    },
                    _ => Err(CpuError::IllegalInstruction(opcode)),
                }
            },
            0xF000...0xFFFF => {
                match opcode & 0x00FF {
                    0x0007 => {
                        Ok(Opcode::AssignToDelayTime((opcode & 0x0F00) >> 8))
                    },
                    0x000A => {
                        Ok(Opcode::AssignOnKeyPress((opcode & 0x0F00) >> 8))
                    },
                    0x0015 => {
                        Ok(Opcode::SetDelayTimer((opcode & 0x0F00) >> 8))
                    },
                    0x0018 => {
                        Ok(Opcode::SetSoundTimer((opcode & 0x0F00) >> 8))
                    },
                    0x001E => {
                        Ok(Opcode::AddToIndexRegister((opcode & 0x0F00) >> 8))
                    },
                    0x0029 => {
                        Ok(Opcode::SetIndexRegisterToSpriteAddr(
                            (opcode & 0x0F00) >> 8)
                        )
                    },
                    0x0033 => {
                        Ok(Opcode::SetBCD((opcode & 0x0F00) >> 8))
                    },
                    0x0055 => {
                        Ok(Opcode::DumpRegister((opcode & 0x0F00) >> 8))
                    },
                    0x0065 => {
                        Ok(Opcode::LoadRegister((opcode & 0x0F00) >> 8))
                    },
                    _ => Err(CpuError::IllegalInstruction(opcode)),
                }
            },
            _ => Err(CpuError::IllegalInstruction(opcode)),
        }
    }

    /// Takes the current opcode in memory and attempts to decode it
    /// If the opcode is legal, the corresponding instruction is executed
    /// If the opcode is illegal, an error is returned and the program is
    /// aborted
    ///
    /// # Example
    ///
    /// ```
    /// let cpu = new Cpu::new();
    /// match cpu.step() {
    ///     Ok(()) => {
    ///         // Opcode was legal
    ///     },
    ///     Err(cpu::CpuError::IllegalInstruction(opcode)) => {
    ///         // Opcode was illegal
    ///         // Abort program
    ///         panic!("Illegal CPU instruction: {:x}", opcode)
    ///    },
    /// }
    /// ```
    fn step(&mut self) -> Result<(), CpuError> {
        let opcode_high = self.memory[self.pc as usize];
        let opcode_low = self.memory[self.pc as usize + 1];
        let opcode = (opcode_high as u16) << 8 | opcode_low as u16;
        // println!("Executing: {:x}", opcode);
        match self.decode(opcode)? {
            Opcode::CallRCAProgram(addr) => {
                // This will likely never be run
                println!("Call RCA Program at {:x}", addr);
            },
            Opcode::ClearDisplay => {
                for pixel in self.display.iter_mut() {
                    *pixel = core::Pixel::Black;
                }
                self.draw_screen = true;
                self.pc += 2;
            },
            Opcode::ReturnFromSubroutine => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },
            Opcode::JumpToAddr(addr) => {
                self.pc = addr;
            },
            Opcode::CallSubroutine(addr) => {
                self.stack[self.sp as usize] = self.pc + 2;
                self.sp += 1;
                self.pc = addr;
            },
            Opcode::SkipIfRegisterSet { addr, value } => {
                if self.registers[addr as usize] == value as u8 {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            Opcode::SkipIfRegisterNotSet { addr, value } => {
                if self.registers[addr as usize] != value as u8 {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            Opcode::SkipIfRegistersEqual { first, second } => {
                let initial = self.registers[first as usize];
                let other = self.registers[second as usize];
                if initial == other {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            }
            Opcode::SetRegister { addr, value } => {
                self.registers[addr as usize] = value as u8;
                self.pc += 2;
            },
            Opcode::AddToRegister { addr, value } => {
                let initial_value = self.registers[addr as usize] as u16;
                let sum = initial_value + value as u16;
                self.registers[addr as usize] = sum as u8;
                self.pc += 2;
            },
            Opcode::AssignRegister { first, second } => {
                let other = self.registers[second as usize];
                self.registers[first as usize] = other;
                self.pc += 2;
            },
            Opcode::AssignRegisterBitwiseOr { first, second } => {
                let initial = self.registers[first as usize];
                let other = self.registers[second as usize];
                self.registers[first as usize] = initial | other;
                self.pc += 2;
            },
            Opcode::AssignRegisterBitwiseAnd { first, second } => {
                let initial = self.registers[first as usize];
                let other = self.registers[second as usize];
                self.registers[first as usize] = initial & other;
                self.pc += 2;
            },
            Opcode::AssignRegisterBitwiseXor { first, second } => {
                let initial = self.registers[first as usize];
                let other = self.registers[second as usize];
                self.registers[first as usize] = initial ^ other;
                self.pc += 2;
            },
            Opcode::AddRegisters { first, second } => {
                let initial = self.registers[first as usize];
                let other = self.registers[second as usize];
                let (sum, carry) = initial.overflowing_add(other);
                if carry {
                    self.registers[0xF] = 1;
                }
                else {
                    self.registers[0xF] = 0;
                }
                self.registers[first as usize] = sum as u8;
                self.pc += 2;
            },
            Opcode::SubtractRegisters { first, second } => {
                let initial = self.registers[first as usize];
                let other = self.registers[second as usize];
                let (diff, borrowed) = initial.overflowing_sub(other);
                if borrowed {
                    self.registers[0xF] = 0;
                }
                else {
                    self.registers[0xF] = 1;
                }
                self.registers[first as usize] = diff as u8;
                self.pc += 2;
            },
            Opcode::AssignRegisterBitshiftRight { first, second } => {
                let other = self.registers[second as usize];
                let lsb = other & 0b0000_0001;
                self.registers[0xF] = lsb;
                self.registers[first as usize] = other >> 1;
                self.pc += 2;
            },
            Opcode::SubtractFirstRegister { first, second } => {
                let initial = self.registers[first as usize];
                let other = self.registers[second as usize];
                let (diff, borrowed) = other.overflowing_sub(initial);
                if borrowed {
                    self.registers[0xF] = 0;
                }
                else {
                    self.registers[0xF] = 1;
                }
                self.registers[first as usize] = diff as u8;

                self.pc += 2;
            },
            Opcode::AssignRegistersBitshiftLeft { first, second } => {
                let msb = (self.registers[second as usize] & 0b1000_0000) >> 7;
                self.registers[0xF] = msb;
                let result = self.registers[second as usize] << 1;
                self.registers[first as usize] = result;
                self.registers[second as usize] = result;
                self.pc += 2;
            },
            Opcode::SkipIfRegistersNotEqual { first, second } => {
                let initial = self.registers[first as usize];
                let other = self.registers[second as usize];
                if initial != other {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            Opcode::SetIndexRegister(addr) => {
                self.index_reg = addr;
                self.pc += 2;
            },
            Opcode::SetProgramCounter(addr) => {
                self.pc = self.registers[0x0] as u16 + addr;
                self.pc += 2;
            },
            Opcode::AssignRandomValue { addr, value } => {
                let result = thread_rng().gen_range(0, 255) & value;
                self.registers[addr as usize] = result as u8;
                self.pc += 2;
            },
            Opcode::Draw { x, y, height } => {
                let loc_x = self.registers[x as usize];
                let loc_y = self.registers[y as usize];
                self.registers[0xF] = 0;

                for col in 0..height {
                    let cell = self.memory[(self.index_reg + col) as usize];
                    for row in 0..8 {
                        if cell & (0b1000_0000 >> row) != 0 {
                            let actual_pos = {
                                (row+loc_x as u16) +
                                (col+loc_y as u16)*core::DISPLAY_WIDTH as u16
                            };
                            let relative_pos = actual_pos % (
                                core::DISPLAY_WIDTH*core::DISPLAY_HEIGHT
                            ) as u16;

                            let new_pixel;
                            match self.display[relative_pos as usize] {
                                core::Pixel::White => {
                                    self.registers[0xF] = 1;
                                    new_pixel = core::Pixel::Black;
                                },
                                _ => {
                                    new_pixel = core::Pixel:: White;
                                },
                            }
                            self.display[relative_pos as usize] = new_pixel;
                        }
                    }
                }

                self.draw_screen = true;
                self.pc += 2;
            },
            Opcode::SkipIfKeyPressed(addr) => {
                let key = self.registers[addr as usize];
                match self.keys[key as usize] {
                    core::KeyState::Pressed => {
                        self.pc += 4;
                    },
                    _ => {
                        self.pc += 2;
                    },
                }
            },
            Opcode::SkipIfKeyNotPressed(addr) => {
                let key = self.registers[addr as usize];
                match self.keys[key as usize] {
                    core::KeyState::Released => {
                        self.pc += 4;
                    },
                    _ => {
                        self.pc += 2;
                    },
                }
            },
            Opcode::AssignToDelayTime(addr) => {
                self.registers[addr as usize] = self.delay_timer;
                self.pc += 2;
            },
            Opcode::AssignOnKeyPress(addr) => {
                for (i, key) in self.keys.iter().enumerate() {
                    match *key {
                        core::KeyState::Pressed => {
                            self.registers[addr as usize] = i as u8;
                            self.pc += 2;
                            break;
                        },
                        _ => {},
                    }
                }
            },
            Opcode::SetDelayTimer(addr) => {
                self.delay_timer = self.registers[addr as usize];
                self.pc += 2;
            },
            Opcode::SetSoundTimer(addr) => {
                self.sound_timer = self.registers[addr as usize];
                self.pc += 2;
            },
            Opcode::AddToIndexRegister(addr) => {
                self.index_reg += self.registers[addr as usize] as u16;
                self.pc += 2;
            },
            Opcode::SetIndexRegisterToSpriteAddr(addr) => {
                self.index_reg = (self.registers[addr as usize] * 0x5) as u16;
                self.pc += 2;
            },
            Opcode::SetBCD(addr) => {
                let reg = self.registers[addr as usize];
                self.memory[self.index_reg as usize] = reg / 100;
                self.memory[(self.index_reg+1) as usize] = (reg / 10) % 10;
                self.memory[(self.index_reg+2) as usize] = (reg % 100) % 10;
                self.pc += 2;
            },
            Opcode::DumpRegister(addr) => {
                for i in 0..addr+1 {
                    let value = self.registers[i as usize];
                    self.memory[(self.index_reg + i) as usize] = value;
                }
                self.pc += 2;
            },
            Opcode::LoadRegister(addr) => {
                for i in 0..addr+1 {
                    let value = self.memory[(self.index_reg + i) as usize];
                    self.registers[i as usize] = value;
                }
                self.pc += 2;
            },
        }
        Ok(())
    }

    /// Executes the next cycle of the Cpu in an infinite loop
    /// First, checks to see if a key was pressed
    /// Second, verifies and executes the next opcode in memory
    /// Third, increments or resets the Cpu timers
    /// Redraws the screen if necessary
    ///
    /// # Arguments
    ///
    /// * `sdl_context` - A reference to an Sdl object, which is used to
    /// initialize the event loop as well as core functions, such as playing
    /// sounds and rendering the display
    ///
    /// # Example
    ///
    /// ```
    /// let cpu = new Cpu::new();
    /// let sdl_context = sdl2::init().unwrap();
    /// match cpu.run(&sdl_context) {
    ///     Ok(()) => {
    ///         // Opcode was legal
    ///     },
    ///     Err(cpu::CpuError::IllegalInstruction(opcode)) => {
    ///         // Opcode was illegal
    ///         // Abort program
    ///         panic!("Illegal CPU instruction: {:x}", opcode)
    ///    },
    /// }
    /// ```
    pub fn run(&mut self, sdl_context: &Sdl) -> Result<(), CpuError> {
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
                        core.handle_key_down(self, keycode)
                    },
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        core.handle_key_up(self, keycode)
                    },
                    _ => {},
                }
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
            self.step()?;
            self.update_timers(&mut core);
            if self.draw_screen {
                core.draw(self);
            }
        }
        Ok(())
    }

    /// Increments or resets the sound and delay timers
    /// The function checks to see if the Cpu counter is equal to 10 before
    /// changing the timers
    /// This value was chosen in order to maintain a cpu clock rate of 600 Hz
    /// Without the counter, the timers would execute at a rate of 60 Hz, so
    /// the timers must wait by a factor of 10 in order to maintain speed with
    /// the cpu
    ///
    /// # Arguments
    ///
    /// * `core` - A reference to a Core object, which here is used to
    /// play and stop sounds
    ///
    /// # Example
    ///
    /// ```
    /// let cpu = new Cpu::new();
    /// let sdl_context = sdl2::init().unwrap();
    /// let mut core = core::Core::new(&sdl_context);
    /// cpu.update_timers(&mut core);
    /// ```
    /// NOTE: This example uses the function as a public method, but the
    /// function is in fact private and should be used within the context
    /// of the Cpu object
    fn update_timers(&mut self, core: &mut Core) {
        if self.counter == 10 {
            if self.delay_timer > 0 { self.delay_timer -= 1; }
            if self.sound_timer > 0 {
                if self.sound_timer == 1 { core.play_sound(); }
                self.sound_timer -= 1;
            }
            else {
                core.stop_sound();
            }
            self.counter = 0;
        }
        else {
            self.counter += 1;
        }
    }
}

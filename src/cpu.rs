use std;
use std::fs::File;
use std::io::prelude::*;
use rand::{thread_rng, Rng};

use core;

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
    SetProgramCounter { register_addr: u16, opcode_addr: u16 },
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

#[allow(dead_code)]
pub struct Cpu {
    pc: u16,
    index_reg: u16,
    registers: Vec<u8>,
    keys: Vec<core::KeyState>,
    delay_timer: u8,
    sound_timer: u8,
    pub display: Vec<core::Pixel>,
    pub draw_screen: bool,
    stack: Vec<u16>,
    sp: u16,
    memory: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0x200,
            index_reg: 0,
            registers: vec![0; 16],
            keys: vec![core::KeyState::Released; 16],
            delay_timer: 0,
            sound_timer: 0,
            display: vec![core::Pixel::Black; 64*32],
            draw_screen: false,
            stack: vec![0; 16],
            sp: 0,
            memory: vec![0; 4096],
        }
    }

    pub fn load_fontset(&mut self) {
        for i in 0..80 {
            self.memory[i] = core::FONTSET[i];
        }
    }

    pub fn load_rom(&mut self, path: &str) -> std::io::Result<()> {
        let mut rom = File::open(path)?;
        let mut buffer = vec![];
        rom.read_to_end(&mut buffer)?;
        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }
        Ok(())
    }

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
            0xB000...0xBFFF => {
                Ok(Opcode::SetProgramCounter {
                    register_addr: 0x0000,
                    opcode_addr: opcode & 0x0FFF,
                })
            },
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

    pub fn step(&mut self) -> Result<(), CpuError> {
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
                self.pc += 2;
            },
            Opcode::JumpToAddr(addr) => {
                self.pc = addr;
            },
            Opcode::CallSubroutine(addr) => {
                self.stack[self.sp as usize] = self.pc;
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
                if self.registers[first as usize] ==
                   self.registers[second as usize]
                {
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
                self.registers[addr as usize] += value as u8;
                self.pc += 2;
            },
            Opcode::AssignRegister { first, second } => {
                self.registers[first as usize] = second as u8;
                self.pc += 2;
            },
            Opcode::AssignRegisterBitwiseOr { first, second } => {
                self.registers[first as usize] |=
                self.registers[second as usize];
                self.pc += 2;
            },
            Opcode::AssignRegisterBitwiseAnd { first, second } => {
                self.registers[first as usize] &=
                self.registers[second as usize];
                self.pc += 2;
            },
            Opcode::AssignRegisterBitwiseXor { first, second } => {
                self.registers[first as usize] ^=
                self.registers[second as usize];
                self.pc += 2;
            },
            Opcode::AddRegisters { first, second } => {
                let (sum, overflowed) =
                    self.registers[first as usize].overflowing_add(
                        self.registers[second as usize]
                    );
                if overflowed {
                    self.registers[0xF] = 1;
                }
                else {
                    self.registers[0xF] = 0;
                }
                self.registers[first as usize] = sum as u8;
                self.pc += 2;
            },
            Opcode::SubtractRegisters { first, second } => {
                let (diff, borrowed) =
                    self.registers[first as usize].overflowing_sub(
                        self.registers[second as usize]
                    );
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
                self.registers[0xF] = self.registers[first as usize] & 0x01;
                self.registers[first as usize] =
                    self.registers[second as usize] >> 1;
                self.pc += 2;
            },
            Opcode::SubtractFirstRegister { first, second } => {
                let result = self.registers[second as usize] -
                             self.registers[first as usize];
                self.registers[first as usize] = result;
                self.pc += 2;
            },
            Opcode::AssignRegistersBitshiftLeft { first, second } => {
                self.registers[0xF] =
                    (self.registers[first as usize] & 0x80) >> 7;
                let result = self.registers[second as usize] << 1;
                self.registers[first as usize] = result;
                self.registers[second as usize] = result;
                self.pc += 2;
            },
            Opcode::SkipIfRegistersNotEqual { first, second } => {
                if self.registers[first as usize] !=
                   self.registers[second as usize]
                {
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
            Opcode::SetProgramCounter { register_addr, opcode_addr } => {
                self.pc = register_addr + opcode_addr;
            },
            Opcode::AssignRandomValue { addr, value } => {
                let result = thread_rng().gen_range(0, 255) & value;
                self.registers[addr as usize] = result as u8;
                self.pc += 2;
            },
            Opcode::Draw { x, y, height } => {
                // let mut pixel_was_unset = false;
                // self.registers[0xF] = 0;

                // for col in 0..height {
                //     let cell = self.memory[(self.index_reg + col) as usize];
                //     for row in 0..8 {
                //         let i = ((row+x)*32 + (col+y)) as usize;
                //         let curr_pixel = self.display[i];
                //         let new_pixel = cell & (0b1000_0000 >> row);
                //         let new_pixel = match new_pixel {
                //             0 => core::Pixel::Black,
                //             _ => core::Pixel::White,
                //         };

                //         match (curr_pixel, new_pixel) {
                //             (core::Pixel::White, core::Pixel::Black) => {
                //                 pixel_was_unset = true;
                //             },
                //             _ => {},
                //         }
                //         self.display[i] = new_pixel;
                //     }
                // }
                // self.registers[0xF] = match pixel_was_unset {
                //     true => 1,
                //     false => 0,
                // };

                let loc_x = self.registers[x as usize];
                let loc_y = self.registers[y as usize];
                self.registers[0xF] = 0;
                for y_line in 0..height {
                    let pixel = self.memory[(self.index_reg + y_line) as usize];
                    for x_line in 0..8 {
                        if pixel & (0x80 >> x_line) != 0 {
                            let sprite_loc = loc_x as u16 + x_line + ((loc_y as u16 + y_line)*64);
                            let curr_pixel = self.display[sprite_loc as usize];
                            let new_pixel;
                            match curr_pixel {
                                core::Pixel::Black => {
                                    self.registers[0xF] = 1;
                                    new_pixel = core::Pixel::White;
                                },
                                _ => { new_pixel = core::Pixel::Black },
                            }
                            self.display[sprite_loc as usize] = new_pixel;
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
                self.delay_timer = addr as u8;
                self.pc += 2;
            },
            Opcode::SetSoundTimer(addr) => {
                self.sound_timer = addr as u8;
                self.pc += 2;
            },
            Opcode::AddToIndexRegister(addr) => {
                self.index_reg += addr;
                self.pc += 2;
            },
            Opcode::SetIndexRegisterToSpriteAddr(addr) => {
                self.index_reg = (self.registers[addr as usize] * 0x5) as u16;
                self.pc += 2;
            },
            Opcode::SetBCD(addr) => {
                let reg = self.registers[addr as usize];
                self.memory[self.index_reg as usize] = reg / 100;
                self.memory[(self.index_reg+1) as usize] = (reg/10) % 10;
                self.memory[(self.index_reg+2) as usize] = (reg%100) % 10;
                self.pc += 2;
            },
            Opcode::DumpRegister(addr) => {
                for i in 0..addr+1 {
                    self.memory[self.index_reg as usize] =
                        self.registers[i as usize];
                    self.index_reg += 1;
                }
                self.pc += 2;
            },
            Opcode::LoadRegister(addr) => {
                for i in 0..addr+1 {
                    self.registers[i as usize] =
                        self.memory[self.index_reg as usize];
                    self.index_reg += 1;
                }
                self.pc += 2;
            },
        }
        Ok(())
    }
}

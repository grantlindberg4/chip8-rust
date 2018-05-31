use std::fs::File;
use std::io::prelude::*;

enum Opcode {
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
    AssignToKeyPress(u16),
    SetDelayTimer(u16),
    SetSoundTimer(u16),
    AddToIndexRegister(u16),
    SetIndexRegisterToSpriteAddr(u16),
    SetBCD(u16),
    DumpRegister(u16),
    LoadRegister(u16),
}

enum CpuError {
    IllegalInstruction(u16),
}

#[derive(Clone, Copy)]
enum Pixel {
    Black,
    White,
}

#[allow(dead_code)]
struct Cpu {
    pc: u16,
    index_reg: u16,
    registers: Vec<u8>,
    graphics: Vec<Pixel>,
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
            graphics: vec![Pixel::Black; 64*32],
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

    fn run(&mut self) -> Result<(), CpuError> {
        loop {
            self.step()?;
        }
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
                    height: opcode & 0x00FF,
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
                        Ok(Opcode::AssignToKeyPress((opcode & 0x0F00) >> 8))
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

    fn step(&mut self) -> Result<(), CpuError> {
        let opcode_high = self.memory[self.pc as usize];
        let opcode_low = self.memory[self.pc as usize + 1];
        let opcode = (opcode_high as u16) << 8 | opcode_low as u16;
        println!("Executing: {:x}", opcode);
        match self.decode(opcode)? {
            Opcode::CallRCAProgram(addr) => {
                println!("Call RCA Program at {:x}", addr);
            },
            Opcode::ClearDisplay => {
                println!("Clear the screen");
            },
            Opcode::ReturnFromSubroutine => {
                println!("Return from subroutine");
            },
            Opcode::JumpToAddr(addr) => {
                println!("Jump to {:x}", addr);
            },
            Opcode::CallSubroutine(addr) => {
                println!("Call subroutine at {:x}", addr);
            },
            Opcode::SkipIfRegisterSet { addr, value } => {
                println!("Skip next if {:x} equals {:x}", addr, value);
            },
            Opcode::SkipIfRegisterNotSet { addr, value } => {
                println!("Skip next if {:x} does not equal {:x}", addr, value);
            },
            Opcode::SkipIfRegistersEqual { first, second } => {
                println!("Skip next if {:x} equals {:x}", first, second);
            }
            Opcode::SetRegister { addr, value } => {
                println!("Assign {:x} to {:x}", value, addr);
            },
            Opcode::AddToRegister { addr, value } => {
                println!("Add {:x} to {:x}", value, addr);
            },
            Opcode::AssignRegister { first, second } => {
                println!("Assign {:x} to {:x}", second, first);
            },
            Opcode::AssignRegisterBitwiseOr { first, second } => {
                println!("Assign {:x} | {:x} to {:x}", first, second, first);
            },
            Opcode::AssignRegisterBitwiseAnd { first, second } => {
                println!("Assign {:x} & {:x} to {:x}", first, second, first);
            },
            Opcode::AssignRegisterBitwiseXor { first, second } => {
                println!("Assign {:x} ^ {:x} to {:x}", first, second, first);
            },
            Opcode::AddRegisters { first, second } => {
                println!("Add {:x} to {:x}", second, first);
            },
            Opcode::SubtractRegisters { first, second } => {
                println!("Subtract {:x} from {:x}", second, first);
            },
            Opcode::AssignRegisterBitshiftRight { first, second } => {
                println!("Assign {:x} to {:x} >> 1", first, second);
            },
            Opcode::SubtractFirstRegister { first, second } => {
                println!("Assign {:x} - {:x} to {:x}", second, first, first);
            },
            Opcode::AssignRegistersBitshiftLeft { first, second } => {
                println!("Assign {:x} and {:x} to {:x} << 1", first, second, second);
            },
            Opcode::SkipIfRegistersNotEqual { first, second } => {
                println!("Skip if {:x} does not equal {:x}", first, second);
            },
            Opcode::SetIndexRegister(addr) => {
                println!("Set index register to {:x}", addr);
            },
            Opcode::SetProgramCounter { register_addr, opcode_addr } => {
                println!("Set program counter to {:x} + {:x}", register_addr, opcode_addr);
            },
            Opcode::AssignRandomValue { addr, value } => {
                println!("Assign rand() & {:x} to {:x}", value, addr);
            },
            Opcode::Draw { x, y, height } => {
                println!("Draw sprite at ({:x}, {:x}, {:x})", x, y, height);
            },
            Opcode::SkipIfKeyPressed(addr) => {
                println!("Skip if key pressed at {:x}", addr);
            },
            Opcode::SkipIfKeyNotPressed(addr) => {
                println!("Skip if key not pressed at {:x}", addr);
            },
            Opcode::AssignToDelayTime(addr) => {
                println!("Assign delay time to {:x}", addr);
            },
            Opcode::AssignToKeyPress(addr) => {
                println!("Assign key press to {:x}", addr);
            },
            Opcode::SetDelayTimer(addr) => {
                println!("Set delay timer to value of {:x}", addr);
            },
            Opcode::SetSoundTimer(addr) => {
                println!("Set sound timer to value of {:x}", addr);
            },
            Opcode::AddToIndexRegister(addr) => {
                println!("Add value of {:x} to index register", addr);
            },
            Opcode::SetIndexRegisterToSpriteAddr(addr) => {
                println!("Set index register to location of sprite in {:x}", addr);
            },
            Opcode::SetBCD(addr) => {
                println!("Set index register to BCD of {:x}", addr);
            },
            Opcode::DumpRegister(addr) => {
                println!("Store value at 0x0 to {:x}", addr);
            },
            Opcode::LoadRegister(addr) => {
                println!("Fill value at 0x0 to {:x}", addr);
            },
        }
        self.pc += 2;
        Ok(())
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
        Err(CpuError::IllegalInstruction(opcode)) => {
            panic!("Illegal CPU instruction: {:x}", opcode)
        },
    }
}

use std::{
    fs::File,
    io::{Read, Seek},
};

use crate::{register::Register, screen::Screen};

pub fn start(register: &mut Register, screen: &mut Screen) {
    let mut rom: File = File::open("Pong.ch8").expect("Failed to open file");
    let rom_data = {
        let mut data = Vec::new();
        rom.read_to_end(&mut data).expect("Failed to read file");
        data
    };
    rom.seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek file");

    let mut buffer: [u8; 2] = [0; 2];

    loop {
        match rom.read_exact(&mut buffer) {
            Ok(_) => {
                let opcode = u16::from_be_bytes(buffer);

                match opcode & 0xF000 {
                    0x0000 => match opcode {
                        0x00E0 => screen.clear(),
                        0x00EE => println!("{:04X} RET", opcode),
                        _ => {
                            let addr = opcode & 0x0FFF;
                            println!("{:04X} SYS {:03X}", opcode, addr);
                        }
                    },
                    0x1000 => {
                        let addr = opcode & 0x0FFF;
                        println!("{:04X} JP {:03X}", opcode, addr);
                    }
                    0x2000 => {
                        let addr = opcode & 0x0FFF;
                        println!("{:04X} CALL {:03X}", opcode, addr);
                    }
                    0x3000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let byte = opcode & 0x00FF;
                        println!("{:04X} SE V{:X}, {:02X}", opcode, x, byte);
                    }
                    0x4000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let byte = opcode & 0x00FF;
                        println!("{:04X} SNE V{:X}, {:02X}", opcode, x, byte);
                    }
                    0x5000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let y = (opcode & 0x00F0) >> 4;
                        println!("{:04X} SE V{:X}, V{:X}", opcode, x, y);
                    }
                    0x6000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let byte = opcode & 0x00FF;
                        println!("{:04X} LD V{:X}, {:02X}", opcode, x, byte);
                        register.set_register(x, byte as u8);
                    }
                    0x7000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let byte = opcode & 0x00FF;
                        println!("{:04X} ADD V{:X}, {:02X}", opcode, x, byte);
                    }
                    0x8000 => match opcode & 0x000F {
                        0x0000 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} LD V{:X}, V{:X}", opcode, x, y);
                        }
                        0x0001 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} OR V{:X}, V{:X}", opcode, x, y);
                        }
                        0x0002 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} AND V{:X}, V{:X}", opcode, x, y);
                        }
                        0x0003 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} XOR V{:X}, V{:X}", opcode, x, y);
                        }
                        0x0004 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} ADD V{:X}, V{:X}", opcode, x, y);
                        }
                        0x0005 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} SUB V{:X}, V{:X}", opcode, x, y);
                        }
                        0x0006 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} SHR V{:X}", opcode, x);
                        }
                        0x0007 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} SUBN V{:X}, V{:X}", opcode, x, y);
                        }
                        0x000E => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} SHL V{:X}", opcode, x);
                        }
                        _ => (),
                    },
                    0x9000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let y = (opcode & 0x00F0) >> 4;
                        println!("{:04X} SNE V{:X}, V{:X}", opcode, x, y);
                    }
                    0xA000 => {
                        let addr = opcode & 0x0FFF;
                        println!("{:04X} LD I, {:03X}", opcode, addr);
                        register.i = addr - 0x200;
                    }
                    0xB000 => {
                        let addr = opcode & 0x0FFF;
                        println!("{:04X} JP V0, {:03X}", opcode, addr);
                    }
                    0xC000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let byte = opcode & 0x00FF;
                        println!("{:04X} RND V{:X}, {:02X}", opcode, x, byte);
                    }
                    0xD000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let y = (opcode & 0x00F0) >> 4;
                        let nibble = opcode & 0x000F;
                        let register_x = register.get_register(x) as usize;
                        let register_y = register.get_register(y) as usize;
                        println!("{:04X} DRW V{:X}, V{:X}, {:X}", opcode, x, y, nibble);

                        for i in 0..nibble {
                            let index = register.i as usize + i as usize;
                            if index < rom_data.len() {
                                let byte = rom_data[index];
                            } else {
                                println!("Index out of bounds");
                            }
                        }
                    }
                    0xE000 => match opcode & 0x00FF {
                        0x009E => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} SKP V{:X}", opcode, x);
                        }
                        0x00A1 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} SKNP V{:X}", opcode, x);
                        }
                        _ => (),
                    },
                    0xF000 => match opcode & 0x00FF {
                        0x0007 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD V{:X}, DT", opcode, x);
                        }
                        0x000A => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD V{:X}, K", opcode, x);
                        }
                        0x0015 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD DT, V{:X}", opcode, x);
                        }
                        0x0018 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD ST, V{:X}", opcode, x);
                        }
                        0x001E => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} ADD I, V{:X}", opcode, x);
                        }
                        0x0029 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD F, V{:X}", opcode, x);
                        }
                        0x0033 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD B, V{:X}", opcode, x);
                        }
                        0x0055 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD [I], V{:X}", opcode, x);
                        }
                        0x0065 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD V{:X}, [I]", opcode, x);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    panic!("Failed to read file: {:?}", e);
                }
            }
        }
    }
}

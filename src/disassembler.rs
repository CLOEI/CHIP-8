use std::{
    fs::File,
    io::{Read, Seek},
};

use crate::{
    register::Register,
    screen::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH},
};

pub fn start(register: &mut Register, screen: &mut Screen) {
    let mut rom: File = File::open("Pong.ch8").expect("Failed to open file");
    let mut memory = {
        let mut memory = [0; 0xFFF];
        let mut data = Vec::new();
        rom.read_to_end(&mut data).expect("Failed to read file");

        for (i, byte) in data.iter().enumerate() {
            memory[i + 0x200] = *byte;
        }
        memory
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
                        0x00EE => {
                            println!("{:04X} RET", opcode);
                            register.pc = register.stack.pop().unwrap();
                            register.sp -= 1;
                        }
                        _ => {
                            let addr = opcode & 0x0FFF;
                            println!("{:04X} SYS {:03X}", opcode, addr);
                        }
                    },
                    0x1000 => {
                        let addr = opcode & 0x0FFF;
                        println!("{:04X} JP {:03X}", opcode, addr);
                        register.pc = addr;
                    }
                    0x2000 => {
                        let addr = opcode & 0x0FFF;
                        println!("{:04X} CALL {:03X}", opcode, addr);
                        register.sp += 1;
                        register.stack.push(register.pc);
                    }
                    0x3000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let byte = opcode & 0x00FF;
                        println!("{:04X} SE V{:X}, {:02X}", opcode, x, byte);

                        let register_x = register.get_register(x);
                        if register_x == byte as u8 {
                            register.pc += 2;
                        }
                    }
                    0x4000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let byte = opcode & 0x00FF;
                        println!("{:04X} SNE V{:X}, {:02X}", opcode, x, byte);

                        let register_x = register.get_register(x);
                        if register_x != byte as u8 {
                            register.pc += 2;
                        }
                    }
                    0x5000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let y = (opcode & 0x00F0) >> 4;
                        println!("{:04X} SE V{:X}, V{:X}", opcode, x, y);

                        let register_x = register.get_register(x);
                        let register_y = register.get_register(y);
                        if register_x == register_y {
                            register.pc += 2;
                        }
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

                        let register_x = register.get_register(x);
                        let (result, _) = register_x.overflowing_add(byte as u8);
                        register.set_register(x, result);
                    }
                    0x8000 => match opcode & 0x000F {
                        0x0000 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} LD V{:X}, V{:X}", opcode, x, y);

                            let register_y = register.get_register(y);
                            register.set_register(x, register_y);
                        }
                        0x0001 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} OR V{:X}, V{:X}", opcode, x, y);

                            let register_x = register.get_register(x);
                            let register_y = register.get_register(y);

                            register.set_register(x, register_x | register_y);
                        }
                        0x0002 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} AND V{:X}, V{:X}", opcode, x, y);

                            let register_x = register.get_register(x);
                            let register_y = register.get_register(y);

                            register.set_register(x, register_x & register_y);
                        }
                        0x0003 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} XOR V{:X}, V{:X}", opcode, x, y);

                            let register_x = register.get_register(x);
                            let register_y = register.get_register(y);

                            register.set_register(x, register_x ^ register_y);
                        }
                        0x0004 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} ADD V{:X}, V{:X}", opcode, x, y);

                            let register_x = register.get_register(x);
                            let register_y = register.get_register(y);

                            let (result, overflow) = register_x.overflowing_add(register_y);
                            register.set_register(x, result);
                            register.set_register(0xF, overflow as u8);
                        }
                        0x0005 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} SUB V{:X}, V{:X}", opcode, x, y);

                            let register_x = register.get_register(x);
                            let register_y = register.get_register(y);

                            let (result, borrow) = register_x.overflowing_sub(register_y);
                            register.set_register(x, result);
                            register.set_register(0xF, !borrow as u8);
                        }
                        0x0006 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} SHR V{:X}", opcode, x);

                            let register_x = register.get_register(x);
                            if register_x & 0x1 == 1 {
                                register.set_register(0xF, 1);
                            } else {
                                register.set_register(0xF, 0);
                            }

                            register.set_register(x, register_x >> 1);
                        }
                        0x0007 => {
                            let x = (opcode & 0x0F00) >> 8;
                            let y = (opcode & 0x00F0) >> 4;
                            println!("{:04X} SUBN V{:X}, V{:X}", opcode, x, y);

                            let register_x = register.get_register(x);
                            let register_y = register.get_register(y);

                            if register_x > register_y {
                                register.set_register(0xF, 1);
                            } else {
                                register.set_register(0xF, 0);
                            }

                            register.set_register(x, register_y - register_x);
                        }
                        0x000E => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} SHL V{:X}", opcode, x);

                            let register_x = register.get_register(x);
                            if register_x & 0x80 == 0x80 {
                                register.set_register(0xF, 1);
                            } else {
                                register.set_register(0xF, 0);
                            }

                            register.set_register(x, register_x << 1);
                        }
                        _ => (),
                    },
                    0x9000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let y = (opcode & 0x00F0) >> 4;
                        println!("{:04X} SNE V{:X}, V{:X}", opcode, x, y);

                        let register_x = register.get_register(x);
                        let register_y = register.get_register(y);

                        if register_x != register_y {
                            register.pc += 2;
                        }
                    }
                    0xA000 => {
                        let addr = opcode & 0x0FFF;
                        println!("{:04X} LD I, {:03X}", opcode, addr);
                        register.i = addr;
                    }
                    0xB000 => {
                        let addr = opcode & 0x0FFF;
                        println!("{:04X} JP V0, {:03X}", opcode, addr);

                        register.pc = addr + register.get_register(0) as u16;
                    }
                    0xC000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let byte = opcode & 0x00FF;
                        println!("{:04X} RND V{:X}, {:02X}", opcode, x, byte);

                        let random: u8 = rand::random();

                        register.set_register(x, random & byte as u8);
                    }
                    0xD000 => {
                        let x = (opcode & 0x0F00) >> 8;
                        let y = (opcode & 0x00F0) >> 4;
                        let nibble = opcode & 0x000F;
                        let register_x = register.get_register(x) as usize;
                        let register_y = register.get_register(y) as usize;
                        println!("{:04X} DRW V{:X}, V{:X}, {:X}", opcode, x, y, nibble);

                        println!("Register X: {}", register_x);
                        println!("Register Y: {}", register_y);

                        register.set_register(0xF, 0);

                        for i in 0..nibble {
                            let index = register.i as usize + i as usize;
                            let byte = memory[index];
                            for j in 0..8 {
                                let x = (register_x + j) % SCREEN_WIDTH;
                                let y = (register_y + i as usize) % SCREEN_HEIGHT;
                                let pixel = byte >> (7 - j);

                                if pixel == 1 && screen.pixels[y][x] {
                                    register.set_register(0xF, 1);
                                }

                                screen.pixels[y][x] ^= pixel == 1;
                            }
                        }
                    }
                    0xE000 => match opcode & 0x00FF {
                        0x009E => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} SKP V{:X}", opcode, x);

                            let register_x = register.get_register(x);
                            if register_x == 1 {
                                register.pc += 2;
                            }
                        }
                        0x00A1 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} SKNP V{:X}", opcode, x);

                            let register_x = register.get_register(x);
                            if register_x == 0 {
                                register.pc += 2;
                            }
                        }
                        _ => (),
                    },
                    0xF000 => match opcode & 0x00FF {
                        0x0007 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD V{:X}, DT", opcode, x);

                            register.set_register(x, register.timer.delay);
                        }
                        0x000A => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD V{:X}, K", opcode, x);

                            let mut key = [0; 16];
                            std::io::stdin().read(&mut key).expect("Failed to read key");

                            for (i, &k) in key.iter().enumerate() {
                                if k == 1 {
                                    register.set_register(x, i as u8);
                                    break;
                                }
                            }
                        }
                        0x0015 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD DT, V{:X}", opcode, x);

                            register.timer.delay = register.get_register(x);
                        }
                        0x0018 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD ST, V{:X}", opcode, x);

                            register.timer.sound = register.get_register(x);
                        }
                        0x001E => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} ADD I, V{:X}", opcode, x);

                            register.i += register.get_register(x) as u16;
                        }
                        0x0029 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD F, V{:X}", opcode, x);

                            register.i = register.get_register(x) as u16 * 5;
                        }
                        0x0033 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD B, V{:X}", opcode, x);

                            let register_x = register.get_register(x);
                            memory[register.i as usize] = register_x / 100;
                            memory[register.i as usize + 1] = (register_x / 10) % 10;
                            memory[register.i as usize + 2] = register_x % 10;
                        }
                        0x0055 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD [I], V{:X}", opcode, x);

                            for i in 0..=x {
                                memory[register.i as usize + i as usize] = register.get_register(i);
                            }
                        }
                        0x0065 => {
                            let x = (opcode & 0x0F00) >> 8;
                            println!("{:04X} LD V{:X}, [I]", opcode, x);

                            for i in 0..=x {
                                register.set_register(i, memory[register.i as usize + i as usize]);
                            }
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

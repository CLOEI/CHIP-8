use core::num;

#[derive(Debug, Default)]
pub struct Register {
    pub v_0: u8,
    pub v_1: u8,
    pub v_2: u8,
    pub v_3: u8,
    pub v_4: u8,
    pub v_5: u8,
    pub v_6: u8,
    pub v_7: u8,
    pub v_8: u8,
    pub v_9: u8,
    pub v_a: u8,
    pub v_b: u8,
    pub v_c: u8,
    pub v_d: u8,
    pub v_e: u8,
    pub v_f: u8,
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub timer: Timer,
}

#[derive(Debug, Default)]
pub struct Timer {
    pub delay: u8,
    pub sound: u8,
}

impl Register {
    pub fn set_register(&mut self, number: u16, byte: u8) {
        match number {
            0x0 => self.v_0 = byte,
            0x1 => self.v_1 = byte,
            0x2 => self.v_2 = byte,
            0x3 => self.v_3 = byte,
            0x4 => self.v_4 = byte,
            0x5 => self.v_5 = byte,
            0x6 => self.v_6 = byte,
            0x7 => self.v_7 = byte,
            0x8 => self.v_8 = byte,
            0x9 => self.v_9 = byte,
            0xA => self.v_a = byte,
            0xB => self.v_b = byte,
            0xC => self.v_c = byte,
            0xD => self.v_d = byte,
            0xE => self.v_e = byte,
            0xF => self.v_f = byte,
            _ => (),
        }
    }

    pub fn get_register(&self, number: u16) -> u8 {
        match number {
            0x0 => self.v_0,
            0x1 => self.v_1,
            0x2 => self.v_2,
            0x3 => self.v_3,
            0x4 => self.v_4,
            0x5 => self.v_5,
            0x6 => self.v_6,
            0x7 => self.v_7,
            0x8 => self.v_8,
            0x9 => self.v_9,
            0xA => self.v_a,
            0xB => self.v_b,
            0xC => self.v_c,
            0xD => self.v_d,
            0xE => self.v_e,
            0xF => self.v_f,
            _ => 0,
        }
    }
}

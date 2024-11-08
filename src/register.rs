#[derive(Debug, Default)]
pub struct Register {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    vA: u8,
    vB: u8,
    vC: u8,
    vD: u8,
    vE: u8,
    vF: u8,
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    timer: Timer,
}

#[derive(Debug, Default)]
pub struct Timer {
    delay: u8,
    sound: u8,
}

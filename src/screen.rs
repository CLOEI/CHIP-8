pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Screen {
    pub pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            pixels: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }
}

impl Screen {
    pub fn draw(&self) {}
    pub fn clear(&mut self) {
        self.pixels = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    }
}

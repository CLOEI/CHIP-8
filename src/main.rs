use register::Register;
use screen::Screen;

mod disassembler;
mod register;
mod screen;

fn main() {
    let mut register = Register::default();
    let mut screen = Screen::default();

    disassembler::start(&mut register, &mut screen);

    // loop {
    //     screen.draw();
    // }
    screen.draw();
}

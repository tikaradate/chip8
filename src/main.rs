mod cpu;

fn main() {
    let path_rom = std::env::args().nth(1).expect("no rom given");
    let mut emulator = cpu::Chip8::init();
    emulator.load_rom(&path_rom);

    // both width and height are multiplied by a constant, 
    // otherwise it would be too small
    const ZOOM: u32 = 10;
    let height = ZOOM*64;
    let width  = ZOOM*32;
 
    // main loop
    loop {
        break;
    }
}
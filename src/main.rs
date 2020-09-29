use::ggez::*;

mod cpu;
mod inputs;

struct emulator{
    chip8: cpu::Chip8,
    keys: inputs::keys,
}
fn main() {
    let path_rom = std::env::args().nth(1).expect("no rom given");
    let mut emulator = cpu::Chip8::init();
    emulator.load_rom(&path_rom);
    
    // both width and height are multiplied by a constant, 
    // otherwise it would be too small
    const ZOOM: u32 = 10;
    let height = ZOOM*64;
    let width  = ZOOM*32;

    let (mut ctx, mut event_loop) =
       ContextBuilder::new("CHIP-8", "x")
           .build()
           .unwrap();

    let mut my_game = MyGame::new(&mut ctx);

 
    // main loop
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
use ::ggez::*;

mod cpu;
mod inputs;

struct emulator {
    chip8: cpu::Chip8,
    keys: inputs::keys,
}
impl emulator {
    // for the loop
    // do it on main drawing in emulator_state, otherwise
    // it will get messy
    pub fn draw(chip8: &cpu::Chip8, ctx: &mut Context) -> GameResult<()> {
        // bg is black and fg is white
        let mesh = &mut graphics::MeshBuilder::new();
        let background: graphics::Color = [0.0, 0.0, 0.0, 0.0].into();
        let foreground: graphics::Color = [1.0, 1.0, 1.0, 1.0].into();
        let color;
        for i in 0..64 {
            for j in 0..32 {
                if chip8.gfx[i] == true {
                    color = foreground;
                } else {
                    color = background;
                }
                let rect = graphics::Rect::new(i as f32,j as f32, 1.0, 1.2);
                let mesh_rect = graphics::MeshBuilder::rectangle(mesh, graphics::DrawMode::fill(), rect, color);
            }
        }
        Ok(())
    }
}
fn main() {
    let path_rom = std::env::args().nth(1).expect("no rom given");
    let mut emulator = cpu::Chip8::init();
    emulator.load_rom(&path_rom);

    // both width and height are multiplied by a constant,
    // otherwise it would be too small
    const ZOOM: u32 = 10;
    let height = ZOOM * 64;
    let width = ZOOM * 32;

    let (mut ctx, mut event_loop) = ContextBuilder::new("CHIP-8", "x").build().unwrap();

    let mut my_game = MyGame::new(&mut ctx);

    // main loop
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

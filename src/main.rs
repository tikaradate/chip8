use ::ggez::*;
use std::time::{Duration, SystemTime};
use std::thread;

mod cpu;
mod input;

const ZOOM: usize = 10;
const HEIGHT: usize = 64;
const WIDTH: usize = 32;

struct Emulator {
    chip8: cpu::Chip8,
    acc_time: Duration,
}
impl Emulator {
    pub fn new() -> Self {
        Emulator {
            chip8: cpu::Chip8::init(),
            acc_time: Duration::new(0,0),
        }
    }
}
impl event::EventHandler for Emulator {
    // for the loop
    // do it on main drawing in emulator_state, otherwise
    // it will get messy
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // only draws if it has been asked by an opcode
        if self.chip8.update_screen == true {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        let mut screen_mesh = graphics::MeshBuilder::new();
        let final_mesh;
        // bg is black and fg is white
        // maybe redundant??
        let background = graphics::BLACK;
        let foreground = graphics::WHITE;
        let mut color;
        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                if self.chip8.gfx[i*WIDTH as usize + j] == true {
                    color = foreground;
                } else {
                    color = background;
                }
                // adds the new rectangle to the screen_mesh
                screen_mesh.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new((i*ZOOM) as f32, (j*ZOOM) as f32, ZOOM as f32, ZOOM as f32),
                    color,
                );
                
            }
        }
        final_mesh = screen_mesh.build(ctx)?;
        graphics::draw(ctx, &final_mesh, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
        } else {
            Ok(())
        }
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // this equals around 500Hz, although it's not precise
        let milli = Duration::from_millis(4);
        self.acc_time += milli;
        println!("delay.timer: {}", self.chip8.delay_timer);
        if self.acc_time >= Duration::from_millis(16) && self.chip8.delay_timer != 0{
            self.acc_time = Duration::from_millis(0);
            self.chip8.delay_timer -= 1;
        }
        self.chip8.get_opcode();
        self.chip8.decode_opcode();
        //TODO: using the flags in the chip8 struct, pause until the input is received
        thread::sleep(milli);
        Ok(())
    }
}
fn main() {
    // getting the rom path from cmd
    let path_rom = std::env::args().nth(1).expect("no rom given");

    let (mut ctx, mut event_loop) = ContextBuilder::new("CHIP-8", "x")
    .window_setup(ggez::conf::WindowSetup::default().title("CHIP-8"))
    .window_mode(ggez::conf::WindowMode::default().dimensions((HEIGHT*ZOOM) as f32, (WIDTH*ZOOM) as f32))
    .build().unwrap();

    let emulator = &mut Emulator::new();
    emulator.chip8.load_rom(&path_rom);
    emulator.chip8.load_font();

    // main loop
    match event::run(&mut ctx, &mut event_loop, emulator) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

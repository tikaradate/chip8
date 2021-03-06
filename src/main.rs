use ::ggez::*;
use ggez::event::{KeyCode};
use std::thread;
use std::time::{Duration, Instant};

mod cpu;

// the chip-8 has a screen of 64x32, which is quite small
// for modern screens, so a ZOOM is applied
const ZOOM: usize = 20;
const HEIGHT: usize = 32;
const WIDTH: usize = 64;


struct Emulator {
    chip8: cpu::Chip8,
    acc_timer: Duration,
}
impl Emulator {
    pub fn new() -> Self {
        Emulator {
            chip8: cpu::Chip8::init(),
            acc_timer: Duration::new(0, 0),
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
            //graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
            let mut screen_mesh = graphics::MeshBuilder::new();
            let final_mesh;
            // bg is black and fg is white
            let background = graphics::BLACK;
            let foreground = graphics::WHITE;
            let mut color;
            for i in 0..HEIGHT as usize {
                for j in 0..WIDTH as usize {
                    if self.chip8.gfx[i * WIDTH as usize + j] == true {
                        color = foreground;
                    } else {
                        color = background;
                    }
                    // adds the new rectangle to the screen_mesh
                    screen_mesh.rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            (j * ZOOM) as f32,
                            (i * ZOOM) as f32,
                            ZOOM as f32,
                            ZOOM as f32,
                        ),
                        color,
                    );
                }
            }       
            // builds the final mesh
            final_mesh = screen_mesh.build(ctx)?;
            // draws it to the screen
            graphics::draw(ctx, &final_mesh, graphics::DrawParam::default())?;
            graphics::present(ctx)?;
            self.chip8.update_screen = false;
            Ok(())
        } else {
            Ok(())
        }
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // this equals to 500Hz, maybe change to user input in the future
        let milli = Duration::from_millis(2);
        let sixty_hz = Duration::from_millis(16);
        let now = Instant::now();
        
        self.chip8.get_opcode();
        self.chip8.decode_opcode();
        self.acc_timer += now.elapsed();
        // the delay timer needs to be decreased at a rate of 60Hz
        if self.acc_timer >= sixty_hz && self.chip8.delay_timer > 0 {
            self.acc_timer -= sixty_hz;
            self.chip8.delay_timer -= 1;
        }
        // sleeps for the necessary time - the time it took to process the opcodes
        let real_milli = milli - now.elapsed();
        thread::sleep(real_milli);
        self.acc_timer += real_milli;
        Ok(())
    }

    // checks if a key has been pressed
    // maybe in a different file?
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool
    ) 
    {
        match keycode {
            KeyCode::Key1 => {
                self.chip8.key[1] = true;
            }
            KeyCode::Key2 => {
                self.chip8.key[2] = true;
            }
            KeyCode::Key3 => {
                self.chip8.key[3] = true;
            }
            KeyCode::Key4 => {
                self.chip8.key[0xC] = true;
            }
            KeyCode::Q  => {
                self.chip8.key[4] = true;
            }
            KeyCode::W => {
                self.chip8.key[5] = true;
            }
            KeyCode::E => {
                self.chip8.key[6] = true;
            }
            KeyCode::R => {
                self.chip8.key[0xD] = true;
            }
            KeyCode::A => {
                self.chip8.key[7] = true;
            }
            KeyCode::S => {
                self.chip8.key[8] = true;
            }
            KeyCode::D => {
                self.chip8.key[9] = true;
            }
            KeyCode::F => {
                self.chip8.key[0xE] = true;
            }
            KeyCode::Z => {
                self.chip8.key[0xA] = true;
            }
            KeyCode::X => {
                self.chip8.key[0] = true;
            }
            KeyCode::C => {
                self.chip8.key[0xB] = true;
            }
            KeyCode::V => {
                self.chip8.key[0xF] = true;
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
    // checks if a key as been released
    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
    )
    {
        match keycode {
            KeyCode::Key1 => {
                self.chip8.key[1] = false;
            }
            KeyCode::Key2 => {
                self.chip8.key[2] = false;
            }
            KeyCode::Key3 => {
                self.chip8.key[3] = false;
            }
            KeyCode::Key4 => {
                self.chip8.key[0xC] = false;
            }
            KeyCode::Q => {
                self.chip8.key[4] = false;
            }
            KeyCode::W => {
                self.chip8.key[5] = false;
            }
            KeyCode::E => {
                self.chip8.key[6] = false;
            }
            KeyCode::R => {
                self.chip8.key[0xD] = false;
            }
            KeyCode::A => {
                self.chip8.key[7] = false;
            }
            KeyCode::S => {
                self.chip8.key[8] = false;
            }
            KeyCode::D => {
                self.chip8.key[9] = false;
            }
            KeyCode::F => {
                self.chip8.key[0xE] = false;
            }
            KeyCode::Z => {
                self.chip8.key[0xA] = false;
            }
            KeyCode::X => {
                self.chip8.key[0] = false;
            }
            KeyCode::C => {
                self.chip8.key[0xB] = false;
            }
            KeyCode::V => {
                self.chip8.key[0xF] = false;
            }
            _ => (),
        }
    }
}
fn main() {
    // getting the rom path from cmd
    let path_rom = std::env::args().nth(1).expect("no rom given");

    // window configuration
    let (mut ctx, mut event_loop) = ContextBuilder::new("CHIP-8", "Vinicius Tikara")
        .window_setup(ggez::conf::WindowSetup::default().title("CHIP-8"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions((WIDTH * ZOOM) as f32, (HEIGHT * ZOOM) as f32),
        )
        .build()
        .unwrap();
    // initialization of the emulator
    let emulator = &mut Emulator::new();
    emulator.chip8.load_rom(&path_rom);
    emulator.chip8.load_font();

    // main loop
    match event::run(&mut ctx, &mut event_loop, emulator) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

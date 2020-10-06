use ::ggez::*;
use ggez::event::{KeyCode};
use std::thread;
use std::time::{Duration, Instant};

mod cpu;
mod input;

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
            // maybe redundant??
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
            final_mesh = screen_mesh.build(ctx)?;
            graphics::draw(ctx, &final_mesh, graphics::DrawParam::default())?;
            graphics::present(ctx)?;
            self.chip8.update_screen = false;
            Ok(())
        } else {
            Ok(())
        }
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // this equals to 250Hz
        let milli = Duration::from_millis(2);
        let sixty_hz = Duration::from_millis(16);
        let now = Instant::now();
        
        self.chip8.get_opcode();
        self.chip8.decode_opcode();
        self.acc_timer += now.elapsed();
        if self.acc_timer >= sixty_hz && self.chip8.delay_timer != 0 {
            self.acc_timer -= sixty_hz;
            self.chip8.delay_timer -= 1;
        }
        thread::sleep(milli - now.elapsed());
        self.acc_timer += milli;
        Ok(())
    }

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
                self.chip8.key[0] = true;
            }
            KeyCode::Key2 => {
                self.chip8.key[1] = true;
            }
            KeyCode::Key3 => {
                self.chip8.key[2] = true;
            }
            KeyCode::Q => {
                self.chip8.key[3] = true;
            }
            KeyCode::W  => {
                self.chip8.key[4] = true;
            }
            KeyCode::E => {
                self.chip8.key[5] = true;
            }
            KeyCode::A => {
                self.chip8.key[6] = true;
            }
            KeyCode::S => {
                self.chip8.key[7] = true;
            }
            KeyCode::D => {
                self.chip8.key[8] = true;
            }
            KeyCode::Z => {
                self.chip8.key[9] = true;
            }
            KeyCode::X => {
                self.chip8.key[10] = true;
            }
            KeyCode::C => {
                self.chip8.key[11] = true;
            }
            KeyCode::Key4 => {
                self.chip8.key[12] = true;
            }
            KeyCode::R => {
                self.chip8.key[13] = true;
            }
            KeyCode::F => {
                self.chip8.key[14] = true;
            }
            KeyCode::V => {
                self.chip8.key[15] = true;
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
    )
    {
        match keycode {
            KeyCode::Key1 => {
                self.chip8.key[0] = false;
            }
            KeyCode::Key2 => {
                self.chip8.key[1] = false;
            }
            KeyCode::Key3 => {
                self.chip8.key[2] = false;
            }
            KeyCode::Q => {
                self.chip8.key[3] = false;
            }
            KeyCode::W  => {
                self.chip8.key[4] = false;
            }
            KeyCode::E => {
                self.chip8.key[5] = false;
            }
            KeyCode::A => {
                self.chip8.key[6] = false;
            }
            KeyCode::S => {
                self.chip8.key[7] = false;
            }
            KeyCode::D => {
                self.chip8.key[8] = false;
            }
            KeyCode::Z => {
                self.chip8.key[9] = false;
            }
            KeyCode::X => {
                self.chip8.key[10] = false;
            }
            KeyCode::C => {
                self.chip8.key[11] = false;
            }
            KeyCode::Key4 => {
                self.chip8.key[12] = false;
            }
            KeyCode::R => {
                self.chip8.key[13] = false;
            }
            KeyCode::F => {
                self.chip8.key[14] = false;
            }
            KeyCode::V => {
                self.chip8.key[15] = false;
            }
            _ => (),
        }
    }
}
fn main() {
    // getting the rom path from cmd
    let path_rom = std::env::args().nth(1).expect("no rom given");

    let (mut ctx, mut event_loop) = ContextBuilder::new("CHIP-8", "x")
        .window_setup(ggez::conf::WindowSetup::default().title("CHIP-8"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions((WIDTH * ZOOM) as f32, (HEIGHT * ZOOM) as f32),
        )
        .build()
        .unwrap();

    let emulator = &mut Emulator::new();
    emulator.chip8.load_rom(&path_rom);
    emulator.chip8.load_font();

    // main loop
    match event::run(&mut ctx, &mut event_loop, emulator) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

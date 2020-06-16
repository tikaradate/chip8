use std::fs;


pub struct Chip8{
    opcode: usize,
    memory: [usize; 4096],
    reg: [usize; 16],
    pc: usize,
    I: usize,
    delay_timer: usize,
    sound_timer: usize,
    stack: [usize; 16],
    sp: usize,
}

impl Chip8{
    //fonts are loaded starting at this address
    const FONT_ADDR: usize = 0x50;
    //the pc starts at this address
    const START_ADDR: usize = 0x200;
    const FONT: [usize; 80] = [
        	0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        	0x20, 0x60, 0x20, 0x20, 0x70, // 1
        	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        	0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
    fn init() -> Self{
        
        //set all memory to 0's
        let mut memory = [0; 4096];
        let mut k = 0; 
        for i in Chip8::FONT.iter(){
            memory[Chip8::FONT_ADDR + k] = *i;
            k += 1;
        }
        Self{
        //TODO: arrange it like the struct
            pc: Chip8::START_ADDR,
            stack: [0; 16],
            opcode: 0,
            I: 0,
            sp: 0,
            memory,
            reg: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn load_font(&mut self){
        for i in 0..Chip8::FONT.len(){
            self.memory[i+Chip8::FONT_ADDR] = Chip8::FONT[i];
        }
        
    }

    fn load_rom(&mut self, path: &str){
        let rom = fs::read(path)
                    .expect("Unable to read file");

        for i in 0..rom.len(){
            self.memory[i+Chip8::START_ADDR] = rom[i].into();
        }
    }

}

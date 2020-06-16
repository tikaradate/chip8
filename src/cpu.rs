
struct chip8{
    opcode: u16;
    memory: [u8, 4096];
    reg: [u8, 16];
    pc: u16;
    I: u16;
    delay_timer: u8;
    sound_timer: u8;
    stack: [u16, 16];
    sp: u16;
}

impl chip8{
    //fonts are loaded starting at this address
    const FONT_ADDR: usize = 0x50;
    //the pc starts at this address
    const STRT_ADDR: usize = 0x200;
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
        let memory = [0; 4096];
        let Self{
        //TODO: arrange like the struct
            pc = chip8::FONT_ADDR,
            stack = [0; 16],
            opcode = 0,
            I = 0,
            sp = 0,
            memory = Self.load_font(),
            reg = [0; 16],
            delay_timer = 0,
            sound_timer = 0
        }
    }

    fn load_font(&mut self){
        for i in chip8::FONT.len(){
            self.memory[i+chip8::FONT_ADDR] = chip::FONT[i];
        }
        
    }

    fn load_rom(&mut self, path: &str){
        //TODO: how to read files
        for i in rom.len(){
            self.memory[i+chip8::START_ADDR] = rom[i];
        }
    }



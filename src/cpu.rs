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

    fn get_opcode(&mut self){
        let high = self.memory[self.pc];
        let low = self.memory[self.pc + 1];
        self.opcode = (high << 8) | low;
    }

    fn decode_opcode(&mut self){
        let cod1 = (opcode & 0xF000) >> 12;
        let cod2 = (opcode & 0x0F00) >> 8;
        let cod3 = (opcode & 0x00F0) >> 4;
        let cod4 = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let nn = opcode & 0x00FF;
        let vx = cod2;
        let vy = cod2;

        match(cod1, cod2, cod3, cod4){
            (0, 0, 0xE, 0) => self.clear_scr(),
            (0, 0, 0xE, 0xE) => self.ret_from_sub(),
            (1, _, _, _) => self.goto(nnn),
            (2, _, _, _) => self.call(nnn),
            (3, _, _, _) => self.ieq_const(vx, nn),
            (4, _, _, _) => self.neq_const(vx, nn),
            (5, _, _, 0) => self.ieq(vx, vy),
            (6, _, _, _) => self.set_vx_const(vx ,nn),
            (7, _, _, _) => self.adds_const(vx, nn),
            (8, _, _, 0) => self.set_vx_vy(vx, vy),
            (8, _, _, 1) => self.set_or_vxvy(vx, vy),
            (8, _, _, 2) => self.set_and_vxvy(vx, vy),
            (8, _, _, 3) => self.set_xor_vxvy(vx, vy),
            (8, _, _, 4) => self.adds_vy(vx, vy),
            (8, _, _, 5) => self.subs_vy(vx, vy),
            (8, _, _, 6) => self.shift_r1(vx, vy),
            (8, _, _, 7) => self.subs_vxvy(vx, vy),
            (8, _, _, 0xE) => self.shif_l1(vx, vy),
            (9, _, _, 0) => self.neq(vx, vy),
            (0xA, _, _, _) => self.set_I(nnn),
            (0xB, _, _, _) => self.jump_v0(nnn),
            (0xC, _, _, _) => self.rand(vx, nn),
            (0xD, _, _, _) => self.draw(vx, vy, n),
            (0xE, _, 9, 0xE) => self.ieq_key(vx),
            (0xE, _, 0xA, 1) => self.neq_key(vx),
            (0xF, _, 0, 7) => self.get_delay(vx),
            (0xF, _, 0, 0xA) => self.get_key(vx),
            (0xF, _, 1, 5) => self.set_delay(vx),
            (0xF, _, 1, 8) => self.set_sound(vx),
            (0xF, _, 1, 0xE) => self.add_I_vx(vx),
            (0xF, _, 2, 9) => self.set_I_sprite(vx),
            (0xF, _, 3, 3) => self.set_BCD(vx),
            (0xF, _, 5, 5) => self.store_regs_mem(vx),
            (0xF, _, 6, 5) => self.load_regs_mem(vx),
            (_, _, _, _,) => panic!("opcode {:?} not found", self.opcode),
        };

    }

    fn clear_scr(&mut self){
        
    }

    fn ret_from_sub(&mut self){
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    fn goto(&mut self, nnn:usize){
        self.pc = nnn;
    }

    fn call(&mut self, nnn:usize){
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn ieq_const(&mut self, vx:usize, nn:usize){
        if self.reg[vx] == nn{
            self.pc += self.OPCODE_SIZE;
        }
    }
}

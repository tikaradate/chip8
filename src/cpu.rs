use rand::Rng;
use std::fs;

pub struct Chip8 {
    pub opcode: usize,
    memory: [usize; 4096],
    reg: [usize; 16],
    pc: usize,
    index: usize,
    pub delay_timer: usize,
    pub sound_timer: usize,
    stack: [usize; 16],
    sp: usize,
    pub key: [bool; 16],
    pub gfx: [bool; 64 * 32],
    // internal flag
    pub update_screen: bool,
}

impl Chip8 {
    // fonts are loaded starting at this address
    const FONT_ADDR: usize = 0x050;
    // the pc starts at this address
    const START_ADDR: usize = 0x200;
    // the size of a opcode, used in some contexts
    const OPCODE_SIZE: usize = 2;
    // each member of the font is drawed line by line
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

    pub fn init() -> Self {
        //set all memory to 0's
        let mut memory = [0; 4096];

        // load the font into memory
        let mut k = 0;
        for i in Chip8::FONT.iter() {
            memory[Chip8::FONT_ADDR + k] = *i;
            k += 1;
        }
        // initiliazes the struct
        Self {
            opcode: 0,
            memory,
            reg: [0; 16],
            pc: Chip8::START_ADDR,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [false; 16],
            gfx: [false; 64 * 32],
            update_screen: false,
        }
    }
    // loads the font starting from a defined offset
    pub fn load_font(&mut self) {
        for i in 0..Chip8::FONT.len() {
            self.memory[i + Chip8::FONT_ADDR] = Chip8::FONT[i];
        }
    }
    // loads the rom(if possible) starting from a defined offset
    pub fn load_rom(&mut self, path: &str) {
        let rom = fs::read(path).expect("Unable to read file");

        for i in 0..rom.len() {
            self.memory[i + Chip8::START_ADDR] = rom[i].into();
        }
    }

    // reads the opcode pointed by PC
    pub fn get_opcode(&mut self) {
        let high = self.memory[self.pc];
        let low = self.memory[self.pc + 1];
        self.opcode = ((high) << 8) | low;
    }
    // decodes the opcode and calls the correct function
    pub fn decode_opcode(&mut self) {
        let opcode = self.opcode;
        //println!("opcode: {:x}", opcode);
        let cod1 = (opcode & 0xF000) >> 12;
        let cod2 = (opcode & 0x0F00) >> 8;
        let cod3 = (opcode & 0x00F0) >> 4;
        let cod4 = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let nn = opcode & 0x00FF;
        let n = opcode & 0x000F;
        let vx = cod2;
        let vy = cod3;

        match (cod1, cod2, cod3, cod4) {
            (0, 0, 0xE, 0) => self.clear_scr(),
            (0, 0, 0xE, 0xE) => self.ret_from_sub(),
            (1, _, _, _) => self.goto(nnn),
            (2, _, _, _) => self.call(nnn),
            (3, _, _, _) => self.ieq_const(vx, nn),
            (4, _, _, _) => self.neq_const(vx, nn),
            (5, _, _, 0) => self.ieq(vx, vy),
            (6, _, _, _) => self.set_vx_const(vx, nn),
            (7, _, _, _) => self.adds_const(vx, nn),
            (8, _, _, 0) => self.set_vx_vy(vx, vy),
            (8, _, _, 1) => self.set_or_vx_vy(vx, vy),
            (8, _, _, 2) => self.set_and_vx_vy(vx, vy),
            (8, _, _, 3) => self.set_xor_vx_vy(vx, vy),
            (8, _, _, 4) => self.adds_vx_vy(vx, vy),
            (8, _, _, 5) => self.subs_vx_vy(vx, vy),
            (8, _, _, 6) => self.shift_r1(vx),
            (8, _, _, 7) => self.subs_vy_vx(vx, vy),
            (8, _, _, 0xE) => self.shift_l1(vx),
            (9, _, _, 0) => self.neq(vx, vy),
            (0xA, _, _, _) => self.set_i(nnn),
            (0xB, _, _, _) => self.jump_v0(nnn),
            (0xC, _, _, _) => self.random(vx, nn),
            (0xD, _, _, _) => self.draw(vx, vy, n),
            (0xE, _, 9, 0xE) => self.ieq_key(vx),
            (0xE, _, 0xA, 1) => self.neq_key(vx),
            (0xF, _, 0, 7) => self.get_delay(vx),
            (0xF, _, 0, 0xA) => self.get_key(vx),
            (0xF, _, 1, 5) => self.set_delay(vx),
            (0xF, _, 1, 8) => self.set_sound(vx),
            (0xF, _, 1, 0xE) => self.add_i_vx(vx),
            (0xF, _, 2, 9) => self.set_i_sprite(vx),
            (0xF, _, 3, 3) => self.set_bcd(vx),
            (0xF, _, 5, 5) => self.store_regs_mem(vx),
            (0xF, _, 6, 5) => self.load_regs_mem(vx),
            (_, _, _, _) => panic!("opcode {:?} not found", self.opcode),
        };
        if cod1 != 1 && cod1 != 2 && cod1 != 0xB {
            self.pc += Chip8::OPCODE_SIZE;
        }
    }
    // 00E0
    // set the screen to black
    fn clear_scr(&mut self) {
        self.gfx = [false; 64 * 32];
        self.update_screen = true;
    }
    // 00EE
    // return from function
    fn ret_from_sub(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }
    // 1NNN
    // unconditional jump
    fn goto(&mut self, nnn: usize) {
        self.pc = nnn;
    }
    // 2NNN
    // function calling
    fn call(&mut self, nnn: usize) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }
    // 3XNN
    // if VX is equal to NN, skip the next instruction
    fn ieq_const(&mut self, vx: usize, nn: usize) {
        if self.reg[vx] == nn {
            self.pc += Chip8::OPCODE_SIZE;
        }
    }
    // 4XNN
    // if VX isn't equal to NN, skip the next instruction
    fn neq_const(&mut self, vx: usize, nn: usize) {
        if self.reg[vx] != nn {
            self.pc += Chip8::OPCODE_SIZE;
        }
    }
    // 5XY0
    // if VX is equal to VY, skip the next instruction
    fn ieq(&mut self, vx: usize, vy: usize) {
        if self.reg[vx] == self.reg[vy] {
            self.pc += Chip8::OPCODE_SIZE;
        }
    }
    // 6XNN
    // set VX to NN
    fn set_vx_const(&mut self, vx: usize, nn: usize) {
        self.reg[vx] = nn;
    }
    // 7XNN
    // adds NN to VX
    fn adds_const(&mut self, vx: usize, nn: usize) {
        self.reg[vx] = (self.reg[vx] + nn) % 0x100;
    }
    // 8XY0
    // set VX to VY
    fn set_vx_vy(&mut self, vx: usize, vy: usize) {
        self.reg[vx] = self.reg[vy];
    }
    // 8XY1
    // set VX to VX bitswise-OR VY
    fn set_or_vx_vy(&mut self, vx: usize, vy: usize) {
        self.reg[vx] = self.reg[vx] | self.reg[vy];
    }
    // 8XY2
    // set VX to VX bitswise-AND VY
    fn set_and_vx_vy(&mut self, vx: usize, vy: usize) {
        self.reg[vx] = self.reg[vx] & self.reg[vy];
    }
    // 8XY3
    // set VX to VX bitswise-XOR VY
    fn set_xor_vx_vy(&mut self, vx: usize, vy: usize) {
        self.reg[vx] = self.reg[vx] ^ self.reg[vy];
    }
    // 8XY4
    // set VX to VX added to VY and,
    // if needed, setting VF to the carry flag
    fn adds_vx_vy(&mut self, vx: usize, vy: usize) {
        if self.reg[vx] + self.reg[vy] > 0xFF {
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }
        self.reg[vx] = (self.reg[vx] + self.reg[vy]) % 0x100;
    }
    // 8XY5
    // set VX to VY subtracted from VX and, if needed,
    // setting VF to the borrow flag
    fn subs_vx_vy(&mut self, vx: usize, vy: usize) {
        if self.reg[vx] > self.reg[vy] {
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }
        self.reg[vx] = self.reg[vx].wrapping_sub(self.reg[vy]);
    }
    // 8XY6
    // stores the least significant bit of VX in VF and
    // then shifts VX to the right by 1
    fn shift_r1(&mut self, vx: usize) {
        // bitmask to get the least sig. bit
        self.reg[0xF] = self.reg[vx] & 0x1;
        self.reg[vx] = self.reg[vx] >> 1;
    }
    // 8XY7
    // set VX to VX subtracted from VY and, if needed,
    // setting VF to the borrow flag
    fn subs_vy_vx(&mut self, vx: usize, vy: usize) {
        if self.reg[vy] > self.reg[vx] {
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }
        self.reg[vx] = self.reg[vy].wrapping_sub(self.reg[vx]);
    }
    // 8XYE
    // stores the most significant bit of VX in VF and
    // then shifts VX to the left by 1
    fn shift_l1(&mut self, vx: usize) {
        // uses a bitmask to get the most sig. bit
        // then pushing it to the end
        self.reg[0xF] = (self.reg[vx] & 0x80) >> 7;
        self.reg[vx] = self.reg[vx] << 1;
    }
    // 9XY0
    // if VX is not equal to VY, skip the next instruction
    fn neq(&mut self, vx: usize, vy: usize) {
        if self.reg[vx] != self.reg[vy] {
            self.pc += Chip8::OPCODE_SIZE;
        }
    }
    // ANNN
    // set I to the adress NNN
    fn set_i(&mut self, nnn: usize) {
        self.index = nnn;
    }
    // BNNN
    // sets pc to V0 + NNN
    fn jump_v0(&mut self, nnn: usize) {
        self.pc = self.reg[0] + nnn;
    }
    // CXNN
    // sets VX to rand() bitwise-and NNN
    fn random(&mut self, vx: usize, nn: usize) {
        let mut rng = rand::thread_rng();
        self.reg[vx] = rng.gen_range(0x00, 0xFE) & nn;
    }
    // DXYN
    // draw a sprite at the coordinates VX, VY, with
    // the data starting at I
    fn draw(&mut self, vx: usize, vy: usize, n: usize) {
        // iterates over the height/rows
        let x = self.reg[vx];
        let y = self.reg[vy];
        self.reg[0xF] = 0;
        for yline in 0..n {
            let pixel = self.memory[self.index + yline];
            // iterates collumn by collumn(fixed size of 8)
            for xline in 0..8 {
                if (pixel & (0x80 >> xline)) != 0 {
                    if self.gfx[((x + xline)%64 + ((y + yline)%32 * 64))] == true {
                        self.reg[0xF] = 1;
                    }
                    self.gfx[((x + xline)%64 + ((y + yline)%32 * 64))] ^= true;
                }
            }
        }
        self.update_screen = true;
    }
    // EX9E
    // if VX is equal to the key, skip the next instruction
    fn ieq_key(&mut self, vx: usize) {
        if self.key[self.reg[vx]] == true {
            self.pc += Chip8::OPCODE_SIZE;
        }
    }
    // EXA1
    // if VX is not equal to the key, skip the next instruction
    fn neq_key(&mut self, vx: usize) {
        if self.key[self.reg[vx]] == false {
            self.pc += Chip8::OPCODE_SIZE;
        }
    }
    // FX07
    // set the vx to the delay timer
    fn get_delay(&mut self, vx: usize) {
        self.reg[vx] = self.delay_timer;
    }
    // FX0A
    // waits for a key to get pressed
    fn get_key(&mut self, vx: usize) {
        let mut pressed = false;
        while !pressed {
            for i in 0..self.key.len() {
                if self.key[i] == true {
                    pressed = true;
                    self.reg[vx] = i;
                    break;
                }
            }
        }
    }
    // FX15
    // set the delay timer to the value of VX
    fn set_delay(&mut self, vx: usize) {
        self.delay_timer = self.reg[vx];
    }
    // FX18
    // set the sound timer to the value of VX
    fn set_sound(&mut self, vx: usize) {
        self.sound_timer = self.reg[vx];
    }
    // FX1E
    // sets I to VX added to I
    fn add_i_vx(&mut self, vx: usize) {
        self.index += self.reg[vx];
    }
    // FX29
    // sets I to the spr_addr added to VX
    fn set_i_sprite(&mut self, vx: usize) {
        self.index = Chip8::FONT_ADDR + self.reg[vx] * 5;
    }
    // FX33
    // gets the BCD of VX and sets I to the hundreds place,
    // I + 1 to the tens, and I + 2 to the ones
    fn set_bcd(&mut self, vx: usize) {
        self.memory[self.index] = self.reg[vx] / 100;
        self.memory[self.index + 1] = (self.reg[vx] % 100) / 10;
        self.memory[self.index + 2] = self.reg[vx] % 10;
    }
    // FX55
    // store the value from all registers starting at the address I
    fn store_regs_mem(&mut self, vx: usize) {
        for i in 0..vx + 1 {
            self.memory[self.index + i] = self.reg[i];
        }
    }
    // FX65
    // loads values in all registers starting at the address I
    fn load_regs_mem(&mut self, vx: usize) {
        for i in 0..vx + 1 {
            self.reg[i] = self.memory[self.index + i];
        }
    }
}

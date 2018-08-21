
pub struct Vm {
    mem: [u8;0xfff],
    pc: u16,
    reg: [u8;16],
    reg_i: u16,
    reg_sp: u16,
    stack: [u16;16],
    reg_dt:u8,
    reg_st:u8,
    screen: [u8; ::SCREEN_SIZE],
    key:u8
}

impl Vm {

    pub fn new() -> Vm {
        Vm {
            mem: [0;0xfff],
            pc: 0x200,
            reg: [0;16],
            reg_i: 0,
            reg_sp: 0,
            stack: [0;16],
            reg_dt: 0,
            reg_st: 0,
            screen: [0;::SCREEN_SIZE],
            key:0
        }
    }

    pub fn cycle(&mut self) {
        self.decode_instruction();
    }

    fn update_timers() {
    }

    fn decode_instruction(&mut self) {

            let instr1 = self.mem[self.pc as usize];
            let instr2 = self.mem[self.pc as usize +1];

            let x = (instr1 & 0x0f) as usize;
            let y = (instr2 >> 4) as usize;
            let addr = (x as u16)<<8 | instr2 as u16 ;
            let kk = instr2;
            let n = instr2 &0x0f;

            match instr1 & 0xf0 {
                0x00 => {
                    match instr2 {
                        0xE0 => self.screen = [0; ::SCREEN_SIZE], // CLS
                        0xEE => { // RET
                            self.pc = self.stack[self.reg_sp as usize];
                            self.reg_sp-=1;
                        },
                        _ => self.unknown_opcode()
                    }
                },
                0x10 => { // 1nnn - JMP addr
                    self.pc = addr; 
                    return;
                },
                0x20 => {
                    // 2nnn - CALL addr 
                    self.stack[self.reg_sp as usize] = self.pc ;
                    self.reg_sp+=1;
                    return;
                },
                0x30 => { 
                    // 3xkk - SE Vx, byte 
                    if self.reg[x] == kk { self.pc+=2; }
                },
                0x40 => { 
                    // 4xkk - SNE Vx, byte 
                    if self.reg[x] != kk { self.pc+=2; }
                },
                0x50 => { 
                    // 5xkk - SE Vx, Vy 
                    if self.reg[x] != self.reg[y] { self.pc+=2; }
                },
                0x60 => {
                    // 6xkk - LD Vx, byte
                    self.reg[x] = kk;
                },
                0x70 => {
                    // 7xkk - ADD Vx, byte
                    self.reg[x] += kk;
                },
                0x80 => match instr2 & 0x0f {
                    0x00 => self.reg[x] = self.reg[y] ,  // 8xy0 - LD Vx, Vy
                    0x01 => self.reg[x] |= self.reg[y] , // 8xy1 - OR Vx, Vy
                    0x02 => self.reg[x] &= self.reg[y] ,// 8xy2 - AND Vx, Vy
                    0x03 => self.reg[x] ^= self.reg[y] , // 8xy3 - XOR Vx, Vy
                    0x04 => { // 8xy4 - ADD Vx, Vy
                        self.reg[x] += self.reg[y];
                        //self.reg[0x0f] = (x+y)>0xff ; 
                    } ,
                    0x05 => { // 8xy5 - SUB Vx, Vy
                        self.reg[x] -= self.reg[y];
                        //self.reg[0x0f] =  y>x ; 
                    } ,
                    0x06 => {  // 8xy6 - SHR Vx {, Vy} 
                        self.reg[x] >>=1;
                        //self.reg[0x0f] = x&0x01 ;
                    },
                    0x07 => { // 7 - SUBN Vx, Vy
                        self.reg[x] = self.reg[y] - self.reg[x];
                        //self.reg[0x0f] =  x>y ; 
                    } ,
                    0x0e => {  // 8xy6 - SHL Vx {, Vy} 
                        self.reg[x] <<=1;
                        //self.reg[0x0f] = x&0x80 ;
                    },
                    _ => self.unknown_opcode()
                     
                },
                0x90 => {},
                0xa0 => self.reg_i = addr, // LD I, addr
                0xb0 => { // JP V0, addr
                    self.pc = addr + self.reg[0] as u16;
                    return;
                },
                0xc0 => {},
                0xd0 => self.draw_sprite(self.reg[x], self.reg[y], n), // DRW Vx,Vy,nibble
                0xe0 => {},
                0xf0 => match instr2 {
                    0x07 => self.reg[x] = self.reg_dt , // LD Vx, DT
                    0x0a => {
                        // LD Vx,K
                        if self.key == 0 {return ;}
                        self.reg[x] = self.key;
                    },
                    0x15 => self.reg_dt = self.reg[x], // LD DT, Vx
                    0x18 => self.reg_st = self.reg[x], // LD ST, Vx
                    0x1e => self.reg_i += self.reg[x] as u16, // ADD I,Vx
                    0x29 => self.reg_i = self.reg[x] as u16 * 6, // LD F, Vx 
                    0x33 => {},
                    0x65 => {},
                    _ => self.unknown_opcode()

                },
               _ => self.unknown_opcode()
            }

            self.pc+=2;
    }

    fn draw_sprite(&self, _x:u8, _y:u8, _nibble:u8) {

    }

    fn unknown_opcode(&self) {

        panic!("Unknwown opcode !");
    }


}
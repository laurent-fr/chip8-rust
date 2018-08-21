
pub struct Vm {
    mem: [u8;0xfff],
    pc: u16,
    reg: [u8;16],
    reg_i: u16,
    reg_sp: u16,
    stack: [u16;16],
    reg_dt:u8,
    reg_st:u8,
    screen: [u8;(32*64)]
}

impl Vm {

    pub fn new() -> Vm {
        Vm {
            mem: [0;0xfff],
            pc: 0,
            reg: [0;16],
            reg_i: 0,
            reg_sp: 0,
            stack: [0;16],
            reg_dt: 0,
            reg_st: 0,
            screen: [0;(32*64)]
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

            let x = instr1 & 0x0f;
            let y = instr2 >> 4 ;
            let addr = (x as u16)<<8 | instr2 as u16 ;
            let kk = instr2;
            
            match instr1 & 0xf0 {
                0x00 => {
                    match instr2 {
                        0xE0 => {}, // CLS
                        0xEE => { // RET
                            self.pc = self.stack[self.reg_sp as usize];
                            self.reg_sp-=1;
                        },
                        _ => {
                            panic!("Unknonn opcode !");
                        }
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
                    if self.reg[x as usize] == kk { self.pc+=2; }
                },
                0x40 => { 
                    // 4xkk - SNE Vx, byte 
                    if self.reg[x as usize] != kk { self.pc+=2; }
                },
                0x50 => { 
                    // 5xkk - SE Vx, Vy 
                    if self.reg[x as usize] != self.reg[y as usize] { self.pc+=2; }
                },
                0x60 => {
                    // 6xkk - LD Vx, byte
                    self.reg[x as usize] = kk;
                },
                0x70 => {
                    // 7xkk - ADD Vx, byte
                    self.reg[x as usize] += kk;
                },
                0x80 => match instr2 & 0x0f {
                    0x00 => self.reg[x as usize] = self.reg[y as usize] ,  // 8xy0 - LD Vx, Vy
                    0x01 => self.reg[x as usize] |= self.reg[y as usize] , // 8xy1 - OR Vx, Vy
                    0x02 => self.reg[x as usize] &= self.reg[y as usize] ,// 8xy2 - AND Vx, Vy
                    0x03 => self.reg[x as usize] ^= self.reg[y as usize] , // 8xy3 - XOR Vx, Vy
                    0x04 => { // 8xy4 - ADD Vx, Vy
                        self.reg[x as usize] += self.reg[y as usize];
                        //self.reg[0x0f] = (x+y)>0xff ; 
                    } ,
                    0x05 => { // 8xy5 - SUB Vx, Vy
                        self.reg[x as usize] -= self.reg[y as usize];
                        //self.reg[0x0f] =  y>x ; 
                    } ,
                    0x06 => {  // 8xy6 - SHR Vx {, Vy} 
                        self.reg[x as usize] >>=1;
                        //self.reg[0x0f] = x&0x01 ;
                    },
                    0x07 => { // 7 - SUBN Vx, Vy
                        self.reg[x as usize] = self.reg[y as usize] - self.reg[x as usize];
                        //self.reg[0x0f] =  x>y ; 
                    } ,
                    0x0e => {  // 8xy6 - SHL Vx {, Vy} 
                        self.reg[x as usize] <<=1;
                        //self.reg[0x0f] = x&0x80 ;
                    },
                    _ => {
                            panic!("Unknown opcode !");
                        }
                     
                },
                0x90 => {},
                0xa0 => {},
                0xb0 => {},
                0xc0 => {},
                0xd0 => {},
                0xe0 => {},
                0xf0 => {},
                _ => {
                            panic!("Unknonn opcode !");
                        }
            }

            self.pc+=2;
    }

    fn draw_sprite(x:u8, y:u8) {

    }


}
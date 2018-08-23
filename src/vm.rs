extern crate rand;

use rand::prelude::*;

pub struct Vm {
    mem: [u8;0xfff],
    pc: u16,
    reg: [u8;16],
    reg_i: u16,
    reg_sp: u16,
    stack: [u16;16],
    reg_dt:u8,
    reg_st:u8,
    pub screen: [u8; ::SCREEN_SIZE],
    pub key:i8
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
            key: -1
        }
    }

    
    pub fn update_timers(&mut self) {
        if self.reg_dt>0 {
            self.reg_dt-=1;
        }

        if self.reg_st>0 {
            self.reg_st-=1;
        }
    }

    pub fn cycle(&mut self) -> bool {

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
                        0xE0 => {
                            self.screen = [0; ::SCREEN_SIZE]; // CLS
                            self.pc+=2;
                            return true;
                        },
                        0xEE => { // RET
                            self.pc = self.stack[self.reg_sp as usize];
                            self.reg_sp-=1;
                        },
                        _ => self.unknown_opcode()
                    }
                },
                0x10 => { // 1nnn - JMP addr
                    self.pc = addr; 
                    return false;
                },
                0x20 => {
                    // 2nnn - CALL addr 
                    self.stack[self.reg_sp as usize] = self.pc ;
                    self.reg_sp+=1;
                    return false;
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
                        self.reg[0x0f] = if (self.reg[x] as u32 + self.reg[y] as u32)>0xff { 1 } else { 0 } ;
                        self.reg[x] += self.reg[y];
                    } ,
                    0x05 => { // 8xy5 - SUB Vx, Vy
                        self.reg[0x0f] =  if self.reg[x]>self.reg[y] { 1 } else { 0 }; 
                        self.reg[x] -= self.reg[y];
                        
                    } ,
                    0x06 => {  // 8xy6 - SHR Vx {, Vy} 
                        self.reg[0x0f] = if self.reg[x]&0x01 == 1 { 1 } else { 0 } ;
                        self.reg[x] >>=1;
                        
                    },
                    0x07 => { // 8xy7 - SUBN Vx, Vy
                        self.reg[0x0f] =  if self.reg[y]>self.reg[x] { 1 } else { 0 };
                        self.reg[x] = self.reg[y] - self.reg[x];
                    } ,
                    0x0e => {  // 8xy6 - SHL Vx {, Vy} 
                        self.reg[0x0f] = if self.reg[x]&0x80 == 0x80 { 1 } else { 0 } ;
                        self.reg[x] <<=1;
                    },
                    _ => self.unknown_opcode()
                     
                },
                0x90 => {
                    // 9xy0 - SNE Vx, Vy
                    if self.reg[x] != self.reg[y] {
                        self.pc+=2;
                    }
                },
                0xa0 => self.reg_i = addr, // LD I, addr
                0xb0 => { // JP V0, addr
                    self.pc = addr + self.reg[0] as u16;
                    return false;
                },
                0xc0 => {
                    // Cxkk - RND Vx, byte
                    let rnd:u8 = random();
                    self.reg[x] = rnd & kk ;
                },
                0xd0 => { 
                    self.draw_sprite(x, y, n); // DRW Vx,Vy,nibble
                    self.pc +=2;
                    return true;
                },
                0xe0 => {
                    // Ex9E - SKP Vx
                    if self.key as i32 == self.reg[x] as i32 {
                        self.pc+=2;
                    }
                },
                0xf0 => match instr2 {
                    0x07 => self.reg[x] = self.reg_dt , // LD Vx, DT
                    0x0a => {
                        // LD Vx,K
                        if self.key == -1 {return false;}
                        self.reg[x] = self.key as u8;
                    },
                    0x15 => self.reg_dt = self.reg[x], // LD DT, Vx
                    0x18 => self.reg_st = self.reg[x], // LD ST, Vx
                    0x1e => self.reg_i += self.reg[x] as u16, // ADD I,Vx
                    0x29 => self.reg_i = self.reg[x] as u16 * 6, // LD F, Vx 
                    0x33 => { 
                        // Fx33 - LD B, Vx
                        let mut number = self.reg[x];
                        self.reg[self.reg_i as usize + 2 ] = number % 10;
                        number /=10;
                        self.reg[self.reg_i as usize + 1 ] = number % 10;
                        number /=10;
                        self.reg[self.reg_i as usize  ] = number % 10;
                         
                    },
                    0x55 => {
                        // Fx55 - LD [I], Vx
                        for i in 0..x as usize {
                            self.mem[self.reg_i as usize + i ] = self.reg[i];
                        }
                    },
                    0x65 => {
                        // Fx65 - LD Vx, [I]
                         for i in 0..x as usize {
                            self.reg[i] = self.mem[self.reg_i as usize + i ] ;
                        }
                    },
                    _ => self.unknown_opcode()

                },
               _ => self.unknown_opcode()
            }

            self.pc+=2;

            return false;
    }

    fn draw_sprite(&mut self, x_idx:usize, y_idx:usize, nibble:u8) {
        
            let x = self.reg[x_idx] as u32;
            let y = self.reg[y_idx] as u32;
            self.reg[0x0f] = 0;

            for line in 0..nibble as u32 {
            
                let mut line_wrap = (y+line) % ::HEIGHT;
                let x_byte = x/8;
                let addr1 = line_wrap*::WIDTH_BYTE + x_byte;;
                let addr2 = line_wrap*::WIDTH_BYTE + (x_byte+1) & (::WIDTH_BYTE -1) ;

                let old_byte1 = self.screen[addr1 as usize];
                let new_byte1 = self.mem[(self.reg_i+nibble as u16) as usize] >> (x%8);
            
                self.screen[addr1 as usize] ^= new_byte1;
                
                if (old_byte1|new_byte1) != (old_byte1^new_byte1) {
                    self.reg[0x0f] = 1;
                }
                
                if addr2!=addr1 {
                    let old_byte2 = self.screen[addr2 as usize];
                    let new_byte2 = self.mem[(self.reg_i+nibble as u16) as usize] << (8-(x%8));
                    self.screen[addr2 as usize] ^= new_byte2;
                    if (old_byte2|new_byte2) != (old_byte2^new_byte2) {
                        self.reg[0x0f] = 1;
                    }
                }      

        }
    }

    fn unknown_opcode(&self) {

        panic!("Unknwown opcode !");
    }


}
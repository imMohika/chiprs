pub mod constants;
mod fontset;

use crate::emu::constants::{RAM_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH, STACK_SIZE, START_ADDR, V_SIZE};
use crate::emu::fontset::{FONTSET, FONTSET_SIZE};

pub struct Emulator {
    counter: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; V_SIZE],
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    stack_ptr: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Emulator {
    pub fn new() -> Self {
        let mut emu = Self {
            counter: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; V_SIZE],
            i_reg: 0,
            stack: [0; STACK_SIZE],
            stack_ptr: 0,
            delay_timer: 0,
            sound_timer: 0,
        };
        emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        emu
    }

    pub fn reset(&mut self) {
        self.counter = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; V_SIZE];
        self.i_reg = 0;
        self.stack = [0; STACK_SIZE];
        self.stack_ptr = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        self.ram[start..start + data.len()].copy_from_slice(data);
    }

    pub fn get_screen(&self) -> &[bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.screen
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1{
                // TODO)) beep beep
            }
            self.sound_timer -= 1;
        }
    }

    fn fetch(&mut self) -> u16 {
        let higher = self.ram[self.counter as usize] as u16;
        let lower = self.ram[self.counter as usize + 1] as u16;
        self.counter += 2;

        let op = (higher << 8) | lower;
        op
    }

    fn execute(&mut self, op: u16) {
       let d1 = ((op & 0xF000) >> 12) as u8;
        let d2 = ((op & 0x0F00) >> 8) as u8;
        let d3 = ((op & 0x00F0) >> 4) as u8;
        let d4 = ((op & 0x000F) >> 0) as u8;

        match (d1, d2,d3,d4) {
            // 0000: NOP
            (0,0,0,0) => return,
            // 00E0: Clear screen
            (0,0,0xE,0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            // 1NNN: jump
            (1,_,_,_) => {
                self.counter = op & 0xFFF
            }
            // 6XNN: set register VX
            (6, _,_,_) => {
                self.v_reg[d2 as usize] = (op & 0xFF) as u8;
            }
            // 7XNN: add value to register VX
            (7, _,_,_) => {
                let nn = (op & 0xFF) as u8;
                self.v_reg[d2 as usize] = self.v_reg[d2 as usize].wrapping_add(nn);
            }
            // ANNN: set index register I
            (0xA, _,_,_) => {
                self.i_reg = op & 0xFFF;
            }
            // DXYN: display/draw
            (0xD,_,_,_) => {
                let x_cord = self.v_reg[d2 as usize] as usize;
                let y_cord = self.v_reg[d3 as usize] as usize;
                let rows = d4;

                let mut any_flipped = false;
                for y_line in 0..rows {
                    let pixels = self.ram[(self.i_reg + y_line as u16) as usize];
                    for x_line in 0..8 {
                        // Only flip if current pixel's bit is 1
                        if pixels & (0b10000000 >> x_line) != 0 {
                            let x = (x_cord + x_line) % SCREEN_WIDTH;
                            let y = (y_cord + y_line as usize) % SCREEN_HEIGHT;
                            let idx = x + SCREEN_WIDTH * y;

                            any_flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                self.v_reg[0xF] = if any_flipped { 1 } else { 0 };
            }

            (_,_,_,_) => unimplemented!("unimplemented operation: {}", op),
        }
    }

    fn push(&mut self, v: u16) {
        self.stack[self.stack_ptr as usize] = v;
        self.stack_ptr += 1;
    }

    fn pop(&mut self) -> u16 {
        self.stack_ptr -= 1;
        self.stack[self.stack_ptr as usize]
    }
}

pub mod constants;
pub mod fontset;
pub mod keys;

use crate::emu::constants::{
    KEYPAD_SIZE, RAM_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH, STACK_SIZE, START_ADDR, V_SIZE,
};
use crate::emu::fontset::{FONTSET, FONTSET_SIZE};
use crate::emu::keys::ChipKey;
use rand::random;
use std::collections::HashMap;

pub struct Emulator {
    counter: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; V_SIZE],
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    stack_ptr: u16,
    keys: [bool; KEYPAD_SIZE],
    waiting_for_key_reg: Option<u8>,
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
            keys: [false; KEYPAD_SIZE],
            waiting_for_key_reg: None,
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
        self.keys = [false; KEYPAD_SIZE];
        self.waiting_for_key_reg = None;
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
        if self.waiting_for_key_reg.is_some() {
            return;
        }
        let op = self.fetch();
        self.execute(op);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO)) beep beep
            }
            self.sound_timer -= 1;
        }
    }

    pub fn key_pressed(&mut self, key: ChipKey) {
        self.keys[key.to_hex() as usize] = true;
    }

    pub fn key_released(&mut self, key: ChipKey) {
        let idx = key.to_hex();
        self.keys[idx as usize] = false;
        if let Some(x) = self.waiting_for_key_reg {
            self.v_reg[x as usize] = idx;
            self.waiting_for_key_reg = None;
        }
    }

    pub fn key_states(&self) -> HashMap<ChipKey, bool> {
        (0..16)
            .map(|i| {
                let key = ChipKey::from_hex(i as u8).unwrap();
                (key, self.keys[i])
            })
            .collect()
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
        let d4 = (op & 0x000F) as u8;

        match (d1, d2, d3, d4) {
            // 0000: NOP
            (0, 0, 0, 0) => (),
            // 00E0: Clear screen
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            // 1NNN: jump
            (1, _, _, _) => self.counter = op & 0xFFF,
            // BNNN: jump to NNN + V0
            (0xB, _, _, _) => self.counter = (op & 0xFFF) + self.v_reg[0] as u16,

            // 00EE: return from a subroutine
            (0, 0, 0xE, 0xE) => self.counter = self.pop(),
            // 2NNN: execute subroutine
            (2, _, _, _) => {
                self.push(self.counter);
                self.counter = op & 0xFFF
            }

            // 3XNN: skip if vx == nn
            (3, _, _, _) => {
                let nn = (op & 0xFF) as u8;
                if self.v_reg[d2 as usize] == nn {
                    self.counter += 2;
                }
            }
            // 4XNN: skip if vx != nn
            (4, _, _, _) => {
                let nn = (op & 0xFF) as u8;
                if self.v_reg[d2 as usize] != nn {
                    self.counter += 2;
                }
            }
            // 5XY0: skip if vx == vy
            (5, _, _, _) => {
                if self.v_reg[d2 as usize] == self.v_reg[d3 as usize] {
                    self.counter += 2;
                }
            }
            // 9XY0: skip if vx == vy
            (9, _, _, _) => {
                if self.v_reg[d2 as usize] != self.v_reg[d3 as usize] {
                    self.counter += 2;
                }
            }

            // 6XNN: set register VX
            (6, _, _, _) => {
                self.v_reg[d2 as usize] = (op & 0xFF) as u8;
            }
            // 7XNN: add value to register VX
            (7, _, _, _) => {
                let nn = (op & 0xFF) as u8;
                self.v_reg[d2 as usize] = self.v_reg[d2 as usize].wrapping_add(nn);
            }

            // 8XY0: store the value of VY in VX
            (8, _, _, 0) => {
                self.v_reg[d2 as usize] = self.v_reg[d3 as usize];
            }
            // 8XY1: Set VX to VX or VY
            (8, _, _, 1) => {
                self.v_reg[d2 as usize] |= self.v_reg[d3 as usize];
            }
            // 8XY2: Set VX to VX AND VY
            (8, _, _, 2) => {
                self.v_reg[d2 as usize] &= self.v_reg[d3 as usize];
            }
            // 8XY3: Set VX to VX XOR VY
            (8, _, _, 3) => {
                self.v_reg[d2 as usize] ^= self.v_reg[d3 as usize];
            }
            // 8XY4: Add VY to VX
            (8, _, _, 4) => {
                let (vx, carry) = self.v_reg[d2 as usize].overflowing_add(self.v_reg[d3 as usize]);
                self.v_reg[d2 as usize] = vx;
                self.v_reg[0xF] = if carry { 1 } else { 0 };
            }
            // 8XY5: Sub VY from VX
            (8, _, _, 5) => {
                let (vx, borrow) = self.v_reg[d2 as usize].overflowing_sub(self.v_reg[d3 as usize]);
                self.v_reg[d2 as usize] = vx;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            }
            // 8XY6: right shift VX
            (8, _, _, 6) => {
                // starting with CHIP-48 and SUPER-CHIP in the early 1990s, these instructions were changed so that they shifted VX in place, and ignored the Y completely.
                self.v_reg[d2 as usize] = self.v_reg[d3 as usize];
                let least_sig = self.v_reg[d2 as usize] & 1;
                self.v_reg[d2 as usize] >>= 1;
                self.v_reg[0xF] = least_sig;
            }
            // 8XYE: left shift VX
            (8, _, _, 0xE) => {
                self.v_reg[d2 as usize] = self.v_reg[d3 as usize];
                let most_sig = (self.v_reg[d2 as usize] >> 7) & 1;
                self.v_reg[d2 as usize] <<= 1;
                self.v_reg[0xF] = most_sig;
            }

            // 8XY7: set VX to VY - VX
            (8, _, _, 7) => {
                let (vx, borrow) = self.v_reg[d3 as usize].overflowing_sub(self.v_reg[d2 as usize]);
                self.v_reg[d2 as usize] = vx;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            }

            // ANNN: set index register I
            (0xA, _, _, _) => {
                self.i_reg = op & 0xFFF;
            }
            // CXNN: VX = rand & NN
            (0xC, _, _, _) => {
                let rnd: u8 = random();
                self.v_reg[d2 as usize] = rnd & (op & 0xFF) as u8;
            }
            // DXYN: display/draw
            (0xD, _, _, _) => {
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
            // EX9E: skip if VX key is pressed
            (0xE, _, 9, 0xE) => {
                let vx = self.v_reg[d2 as usize];
                let key = self.keys[vx as usize];
                if key {
                    self.counter += 2;
                }
            }
            // EXA1: skip if VX key is not pressed
            (0xE, _, 0xA, 1) => {
                let vx = self.v_reg[d2 as usize];
                let key = self.keys[vx as usize];
                if !key {
                    self.counter += 2;
                }
            }
            // FX0A: get key
            (0xF, _, 0, 0xA) => {
                self.waiting_for_key_reg = Some(d2);
            }

            // FX07: store delay timer in VX
            (0xF, _, 0, 7) => {
                self.v_reg[d2 as usize] = self.delay_timer;
            }
            // FX15: delay timer = VX
            (0xF, _, 1, 5) => {
                self.delay_timer = self.v_reg[d2 as usize];
            }
            // FX18: sound timer = VX
            (0xF, _, 1, 8) => {
                self.sound_timer = self.v_reg[d2 as usize];
            }
            // FX1E: add VX to I
            (0xF, _, 1, 0xE) => {
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[d2 as usize] as u16)
            }
            // FX29: font character
            (0xF, _, 2, 9) => self.i_reg = (self.v_reg[d2 as usize] * 5) as u16,
            // FX33: binary-coded decimal conversion
            (0xF, _, 3, 3) => {
                let vx = self.v_reg[d2 as usize];
                self.ram[self.i_reg as usize] = vx / 100;
                self.ram[self.i_reg as usize + 1] = (vx / 10) % 10;
                self.ram[self.i_reg as usize + 2] = vx % 10;
            }
            // FX55: save V0..VX into I
            (0xF, _, 5, 5) => {
                for idx in 0..=d2 as usize {
                    self.ram[self.i_reg as usize + idx] = self.v_reg[idx]
                }
            }
            // FX65: load I into V0..VX
            (0xF, _, 6, 5) => {
                for idx in 0..=d2 as usize {
                    self.v_reg[idx] = self.ram[self.i_reg as usize + idx]
                }
            }

            (_, _, _, _) => unimplemented!("unimplemented operation: {:#04x}", op),
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

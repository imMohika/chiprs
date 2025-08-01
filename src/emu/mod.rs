pub mod constants;
pub mod fontset;
pub mod instruction;
pub mod keys;

use crate::emu::constants::{
    KEYPAD_SIZE, RAM_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH, STACK_SIZE, START_ADDR, V_SIZE,
};
use crate::emu::fontset::{FONTSET, FONTSET_SIZE};
use crate::emu::instruction::Instruction;
use crate::emu::keys::ChipKey;
use rand::random;
use std::collections::HashMap;

pub struct Emulator {
    pub counter: u16,
    ram: [u8; RAM_SIZE],
    rom: Vec<u8>,
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; V_SIZE],
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    stack_ptr: u16,
    keys: [bool; KEYPAD_SIZE],
    waiting_for_key_reg: Option<u8>,
    delay_timer: u8,
    sound_timer: u8,

    is_paused: bool,
}

impl Emulator {
    pub fn new() -> Self {
        let mut emu = Self {
            counter: START_ADDR,
            ram: [0; RAM_SIZE],
            rom: Vec::new(),
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; V_SIZE],
            i_reg: 0,
            stack: [0; STACK_SIZE],
            stack_ptr: 0,
            keys: [false; KEYPAD_SIZE],
            waiting_for_key_reg: None,
            delay_timer: 0,
            sound_timer: 0,
            is_paused: false,
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
        self.load_rom()
    }

    pub fn load(&mut self, rom: &[u8]) {
        self.rom = rom.to_vec();
        self.load_rom()
    }

    fn load_rom(&mut self) {
        let start = START_ADDR as usize;
        self.ram[start..start + self.rom.len()].copy_from_slice(&self.rom);
    }

    pub fn get_screen(&self) -> &[bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.screen
    }

    pub fn pause_or_resume(&mut self) {
        self.is_paused = !self.is_paused;
    }
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn tick(&mut self) {
        if self.is_paused {
            return;
        }
        self.next()
    }

    pub fn next(&mut self) {
        if self.waiting_for_key_reg.is_some() {
            return;
        }
        let instruction = self.fetch(self.counter as usize);
        self.counter += 2;

        self.execute(instruction);
    }

    pub fn previous(&mut self) {
        self.counter -= 4;
        self.next()
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

    pub fn fetch(&self, at: usize) -> Instruction {
        let higher = self.ram[at] as u16;
        let lower = self.ram[at + 1] as u16;

        let op = (higher << 8) | lower;
        Instruction::from_opcode(op)
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Nop => (),
            Instruction::ClearScreen => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            Instruction::Ret => self.counter = self.pop(),
            Instruction::Jump { nnn } => self.counter = nnn,
            Instruction::JumpPlusV0 { nnn } => self.counter = nnn + self.v_reg[0] as u16,
            Instruction::Call { nnn } => {
                self.push(self.counter);
                self.counter = nnn
            }
            Instruction::SkipVxEqNN { x, nn } => {
                if self.v_reg[x] == nn {
                    self.counter += 2;
                }
            }
            Instruction::SkipVxNeqNN { x, nn } => {
                if self.v_reg[x] != nn {
                    self.counter += 2;
                }
            }
            Instruction::SkipVxEqVy { x, y } => {
                if self.v_reg[x] == self.v_reg[y] {
                    self.counter += 2;
                }
            }
            Instruction::SkipVxNeqVy { x, y } => {
                if self.v_reg[x] != self.v_reg[y] {
                    self.counter += 2;
                }
            }
            Instruction::SetVxNN { x, nn } => {
                self.v_reg[x] = nn;
            }
            Instruction::SetVxVy { x, y } => {
                self.v_reg[x] = self.v_reg[y];
            }
            Instruction::SetVxDt { x } => {
                self.v_reg[x] = self.delay_timer;
            }
            Instruction::SetVxKey { x } => {
                self.waiting_for_key_reg = Some(x as u8);
            }
            Instruction::SetVxRnd { x, nn } => {
                let rnd: u8 = random();
                self.v_reg[x] = rnd & nn;
            }
            Instruction::SetI { nnn } => {
                self.i_reg = nnn;
            }
            Instruction::SetVxFontToI { x } => self.i_reg = (self.v_reg[x] * 5) as u16,
            Instruction::SetVxBcdToI { x } => {
                let vx = self.v_reg[x];
                self.ram[self.i_reg as usize] = vx / 100;
                self.ram[self.i_reg as usize + 1] = (vx / 10) % 10;
                self.ram[self.i_reg as usize + 2] = vx % 10;
            }
            Instruction::SetDtVx { x } => {
                self.delay_timer = self.v_reg[x];
            }
            Instruction::SetStVx { x } => {
                self.sound_timer = self.v_reg[x];
            }
            Instruction::AddVxNN { x, nn } => {
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            Instruction::AddVxVy { x, y } => {
                let (vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                self.v_reg[x] = vx;
                self.v_reg[0xF] = if carry { 1 } else { 0 };
            }
            Instruction::SubVxVy { x, y } => {
                let (vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                self.v_reg[x] = vx;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            }
            Instruction::SubVyVx { x, y } => {
                let (vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                self.v_reg[x] = vx;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            }
            Instruction::AddVxToI { x } => {
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[x] as u16)
            }
            Instruction::OrVxVy { x, y } => {
                self.v_reg[x] |= self.v_reg[y];
            }
            Instruction::AndVxVy { x, y } => {
                self.v_reg[x] &= self.v_reg[y];
            }
            Instruction::XorVxVy { x, y } => {
                self.v_reg[x] ^= self.v_reg[y];
            }
            Instruction::RShiftVx { x, y } => {
                // starting with CHIP-48 and SUPER-CHIP in the early 1990s, these instructions were changed so that they shifted VX in place, and ignored the Y completely.
                self.v_reg[x] = self.v_reg[y];
                let least_sig = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = least_sig;
            }
            Instruction::LShiftVx { x, y } => {
                self.v_reg[x] = self.v_reg[y];
                let most_sig = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = most_sig;
            }
            Instruction::SkipVxDown { x } => {
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if key {
                    self.counter += 2;
                }
            }
            Instruction::SkipVxUp { x } => {
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if !key {
                    self.counter += 2;
                }
            }
            Instruction::Draw { x, y, n } => {
                let x_cord = self.v_reg[x] as usize;
                let y_cord = self.v_reg[y] as usize;
                let rows = n;

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
            Instruction::SaveVx { x } => {
                for idx in 0..=x {
                    self.ram[self.i_reg as usize + idx] = self.v_reg[idx]
                }
            }
            Instruction::LoadVx { x } => {
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[self.i_reg as usize + idx]
                }
            }
            Instruction::Unknown { opcode } => {
                unimplemented!("unimplemented operation: {:#04x}", opcode)
            }
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

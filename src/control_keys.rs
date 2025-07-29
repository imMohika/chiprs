use crate::emu::Emulator;
use crate::text::{CHAR_SIZE, TextDrawer};
use minifb::{Key, Window};
use std::iter::Iterator;

const GAP: usize = 4;
const SCALE: usize = 1;
const JMP: usize = (CHAR_SIZE + GAP) * SCALE;
const LINES: [&str; 2] = ["F1: reset F2: pause", "F3: prev F4: next"];
const CHARS_PER_LINE: usize = 20;

pub const CONTROL_KEYS_WIDTH: usize = CHARS_PER_LINE * CHAR_SIZE;
pub const CONTROL_KEYS_HEIGHT: usize = LINES.len() * JMP;

pub fn draw_control_keys(text_drawer: &TextDrawer, buffer: &mut [u32], (x, y): (usize, usize)) {
    for (idx, line) in LINES.iter().enumerate() {
        text_drawer.draw(buffer, (x, y + idx * JMP), SCALE, line);
    }
}

pub fn handle_control_keys(window: &Window, emu: &mut Emulator) {
    if (window.is_key_down(Key::F1)) {
        emu.reset()
    }
}

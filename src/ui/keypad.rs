use crate::emu::constants::KEYPAD_SIZE;
use crate::emu::keys::ChipKey;
use crate::emu::Emulator;
use crate::ui::draw::{ShapeDrawer, BORDER_WIDTH};
use crate::ui::text::CHAR_SIZE;

const GAP: usize = 4;
const SCALE: usize = 2;
const PAD_SIZE: usize = CHAR_SIZE + 2;
const JMP: usize = (PAD_SIZE + GAP) * SCALE;

pub const KEYPAD_WIDTH: usize = (KEYPAD_SIZE / 4) * JMP + 2 * (GAP + BORDER_WIDTH);
pub const KEYPAD_HEIGHT: usize = (KEYPAD_SIZE / 4) * JMP + 2 * (GAP + BORDER_WIDTH);

const KEYPAD: [char; 16] = [
    '1', '2', '3', 'C', '4', '5', '6', 'D', '7', '8', '9', 'E', 'A', '0', 'B', 'F',
];
pub fn draw_keypad(
    buffer: &mut [u32],
    emu: &Emulator,
    shape_drawer: &ShapeDrawer,
    (x, y): (usize, usize),
) {
    let start_x = x + 8;
    let start_y = y + 8;

    shape_drawer.border(buffer, (x, y), (x + KEYPAD_WIDTH, y + KEYPAD_HEIGHT + 2));
    let key_states = emu.key_states();
    let pressed_gap = CHAR_SIZE * SCALE + BORDER_WIDTH + 2 * SCALE;

    for (idx, ch) in KEYPAD.iter().enumerate() {
        let pos_x = start_x + ((idx % 4) * JMP);
        let pos_y = start_y + ((idx / 4) * JMP);
        if let Some(key) = ChipKey::from_char(ch) {
            if key_states.get(&key) == Some(&true) {
                shape_drawer.border(
                    buffer,
                    (pos_x, pos_y),
                    (pos_x + pressed_gap, pos_y + pressed_gap),
                );
            }
        }

        shape_drawer.text().draw(
            buffer,
            (
                pos_x + BORDER_WIDTH + 1 * SCALE,
                pos_y + BORDER_WIDTH + 1 * SCALE,
            ),
            SCALE,
            &ch.to_string(),
        );
    }
}

use crate::emu::constants::RAM_SIZE;
use crate::emu::Emulator;
use crate::ui::draw::{ShapeDrawer, BORDER_WIDTH, GAP};
use crate::ui::text::CHAR_SIZE;
use std::cmp::max;

const MAX_CHARS_WIDTH: usize = CHAR_SIZE * 50;
pub const INSTRUCTION_LIST_MAX_WIDTH: usize = MAX_CHARS_WIDTH + 2 * (GAP + BORDER_WIDTH);
pub fn draw_instruction_list(
    buffer: &mut [u32],
    emu: &Emulator,
    shape_drawer: &ShapeDrawer,
    (x, y): (usize, usize),
) -> usize {
    let counter = emu.counter;
    let from = max(counter - 10, 0);
    let to = if counter + 10 > RAM_SIZE as u16 {
        (RAM_SIZE - 1) as u16
    } else {
        counter + 10
    };

    let curr_x = x + GAP;
    let mut curr_y = y + GAP;
    for i in from..to {
        shape_drawer.text().draw(
            buffer,
            (curr_x, curr_y),
            1,
            &format!(
                "{} {} > {}",
                if i == counter { ">" } else { " " },
                &i,
                emu.fetch(i as usize),
            ),
        );
        curr_y += CHAR_SIZE + GAP;
    }

    shape_drawer.border(buffer, (x, y), (x + MAX_CHARS_WIDTH + GAP, curr_y + GAP));

    curr_y + GAP + BORDER_WIDTH
}

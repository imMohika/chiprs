use crate::ui::draw::{ShapeDrawer, BORDER_WIDTH};
use crate::ui::text::CHAR_SIZE;

const GAP: usize = 4;
const SCALE: usize = 1;
const JMP: usize = (CHAR_SIZE + GAP) * SCALE;
const LINES: [&str; 4] = ["F1: reset", "F2: pause/resume", "F3: step", "ESC: exit"];
const MAX_CHARS_WIDTH: usize = 16 * CHAR_SIZE;

pub const CONTROL_KEYS_WIDTH: usize = MAX_CHARS_WIDTH + 2 * (GAP + BORDER_WIDTH);

pub fn draw_control_keys(
    buffer: &mut [u32],
    shape_drawer: &ShapeDrawer,
    (x, y): (usize, usize),
) -> usize {
    let curr_x = x + GAP;
    let mut curr_y = y + GAP;
    for line in LINES.iter() {
        shape_drawer
            .text()
            .draw(buffer, (curr_x, curr_y), SCALE, line);
        curr_y += JMP;
    }

    shape_drawer.border(buffer, (x, y), (x + MAX_CHARS_WIDTH + GAP, curr_y + GAP));

    curr_y + GAP + BORDER_WIDTH
}

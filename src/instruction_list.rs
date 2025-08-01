use crate::draw::ShapeDrawer;
use crate::emu::constants::RAM_SIZE;
use crate::emu::Emulator;
use crate::text::CHAR_SIZE;
use crate::GAP;
use std::cmp::max;

pub fn draw_instruction_list(
    buffer: &mut [u32],
    emu: &Emulator,
    shape_drawer: &ShapeDrawer,
    (x, y): (usize, usize),
) {
    let counter = emu.counter;
    let from = max(counter - 10, 0);
    let to = if counter + 10 > RAM_SIZE as u16 {
        (RAM_SIZE - 1) as u16
    } else {
        counter + 10
    };

    let mut curr_y = y;
    for i in from..to {
        shape_drawer.text().draw(
            buffer,
            (x, curr_y),
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
}

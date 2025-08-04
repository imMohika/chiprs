mod control_keys;
mod draw;
mod instruction_list;
mod keypad;
mod text;

use crate::emu::constants::{EMU_SCREEN_HEIGHT, EMU_SCREEN_WIDTH};
use crate::emu::Emulator;
use crate::ui::control_keys::draw_control_keys;
use crate::ui::draw::{ShapeDrawer, BORDER_WIDTH, GAP, LINE_SIZE};
use crate::ui::instruction_list::{draw_instruction_list, INSTRUCTION_LIST_MAX_WIDTH};
use crate::ui::keypad::{draw_keypad, KEYPAD_HEIGHT, KEYPAD_WIDTH};
use std::cmp::max;

pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct UiDrawer {
    pub emu_size: Size,
    pub emu_scale: usize,
    pub window_size: Size,
    shape_drawer: ShapeDrawer,
}

impl UiDrawer {
    pub fn new(emu_scale: usize) -> Self {
        let emu_width = EMU_SCREEN_WIDTH * emu_scale;
        let emu_height = EMU_SCREEN_HEIGHT * emu_scale;

        let window_width = emu_width + GAP + LINE_SIZE + GAP + INSTRUCTION_LIST_MAX_WIDTH;
        let window_height = emu_height + GAP + LINE_SIZE + GAP + KEYPAD_HEIGHT;

        Self {
            emu_size: Size {
                width: emu_width,
                height: emu_height,
            },
            emu_scale,
            window_size: Size {
                width: window_width,
                height: window_height,
            },
            shape_drawer: ShapeDrawer::new(window_width),
        }
    }

    pub fn draw(&self, emu: &mut Emulator) -> Vec<u32> {
        let mut window_buffer: Vec<u32> = vec![0; self.window_size.width * self.window_size.height];

        for (i, pixel) in emu.get_screen().iter().enumerate() {
            if *pixel {
                let x = i % EMU_SCREEN_WIDTH;
                let y = i / EMU_SCREEN_WIDTH;
                self.shape_drawer
                    .rect(window_buffer.as_mut_slice(), x, y, self.emu_scale);
            }
        }

        self.shape_drawer.border(
            window_buffer.as_mut_slice(),
            (0, 0),
            (self.emu_size.width + GAP, self.emu_size.height + GAP),
        );

        let mut curr_x = self.emu_size.width + GAP + BORDER_WIDTH + GAP;
        let mut curr_y = self.emu_size.height + GAP + LINE_SIZE + GAP;

        let end_y = draw_instruction_list(
            window_buffer.as_mut_slice(),
            &emu,
            &self.shape_drawer,
            (curr_x, 0),
        );

        curr_x = 0;
        curr_y = max(self.emu_size.height, end_y) + BORDER_WIDTH + GAP;
        draw_keypad(
            window_buffer.as_mut_slice(),
            &emu,
            &self.shape_drawer,
            (curr_x, curr_y),
        );

        curr_x += KEYPAD_WIDTH + GAP;

        let end_y = draw_control_keys(
            window_buffer.as_mut_slice(),
            &self.shape_drawer,
            (curr_x, curr_y),
        );
        curr_y = end_y + GAP;

        if emu.is_paused() {
            self.shape_drawer.text().draw(
                window_buffer.as_mut_slice(),
                (curr_x, curr_y),
                2,
                "PAUSED",
            )
        }

        window_buffer
    }
}

use crate::text::TextDrawer;

pub const FOREGROUND: u32 = 0x00FFFFFF; // white

pub const LINE_SIZE: usize = 4;
pub const BORDER_WIDTH: usize = 2;
pub struct ShapeDrawer {
    width: usize,
    text_drawer: TextDrawer,
}

impl ShapeDrawer {
    pub fn new(width: usize) -> ShapeDrawer {
        ShapeDrawer {
            width,
            text_drawer: TextDrawer::new(width),
        }
    }

    pub fn text(self: &Self) -> &TextDrawer {
        &self.text_drawer
    }

    pub fn rect(self: &Self, window_buffer: &mut [u32], x: usize, y: usize, scale: usize) {
        let start_x = x * scale;
        let start_y = y * scale;

        for y_offset in 0..scale {
            for x_offset in 0..scale {
                let window_x = start_x + x_offset;
                let window_y = start_y + y_offset;
                let idx = window_y * self.width + window_x;

                window_buffer[idx] = FOREGROUND;
            }
        }
    }

    pub fn vertical_line(
        self: &Self,
        window_buffer: &mut [u32],
        x: usize,
        from_y: usize,
        to_y: usize,
    ) {
        let start_x = x;

        for y in from_y..to_y {
            for x_offset in 0..LINE_SIZE {
                let x = start_x + x_offset;
                let idx = y * self.width + x;
                window_buffer[idx] = FOREGROUND;
            }
        }
    }

    pub fn horizontal_line(
        &self,
        window_buffer: &mut [u32],
        (from_x,to_x): (usize, usize),
        y: usize,
    ) {
        let start_y = y;

        for x in from_x..to_x {
            for y_offset in 0..LINE_SIZE {
                let y = start_y + y_offset;
                let idx = y * self.width + x;
                window_buffer[idx] = FOREGROUND;
            }
        }
    }

    pub fn border(
        self: &Self,
        window_buffer: &mut [u32],
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) {
        for y in from_y..to_y {
            for x in from_x..to_x {
                let is_top = y < from_y + BORDER_WIDTH;
                let is_bottom = y >= to_y - BORDER_WIDTH;
                let is_left = x < from_x + BORDER_WIDTH;
                let is_right = x >= to_x - BORDER_WIDTH;

                if is_top || is_bottom || is_left || is_right {
                    let idx = y * self.width + x;
                    if idx < window_buffer.len() {
                        window_buffer[idx] = FOREGROUND;
                    }
                }
            }
        }
    }
}

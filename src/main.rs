use std::env;
use std::fs::File;
use std::io::Read;
use minifb::{Key, Window, WindowOptions};
use crate::emu::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::emu::Emulator;

mod emu;

const SCALE : usize = 10;
const WINDOW_WIDTH: usize = SCREEN_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = SCREEN_HEIGHT * SCALE;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} <rom-file>", args[0]);
        return;
    }

    let mut emu = Emulator::new();

    let mut rom = File::open(&args[1]).expect("unable to open rom");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("unable to read rom");
    emu.load(&buffer);
    
    let mut window = Window::new(
        "Chiprs",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);
    window.set_background_color(0,0,0);
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        emu.tick();
        emu.tick_timers();

        let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
        for (i, pixel) in emu.get_screen().iter().enumerate() {
            if *pixel {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;
                draw_rect(window_buffer.as_mut_slice(), x, y);
            }
        }

        window
            .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn draw_rect(window_buffer: &mut [u32], x: usize, y: usize) {
    let start_x = x * SCALE;
    let start_y = y * SCALE;

    for y_offset in 0..SCALE {
        for x_offset in 0..SCALE {
            let window_x = start_x + x_offset;
            let window_y = start_y + y_offset;
            let idx = window_y * WINDOW_WIDTH + window_x;
            
            // Set the pixel color to white
            window_buffer[idx] = 0x00FFFFFF;
        }
    }
}

use crate::emu::Emulator;
use crate::emu::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::emu::keys::ChipKey;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::Read;

mod emu;

const SCALE: usize = 10;
const WINDOW_WIDTH: usize = SCREEN_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = SCREEN_HEIGHT * SCALE;
const TICKS_PER_FRAME: usize = 10;

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
    window.set_background_color(0, 0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| {
                if let Some(key) = convert_key(key) {
                    emu.key_pressed(key)
                }
            });

        for _ in 0..TICKS_PER_FRAME {
            emu.tick();
        }
        emu.tick_timers();

        window.get_keys_released().iter().for_each(|key| {
            if let Some(key) = convert_key(key) {
                emu.key_released(key)
            }
        });

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

fn convert_key(key: &Key) -> Option<ChipKey> {
    match key {
        Key::Key1 => Some(ChipKey::Num1),
        Key::Key2 => Some(ChipKey::Num2),
        Key::Key3 => Some(ChipKey::Num3),
        Key::Key4 => Some(ChipKey::C),
        Key::Q => Some(ChipKey::Num4),
        Key::W => Some(ChipKey::Num5),
        Key::E => Some(ChipKey::Num6),
        Key::R => Some(ChipKey::D),
        Key::A => Some(ChipKey::Num7),
        Key::S => Some(ChipKey::Num8),
        Key::D => Some(ChipKey::Num9),
        Key::F => Some(ChipKey::E),
        Key::Z => Some(ChipKey::A),
        Key::X => Some(ChipKey::Num0),
        Key::C => Some(ChipKey::B),
        Key::V => Some(ChipKey::F),
        _ => None,
    }
}

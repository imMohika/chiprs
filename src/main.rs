use crate::control_keys::{draw_control_keys, handle_control_keys, CONTROL_KEYS_HEIGHT};
use crate::draw::{ShapeDrawer, LINE_SIZE};
use crate::emu::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::emu::keys::ChipKey;
use crate::emu::Emulator;
use crate::instruction_list::draw_instruction_list;
use crate::keypad::{draw_keypad, KEYPAD_HEIGHT, KEYPAD_WIDTH};
use crate::text::CHAR_SIZE;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::Read;

mod control_keys;
mod draw;
mod emu;
mod instruction_list;
mod keypad;
mod text;

const TICKS_PER_FRAME: usize = 11;

const EMU_SCALE: usize = 10;
const EMU_WIDTH: usize = SCREEN_WIDTH * EMU_SCALE;
const EMU_HEIGHT: usize = SCREEN_HEIGHT * EMU_SCALE;

const GAP: usize = 8;
const LINE_SCALE: usize = 4;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} <rom-file>", args[0]);
        return;
    }

    let mut rom = File::open(&args[1]).expect("unable to open rom");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("unable to read rom");

    let mut emu = Emulator::new();
    emu.load(&buffer);

    let window_width = EMU_WIDTH + GAP + LINE_SIZE + GAP + (CHAR_SIZE * 40) + GAP;
    let window_height = EMU_HEIGHT + GAP + LINE_SIZE + GAP + KEYPAD_HEIGHT + GAP;

    let mut window = Window::new("Chiprs", window_width, window_height, WindowOptions {
        resize: false,
        ..WindowOptions::default()
    })
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    window.set_background_color(0, 0, 0);

    let shape_drawer = ShapeDrawer::new(window_width);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_control_keys(&window, &mut emu);

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

        let mut window_buffer: Vec<u32> = vec![0; window_width * window_height];
        for (i, pixel) in emu.get_screen().iter().enumerate() {
            if *pixel {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;
                shape_drawer.rect(window_buffer.as_mut_slice(), x, y, EMU_SCALE);
            }
        }

        shape_drawer.vertical_line(
            window_buffer.as_mut_slice(),
            EMU_WIDTH + GAP,
            0,
            EMU_HEIGHT + GAP,
        );
        shape_drawer.horizontal_line(
            window_buffer.as_mut_slice(),
            (0, EMU_WIDTH + GAP + LINE_SIZE),
            EMU_HEIGHT + GAP,
        );

        draw_instruction_list(
            window_buffer.as_mut_slice(),
            &emu,
            &shape_drawer,
            (EMU_WIDTH + GAP + LINE_SIZE + GAP, GAP),
        );

        draw_keypad(
            &shape_drawer,
            window_buffer.as_mut_slice(),
            &emu,
            (GAP, EMU_HEIGHT + GAP + LINE_SIZE + GAP),
        );

        draw_control_keys(
            shape_drawer.text(),
            window_buffer.as_mut_slice(),
            (GAP + KEYPAD_WIDTH + GAP, EMU_HEIGHT + GAP + LINE_SIZE + GAP),
        );

        if emu.is_paused() {
            shape_drawer.text().draw(
                window_buffer.as_mut_slice(),
                (
                    GAP + KEYPAD_WIDTH + GAP,
                    EMU_HEIGHT + GAP + LINE_SIZE + GAP + CONTROL_KEYS_HEIGHT,
                ),
                1,
                "PAUSED",
            )
        }

        window
            .update_with_buffer(&window_buffer, window_width, window_height)
            .unwrap();
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

use crate::emu::Emulator;
use crate::keys::{convert_key, handle_control_keys};
use crate::ui::UiDrawer;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::Read;

mod emu;
mod keys;
mod ui;

const TICKS_PER_FRAME: usize = 11;
const EMU_SCALE: usize = 10;

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
    let ui = UiDrawer::new(EMU_SCALE);

    let mut window = Window::new(
        "Chiprs",
        ui.window_size.width,
        ui.window_size.height,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    window.set_background_color(0, 0, 0);

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

        let window_buffer = ui.draw(&mut emu);
        window
            .update_with_buffer(
                window_buffer.as_slice(),
                ui.window_size.width,
                ui.window_size.height,
            )
            .unwrap();
    }
}

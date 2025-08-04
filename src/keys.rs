use crate::emu::keys::ChipKey;
use crate::emu::Emulator;
use minifb::{Key, KeyRepeat, Window};

pub fn convert_key(key: &Key) -> Option<ChipKey> {
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

pub fn handle_control_keys(window: &Window, emu: &mut Emulator) {
    if window.is_key_pressed(Key::F1, KeyRepeat::No) {
        emu.reset()
    }

    if window.is_key_pressed(Key::F2, KeyRepeat::No) {
        emu.pause_or_resume()
    }

    if window.is_key_pressed(Key::F3, KeyRepeat::Yes) {
        emu.next()
    }
}

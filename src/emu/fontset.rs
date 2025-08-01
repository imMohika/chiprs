pub const FONTSET_SIZE: usize = 16 * 5;

pub const FONTSET: [u8; FONTSET_SIZE] = [
    0b11110000, 0b10010000, 0b10010000, 0b10010000, 0b11110000, // 0
    0b01100000, 0b00100000, 0b00100000, 0b00100000, 0b01110000, // 1
    0b11110000, 0b00010000, 0b11110000, 0b10000000, 0b11110000, // 2
    0b11110000, 0b00010000, 0b11110000, 0b00010000, 0b11110000, // 3
    0b10100000, 0b10100000, 0b11110000, 0b00100000, 0b00100000, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
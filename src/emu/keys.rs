use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChipKey {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl ChipKey {
    pub fn to_hex(&self) -> u8 {
        match self {
            ChipKey::Num0 => 0x0,
            ChipKey::Num1 => 0x1,
            ChipKey::Num2 => 0x2,
            ChipKey::Num3 => 0x3,
            ChipKey::Num4 => 0x4,
            ChipKey::Num5 => 0x5,
            ChipKey::Num6 => 0x6,
            ChipKey::Num7 => 0x7,
            ChipKey::Num8 => 0x8,
            ChipKey::Num9 => 0x9,
            ChipKey::A => 0xA,
            ChipKey::B => 0xB,
            ChipKey::C => 0xC,
            ChipKey::D => 0xD,
            ChipKey::E => 0xE,
            ChipKey::F => 0xF,
        }
    }

    pub fn from_hex(hex: u8) -> Option<ChipKey> {
        match hex {
            0x0 => Some(ChipKey::Num0),
            0x1 => Some(ChipKey::Num1),
            0x2 => Some(ChipKey::Num2),
            0x3 => Some(ChipKey::Num3),
            0x4 => Some(ChipKey::Num4),
            0x5 => Some(ChipKey::Num5),
            0x6 => Some(ChipKey::Num6),
            0x7 => Some(ChipKey::Num7),
            0x8 => Some(ChipKey::Num8),
            0x9 => Some(ChipKey::Num9),
            0xA => Some(ChipKey::A),
            0xB => Some(ChipKey::B),
            0xC => Some(ChipKey::C),
            0xD => Some(ChipKey::D),
            0xE => Some(ChipKey::E),
            0xF => Some(ChipKey::F),
            _ => None,
        }
    }

    pub fn from_char(c: &char) -> Option<ChipKey> {
        match c {
            '0' => Some(ChipKey::Num0),
            '1' => Some(ChipKey::Num1),
            '2' => Some(ChipKey::Num2),
            '3' => Some(ChipKey::Num3),
            '4' => Some(ChipKey::Num4),
            '5' => Some(ChipKey::Num5),
            '6' => Some(ChipKey::Num6),
            '7' => Some(ChipKey::Num7),
            '8' => Some(ChipKey::Num8),
            '9' => Some(ChipKey::Num9),
            'a' | 'A' => Some(ChipKey::A),
            'b' | 'B' => Some(ChipKey::B),
            'c' | 'C' => Some(ChipKey::C),
            'd' | 'D' => Some(ChipKey::D),
            'e' | 'E' => Some(ChipKey::E),
            'f' | 'F' => Some(ChipKey::F),
            _ => None,
        }
    }
}

impl fmt::Display for ChipKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key_str = match self {
            ChipKey::Num0 => "0",
            ChipKey::Num1 => "1",
            ChipKey::Num2 => "2",
            ChipKey::Num3 => "3",
            ChipKey::Num4 => "4",
            ChipKey::Num5 => "5",
            ChipKey::Num6 => "6",
            ChipKey::Num7 => "7",
            ChipKey::Num8 => "8",
            ChipKey::Num9 => "9",
            ChipKey::A => "A",
            ChipKey::B => "B",
            ChipKey::C => "C",
            ChipKey::D => "D",
            ChipKey::E => "E",
            ChipKey::F => "F",
        };
        write!(f, "{}", key_str)
    }
}

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
    F
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
}
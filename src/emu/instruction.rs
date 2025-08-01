use std::fmt;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    ClearScreen,
    Ret,

    Jump { nnn: u16 },
    JumpPlusV0 { nnn: u16 },
    Call { nnn: u16 },

    SkipVxEqNN { x: usize, nn: u8 },
    SkipVxNeqNN { x: usize, nn: u8 },
    SkipVxEqVy { x: usize, y: usize },
    SkipVxNeqVy { x: usize, y: usize },

    SetVxNN { x: usize, nn: u8 },
    SetVxVy { x: usize, y: usize },
    SetVxDt { x: usize },
    SetVxKey { x: usize },
    SetVxRnd { x: usize, nn: u8 },
    SetI { nnn: u16 },
    SetVxFontToI { x: usize },
    SetVxBcdToI { x: usize },
    SetDtVx { x: usize },
    SetStVx { x: usize },

    AddVxNN { x: usize, nn: u8 },
    AddVxVy { x: usize, y: usize },
    SubVxVy { x: usize, y: usize },
    SubVyVx { x: usize, y: usize },
    AddVxToI { x: usize },

    OrVxVy { x: usize, y: usize },
    AndVxVy { x: usize, y: usize },
    XorVxVy { x: usize, y: usize },

    RShiftVx { x: usize, y: usize },
    LShiftVx { x: usize, y: usize },

    SkipVxDown { x: usize },
    SkipVxUp { x: usize },

    Draw { x: usize, y: usize, n: u8 },
    SaveVx { x: usize },
    LoadVx { x: usize },

    Unknown { opcode: u16 },
}

impl Instruction {
    pub fn from_opcode(opcode: u16) -> Self {
        let d1 = (opcode >> 12) as u8;
        let d2 = ((opcode & 0x0F00) >> 8) as u8;
        let d3 = ((opcode & 0x00F0) >> 4) as u8;
        let d4 = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let x = d2 as usize;
        let y = d3 as usize;
        let n = d4;

        match (d1, d2, d3, d4) {
            // 0000: NOP
            (0, 0, 0, 0) => Self::Nop,
            // 00E0: Clear screen
            (0, 0, 0xE, 0) => Self::ClearScreen,
            // 00EE: return from a subroutine
            (0, 0, 0xE, 0xE) => Self::Ret,
            // 1NNN: jump
            (1, _, _, _) => Self::Jump { nnn },
            // 2NNN: execute subroutine
            (2, _, _, _) => Self::Call { nnn },
            // 3XNN: skip if vx == nn
            (3, _, _, _) => Self::SkipVxEqNN { x, nn },
            // 4XNN: skip if vx != nn
            (4, _, _, _) => Self::SkipVxNeqNN { x, nn },
            // 5XY0: skip if vx == vy
            (5, _, _, 0) => Self::SkipVxEqVy { x, y },
            // 6XNN: set register VX
            (6, _, _, _) => Self::SetVxNN { x, nn },
            // 7XNN: add value to register VX
            (7, _, _, _) => Self::AddVxNN { x, nn },
            // 8XY0: store the value of VY in VX
            (8, _, _, 0) => Self::SetVxVy { x, y },
            // 8XY1: Set VX to VX or VY
            (8, _, _, 1) => Self::OrVxVy { x, y },
            // 8XY2: Set VX to VX AND VY
            (8, _, _, 2) => Self::AndVxVy { x, y },
            // 8XY3: Set VX to VX XOR VY
            (8, _, _, 3) => Self::XorVxVy { x, y },
            // 8XY4: Add VY to VX
            (8, _, _, 4) => Self::AddVxVy { x, y },
            // 8XY5: Sub VY from VX
            (8, _, _, 5) => Self::SubVxVy { x, y },
            // 8XY6: right shift VX
            (8, _, _, 6) => Self::RShiftVx { x, y },
            // 8XY7: set VX to VY - VX
            (8, _, _, 7) => Self::SubVyVx { x, y },
            // 8XYE: left shift VX
            (8, _, _, 0xE) => Self::LShiftVx { x, y },
            // 9XY0: skip if vx == vy
            (9, _, _, 0) => Self::SkipVxNeqVy { x, y },
            // ANNN: set index register I
            (0xA, _, _, _) => Self::SetI { nnn },
            // BNNN: jump to NNN + V0
            (0xB, _, _, _) => Self::JumpPlusV0 { nnn },
            // CXNN: VX = rand & NN
            (0xC, _, _, _) => Self::SetVxRnd { x, nn },
            // DXYN: display/draw
            (0xD, _, _, _) => Self::Draw { x, y, n },
            // EX9E: skip if VX key is pressed
            (0xE, _, 9, 0xE) => Self::SkipVxDown { x },
            // EXA1: skip if VX key is not pressed
            (0xE, _, 0xA, 1) => Self::SkipVxUp { x },
            // FX07: store delay timer in VX
            (0xF, _, 0, 7) => Self::SetVxDt { x },
            // FX0A: get key
            (0xF, _, 0, 0xA) => Self::SetVxKey { x },
            // FX15: delay timer = VX
            (0xF, _, 1, 5) => Self::SetDtVx { x },
            // FX18: sound timer = VX
            (0xF, _, 1, 8) => Self::SetStVx { x },
            // FX1E: add VX to I
            (0xF, _, 1, 0xE) => Self::AddVxToI { x },
            // FX29: font character
            (0xF, _, 2, 9) => Self::SetVxFontToI { x },
            // FX33: binary-coded decimal conversion
            (0xF, _, 3, 3) => Self::SetVxBcdToI { x },
            // FX55: save V0..VX into I
            (0xF, _, 5, 5) => Self::SaveVx { x },
            // FX65: load I into V0..VX
            (0xF, _, 6, 5) => Self::LoadVx { x },
            _ => Self::Unknown { opcode },
        }
    }
}


impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Nop => write!(f, "0000: Nop"),
            Self::ClearScreen => write!(f, "00E0: ClearScreen"),
            Self::Ret => write!(f, "00EE: Ret"),

            Self::Jump { nnn } => write!(f, "{:04X}: Jump {{ nnn: {:03X} }}", 0x1000 | nnn, nnn),
            Self::JumpPlusV0 { nnn } => write!(f, "{:04X}: JumpPlusV0 {{ nnn: {:03X} }}", 0xB000 | nnn, nnn),
            Self::Call { nnn } => write!(f, "{:04X}: Call {{ nnn: {:03X} }}", 0x2000 | nnn, nnn),

            Self::SkipVxEqNN { x, nn } => write!(f, "{:04X}: SkipVxEqNN {{ x: {:X}, nn: {:02X} }}", 0x3000 | (x as u16) << 8 | nn as u16, x, nn),
            Self::SkipVxNeqNN { x, nn } => write!(f, "{:04X}: SkipVxNeqNN {{ x: {:X}, nn: {:02X} }}", 0x4000 | (x as u16) << 8 | nn as u16, x, nn),
            Self::SkipVxEqVy { x, y } => write!(f, "{:04X}: SkipVxEqVy {{ x: {:X}, y: {:X} }}", 0x5000 | (x as u16) << 8 | (y as u16) << 4, x, y),
            Self::SkipVxNeqVy { x, y } => write!(f, "{:04X}: SkipVxNeqVy {{ x: {:X}, y: {:X} }}", 0x9000 | (x as u16) << 8 | (y as u16) << 4, x, y),

            Self::SetVxNN { x, nn } => write!(f, "{:04X}: SetVxNN {{ x: {:X}, nn: {:02X} }}", 0x6000 | (x as u16) << 8 | nn as u16, x, nn),
            Self::SetVxVy { x, y } => write!(f, "{:04X}: SetVxVy {{ x: {:X}, y: {:X} }}", 0x8000 | (x as u16) << 8 | (y as u16) << 4, x, y),
            Self::SetVxDt { x } => write!(f, "{:04X}: SetVxDt {{ x: {:X} }}", 0xF007 | (x as u16) << 8, x),
            Self::SetVxKey { x } => write!(f, "{:04X}: SetVxKey {{ x: {:X} }}", 0xF00A | (x as u16) << 8, x),
            Self::SetVxRnd { x, nn } => write!(f, "{:04X}: SetVxRnd {{ x: {:X}, nn: {:02X} }}", 0xC000 | (x as u16) << 8 | nn as u16, x, nn),
            Self::SetI { nnn } => write!(f, "{:04X}: SetI {{ nnn: {:03X} }}", 0xA000 | nnn, nnn),
            Self::SetVxFontToI { x } => write!(f, "{:04X}: SetVxFontToI {{ x: {:X} }}", 0xF029 | (x as u16) << 8, x),
            Self::SetVxBcdToI { x } => write!(f, "{:04X}: SetVxBcdToI {{ x: {:X} }}", 0xF033 | (x as u16) << 8, x),
            Self::SetDtVx { x } => write!(f, "{:04X}: SetDtVx {{ x: {:X} }}", 0xF015 | (x as u16) << 8, x),
            Self::SetStVx { x } => write!(f, "{:04X}: SetStVx {{ x: {:X} }}", 0xF018 | (x as u16) << 8, x),

            Self::AddVxNN { x, nn } => write!(f, "{:04X}: AddVxNN {{ x: {:X}, nn: {:02X} }}", 0x7000 | (x as u16) << 8 | nn as u16, x, nn),
            Self::AddVxVy { x, y } => write!(f, "{:04X}: AddVxVy {{ x: {:X}, y: {:X} }}", 0x8004 | (x as u16) << 8 | (y as u16) << 4, x, y),
            Self::SubVxVy { x, y } => write!(f, "{:04X}: SubVxVy {{ x: {:X}, y: {:X} }}", 0x8005 | (x as u16) << 8 | (y as u16) << 4, x, y),
            Self::SubVyVx { x, y } => write!(f, "{:04X}: SubVyVx {{ x: {:X}, y: {:X} }}", 0x8007 | (x as u16) << 8 | (y as u16) << 4, x, y),
            Self::AddVxToI { x } => write!(f, "{:04X}: AddVxToI {{ x: {:X} }}", 0xF01E | (x as u16) << 8, x),

            Self::OrVxVy { x, y } => write!(f, "{:04X}: OrVxVy {{ x: {:X}, y: {:X} }}", 0x8001 | (x as u16) << 8 | (y as u16) << 4, x, y),
            Self::AndVxVy { x, y } => write!(f, "{:04X}: AndVxVy {{ x: {:X}, y: {:X} }}", 0x8002 | (x as u16) << 8 | (y as u16) << 4, x, y),
            Self::XorVxVy { x, y } => write!(f, "{:04X}: XorVxVy {{ x: {:X}, y: {:X} }}", 0x8003 | (x as u16) << 8 | (y as u16) << 4, x, y),

            Self::RShiftVx { x, y } => write!(f, "{:04X}: RShiftVx {{ x: {:X}, y: {:X} }}", 0x8006 | (x as u16) << 8 | (y as u16) << 4, x, y),
            Self::LShiftVx { x, y } => write!(f, "{:04X}: LShiftVx {{ x: {:X}, y: {:X} }}", 0x800E | (x as u16) << 8 | (y as u16) << 4, x, y),

            Self::SkipVxDown { x } => write!(f, "{:04X}: SkipVxDown {{ x: {:X} }}", 0xE09E | (x as u16) << 8, x),
            Self::SkipVxUp { x } => write!(f, "{:04X}: SkipVxUp {{ x: {:X} }}", 0xE0A1 | (x as u16) << 8, x),

            Self::Draw { x, y, n } => write!(f, "{:04X}: Draw {{ x: {:X}, y: {:X}, n: {:X} }}", 0xD000 | (x as u16) << 8 | (y as u16) << 4 | n as u16, x, y, n),
            Self::SaveVx { x } => write!(f, "{:04X}: SaveVx {{ x: {:X} }}", 0xF055 | (x as u16) << 8, x),
            Self::LoadVx { x } => write!(f, "{:04X}: LoadVx {{ x: {:X} }}", 0xF065 | (x as u16) << 8, x),

            Self::Unknown { opcode } => write!(f, "{:04X}: Unknown", opcode),
        }
    }
}

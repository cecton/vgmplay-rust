use std::io::Read;

use bincode;

macro_rules! read {
    ($reader:expr) => {
        bincode::deserialize_from(&mut $reader)?
    }
}

#[derive(Debug, Deserialize)]
pub struct Header {
    pub file_id: [u8; 4],
    pub eof_offset: u32,
    pub version: u32,
    pub sn76489: u32,
    pub ym2413: u32,
    pub gd3_offset: u32,
    pub total_samples: u32,
    pub loop_offset: u32,
    pub loop_samples: u32,
    pub rate: u32,
    pub sn76489_feedback: u16,
    pub sn76489_shift_register_width: u8,
    pub reserved_2b: u8,
    pub ym2612: u32,
    pub ym2151: u32,
    pub data_offset: u32,
    pub reserved_38_3f: [u8; 8],
}

impl Header {
    pub fn new<R>(mut reader: R) -> bincode::Result<Header>
    where
        R: Read,
    {
        let mut header: Header = read!(reader);

        header.loop_samples -= 0x20;
        header.loop_samples += header.loop_offset;
        header.loop_offset += 0x1c;

        if header.version < 0x150 {
            header.data_offset = 0;
        }
        header.data_offset -= 12;

        Ok(header)
    }
}

#[derive(Debug)]
pub enum Command {
    GameGearPSGStereo(u8),
    PSG(u8),
    YM2612Port0(u8, u8),
    YM2612Port1(u8, u8),
    Wait(u16),
    End,
}

impl Command {
    pub fn new<R>(mut reader: R) -> bincode::Result<Command>
    where
        R: Read,
    {
        use Command::*;
        let code: u8 = read!(reader);

        match code {
            0x4f => Ok(GameGearPSGStereo(read!(reader))),
            0x50 => Ok(PSG(read!(reader))),
            0x52 => Ok(YM2612Port0(read!(reader), read!(reader))),
            0x53 => Ok(YM2612Port1(read!(reader), read!(reader))),
            0x61 => Ok(Wait(read!(reader))),
            0x66 => Ok(End),
            x @ 0x70..0x80 => Ok(Wait(x as u16 - 0x70)),
            _ => Err(Box::new(bincode::ErrorKind::Custom(format!(
                "unknown command code: {:x}",
                code
            )))),
        }
    }
}

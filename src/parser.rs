use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::Read;

error_chain::error_chain! {
    foreign_links {
        IO(std::io::Error);
        StringFromUtf8(std::string::FromUtf8Error);
    }
}

// == CPU ==
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cpu {
    pub rom_offset:    u32,
    pub entry_address: u32,
    pub load_address:  u32,
    pub size:          u32,

    pub overlay_offset: u32,
    pub overlay_length: u32,

    pub autoload: u32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            rom_offset:     0,
            entry_address:  0,
            load_address:   0,
            size:           0,
            overlay_offset: 0,
            overlay_length: 0,
            autoload:       0,
        }
    }
}

// == Table ==
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub offset: u32,
    pub length: u32,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            offset: 0,
            length: 0,
        }
    }
}

// == NDS Parser ==
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NDSParser {
    pub game_title:                            String,
    pub gamecode:                              String,
    pub makercode:                             String,
    pub unitcode:                              u8,
    pub encryption_seed_select:                u8,
    pub devicecapacity:                        u8,
    pub game_revision:                         u16,
    pub rom_version:                           u8,
    pub internal_flags:                        u8,
    pub arm9:                                  Cpu,
    pub arm7:                                  Cpu,
    pub fnt:                                   Table,
    pub fat:                                   Table,
    pub normal_card_control_register_settings: u32,
    pub secure_card_control_register_settings: u32,
    pub icon_banner_offset:                    u32,
    pub secure_area:                           u16,
    pub secure_transfer_timeout:               u16,

    pub secure_diable:                         u64,
    pub ntr_region_rom_size:                   u32,
    pub header_size:                           u32,
    pub nintendo_logo:                         [u8; 156],
    pub nintendo_logo_crc:                     u16,
    pub header_crc:                            u16,
    pub debugger:                              [u8; 32],
}

impl TryFrom<&str> for NDSParser {
    type Error = Error;

    fn try_from(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer: [u8; 0x181] = [0; 0x181];

        // fetch the information
        file.read_exact(&mut buffer)?;

        // store the values
        let game_title             = String::from_utf8(buffer[0..0xc].to_vec())?;
        let gamecode               = String::from_utf8(buffer[0xc..0x10].to_vec())?;
        let makercode              = String::from_utf8(buffer[0x10..0x12].to_vec())?;
        let unitcode               = u8::from_ne_bytes(buffer[0x12..0x13].try_into().unwrap());
        let encryption_seed_select = u8::from_ne_bytes(buffer[0x13..0x14].try_into().unwrap());
        let devicecapacity         = u8::from_ne_bytes(buffer[0x14..0x15].try_into().unwrap());
        let game_revision          = u16::from_ne_bytes(buffer[0x1c..0x1e].try_into().unwrap());
        let rom_version            = u8::from_ne_bytes(buffer[0x1e..0x1f].try_into().unwrap());
        let internal_flags         = u8::from_ne_bytes(buffer[0x1f..0x20].try_into().unwrap());
        let arm9 = Cpu {
            rom_offset:    u32::from_ne_bytes(buffer[0x20..0x24].try_into().unwrap()),
            entry_address: u32::from_ne_bytes(buffer[0x24..0x28].try_into().unwrap()),
            load_address:  u32::from_ne_bytes(buffer[0x28..0x2c].try_into().unwrap()),
            size:          u32::from_ne_bytes(buffer[0x2c..0x30].try_into().unwrap()),

            overlay_offset: u32::from_ne_bytes(buffer[0x50..0x54].try_into().unwrap()),
            overlay_length: u32::from_ne_bytes(buffer[0x54..0x58].try_into().unwrap()),

            autoload: u32::from_ne_bytes(buffer[0x70..0x74].try_into().unwrap()),
        };
        let arm7 = Cpu {
            rom_offset:    u32::from_ne_bytes(buffer[0x30..0x34].try_into().unwrap()),
            entry_address: u32::from_ne_bytes(buffer[0x34..0x38].try_into().unwrap()),
            load_address:  u32::from_ne_bytes(buffer[0x38..0x3c].try_into().unwrap()),
            size:          u32::from_ne_bytes(buffer[0x3c..0x40].try_into().unwrap()),

            overlay_offset: u32::from_ne_bytes(buffer[0x58..0x5c].try_into().unwrap()),
            overlay_length: u32::from_ne_bytes(buffer[0x5c..0x60].try_into().unwrap()),

            autoload: u32::from_ne_bytes(buffer[0x74..0x78].try_into().unwrap()),
        };
        let fnt = Table {
            offset: u32::from_ne_bytes(buffer[0x40..0x44].try_into().unwrap()),
            length: u32::from_ne_bytes(buffer[0x44..0x48].try_into().unwrap()),
        };
        let fat = Table {
            offset: u32::from_ne_bytes(buffer[0x48..0x4c].try_into().unwrap()),
            length: u32::from_ne_bytes(buffer[0x4c..0x50].try_into().unwrap()),
        };
        let normal_card_control_register_settings =
            u32::from_ne_bytes(buffer[0x60..0x64].try_into().unwrap());
        let secure_card_control_register_settings =
            u32::from_ne_bytes(buffer[0x64..0x68].try_into().unwrap());
        let icon_banner_offset       = u32::from_ne_bytes(buffer[0x68..0x6c].try_into().unwrap());
        let secure_area              = u16::from_ne_bytes(buffer[0x6c..0x6e].try_into().unwrap());
        let secure_transfer_timeout  = u16::from_ne_bytes(buffer[0x6e..0x70].try_into().unwrap());
        let secure_diable            = u64::from_ne_bytes(buffer[0x78..0x80].try_into().unwrap());
        let ntr_region_rom_size      = u32::from_ne_bytes(buffer[0x80..0x84].try_into().unwrap());
        let header_size              = u32::from_ne_bytes(buffer[0x84..0x88].try_into().unwrap());
        let nintendo_logo: [u8; 156] = buffer[0xc0..0x15c].try_into().unwrap();
        let nintendo_logo_crc        = u16::from_ne_bytes(buffer[0x15c..0x15e].try_into().unwrap());
        let header_crc               = u16::from_ne_bytes(buffer[0x15e..0x160].try_into().unwrap());
        let debugger                 = buffer[0x160..0x180].try_into().unwrap();

        Ok(Self {
            game_title,
            gamecode,
            makercode,
            unitcode,
            encryption_seed_select,
            devicecapacity,
            game_revision,
            rom_version,
            internal_flags,
            arm9,
            arm7,
            fnt,
            fat,
            normal_card_control_register_settings,
            secure_card_control_register_settings,
            icon_banner_offset,
            secure_area,
            secure_transfer_timeout,
            secure_diable,
            ntr_region_rom_size,
            header_size,
            nintendo_logo,
            nintendo_logo_crc,
            header_crc,
            debugger,
        })
    }
}

impl Default for NDSParser {
    fn default() -> Self {
        Self {
            game_title:                            String::new(),
            gamecode:                              String::new(),
            makercode:                             String::new(),
            unitcode:                              0,
            encryption_seed_select:                0,
            devicecapacity:                        0,
            game_revision:                         0,
            rom_version:                           0,
            internal_flags:                        0,
            arm9:                                  Cpu::default(),
            arm7:                                  Cpu::default(),
            fnt:                                   Table::default(),
            fat:                                   Table::default(),
            normal_card_control_register_settings: 0,
            secure_card_control_register_settings: 0,
            icon_banner_offset:                    0,
            secure_area:                           0,
            secure_transfer_timeout:               0,
            secure_diable:                         0,
            ntr_region_rom_size:                   0,
            header_size:                           0,
            nintendo_logo:                         [0; 156],
            nintendo_logo_crc:                     0,
            header_crc:                            0,
            debugger:                              [0; 32],
        }
    }
}

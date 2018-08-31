pub mod info;
pub mod offset;
pub mod rom;

use self::info::Info;
use self::offset::Offset;
use self::rom::InnerRom;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;

use std::path::Path;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Header {
    game_info: Info,
    encrypt_seed: u8,
    capacity: u8,
    reserved1: [u8; 7],
    revision: u16,
    rom_version: u8,
    flags: u8,
    arm9_rom: InnerRom,
    arm7_rom: InnerRom,
    file_name_table: Offset,
    file_alloc_table: Offset,
    arm9_overlay: Offset,
    arm7_overlay: Offset,
    normal_card_settings: u32,
    secure_card_settings: u32,
    icon_offset: u32,
    secure_crc: u16,
    secure_transfer_timeout: u16,
    arm9_autoload: u32,
    arm7_autoload: u32,
    secure_disable: u64,
    ntr_size: u32,
    header_size: u32,
    reserved2: [u8; 56],
    logo: [u8; 156],
    logo_crc: u16,
    header_crc: u16,
    debugger_reserved: [u8; 32],
}

impl Header {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        use std::fs::File;

        let mut file = File::open(path)?;

        let game_info = Info::new(&mut file)?;
        let encrypt_seed = file.read_u8()?;
        let capacity = file.read_u8()?;

        let mut reserved1 = [0u8; 7];
        file.read_exact(&mut reserved1)?;

        let revision = file.read_u16::<LittleEndian>()?;
        let rom_version = file.read_u8()?;
        let flags = file.read_u8()?;
        let arm9_rom = InnerRom::new(&mut file)?;
        let arm7_rom = InnerRom::new(&mut file)?;
        let file_name_table = Offset::new(&mut file)?;
        let file_alloc_table = Offset::new(&mut file)?;
        let arm9_overlay = Offset::new(&mut file)?;
        let arm7_overlay = Offset::new(&mut file)?;
        let normal_card_settings = file.read_u32::<LittleEndian>()?;
        let secure_card_settings = file.read_u32::<LittleEndian>()?;
        let icon_offset = file.read_u32::<LittleEndian>()?;
        let secure_crc = file.read_u16::<LittleEndian>()?;
        let secure_transfer_timeout = file.read_u16::<LittleEndian>()?;
        let arm9_autoload = file.read_u32::<LittleEndian>()?;
        let arm7_autoload = file.read_u32::<LittleEndian>()?;
        let secure_disable = file.read_u64::<LittleEndian>()?;
        let ntr_size = file.read_u32::<LittleEndian>()?;
        let header_size = file.read_u32::<LittleEndian>()?;

        let mut reserved2 = [0u8; 56];
        let mut logo = [0u8; 156];

        file.read_exact(&mut reserved2)?;
        file.read_exact(&mut logo)?;

        let logo_crc = file.read_u16::<LittleEndian>()?;
        let header_crc = file.read_u16::<LittleEndian>()?;
        let mut debugger_reserved = [0u8; 32];

        file.read_exact(&mut debugger_reserved)?;

        Ok(Self {
            game_info,
            encrypt_seed,
            capacity,
            reserved1,
            revision,
            rom_version,
            flags,
            arm9_rom,
            arm7_rom,
            file_name_table,
            file_alloc_table,
            arm9_overlay,
            arm7_overlay,
            normal_card_settings,
            secure_card_settings,
            icon_offset,
            secure_crc,
            secure_transfer_timeout,
            arm9_autoload,
            arm7_autoload,
            secure_disable,
            ntr_size,
            header_size,
            reserved2,
            logo,
            logo_crc,
            header_crc,
            debugger_reserved,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        self.game_info.write(writer)?;
        writer.write_u8(self.encrypt_seed)?;
        writer.write_u8(self.capacity)?;
        writer.write_all(&self.reserved1)?;
        writer.write_u16::<LittleEndian>(self.revision)?;
        writer.write_u8(self.rom_version)?;
        writer.write_u8(self.flags)?;
        self.arm9_rom.write(writer)?;
        self.arm7_rom.write(writer)?;
        self.file_name_table.write(writer)?;
        self.file_alloc_table.write(writer)?;
        self.arm9_overlay.write(writer)?;
        self.arm7_overlay.write(writer)?;
        writer.write_u32::<LittleEndian>(self.normal_card_settings)?;
        writer.write_u32::<LittleEndian>(self.secure_card_settings)?;
        writer.write_u32::<LittleEndian>(self.icon_offset)?;
        writer.write_u16::<LittleEndian>(self.secure_crc)?;
        writer.write_u16::<LittleEndian>(self.secure_transfer_timeout)?;
        writer.write_u32::<LittleEndian>(self.arm9_autoload)?;
        writer.write_u32::<LittleEndian>(self.arm7_autoload)?;
        writer.write_u64::<LittleEndian>(self.secure_disable)?;
        writer.write_u32::<LittleEndian>(self.ntr_size)?;
        writer.write_u32::<LittleEndian>(self.header_size)?;
        writer.write_all(&self.reserved2)?;
        writer.write_all(&self.logo)?;
        writer.write_u16::<LittleEndian>(self.logo_crc)?;
        writer.write_u16::<LittleEndian>(self.header_crc)?;
        writer.write_all(&self.debugger_reserved)?;

        Ok(())
    }
}
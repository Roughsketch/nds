use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InnerRom {
    offset: u32,
    entry_addr: u32,
    load_addr: u32,
    size: u32,
}

impl InnerRom {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(Self {
            offset: reader.read_u32::<LittleEndian>()?,
            entry_addr: reader.read_u32::<LittleEndian>()?,
            load_addr: reader.read_u32::<LittleEndian>()?,
            size: reader.read_u32::<LittleEndian>()?,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u32::<LittleEndian>(self.offset)?;
        writer.write_u32::<LittleEndian>(self.entry_addr)?;
        writer.write_u32::<LittleEndian>(self.load_addr)?;
        writer.write_u32::<LittleEndian>(self.size)?;

        Ok(())
    }
}
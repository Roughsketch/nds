use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Info {
    title: [u8; 12],
    gamecode: u32,
    makercode: u16,
    unitcode: u8,
}

impl Info {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut title = [0u8; 12];
        reader.read_exact(&mut title)?;

        Ok(Self {
            title,
            gamecode: reader.read_u32::<LittleEndian>()?,
            makercode: reader.read_u16::<LittleEndian>()?,
            unitcode: reader.read_u8()?,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.title)?;
        writer.write_u32::<LittleEndian>(self.gamecode)?;
        writer.write_u16::<LittleEndian>(self.makercode)?;
        writer.write_u8(self.unitcode)?;

        Ok(())
    }
}
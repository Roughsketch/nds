use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Offset {
    offset: u32,
    length: u32,
}

impl Offset {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(Self {
            offset: reader.read_u32::<LittleEndian>()?,
            length: reader.read_u32::<LittleEndian>()?,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u32::<LittleEndian>(self.offset)?;
        writer.write_u32::<LittleEndian>(self.length)?;

        Ok(())
    }
}
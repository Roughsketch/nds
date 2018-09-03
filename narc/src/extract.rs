use byteorder::{LittleEndian, ReadBytesExt};
use failure::{bail, ensure, Error};
use memmap::Mmap;

use std::fs::File;
use std::path::Path;

use crate::NarcError;

enum Header {
    Size = 0x08,
    FileCount = 0x18,
    FatOffset = 0x1C,
}

/// Extracts files from an Nitro Archive.
#[derive(Debug)]
pub struct Extractor {
    /// A memmap of the NARC to allow easy reading for potentially large files.
    data: Mmap,
}

impl Extractor {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let root = path.as_ref();

        let file = File::open(root)?;
        let data = unsafe { Mmap::map(&file)? };

        //  Minimum acceptable NARC size
        ensure!(data.len() > 0x1C, NarcError::NotEnoughData);

        //  All NARC files must start with "NARC"
        ensure!(&data[..4] == b"NARC", NarcError::InvalidHeader);

        let narc_size = (&data[Header::Size as usize..]).read_u32::<LittleEndian>()?;
        ensure!(data.len() == narc_size as usize, NarcError::SizeMismatch);

        Ok(Self {
            data,
        })
    }

    pub fn extract<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        use nitro_fs::FileSystem;

        let file_count = read_u16(Header::FileCount as usize)?;
        let fat_offset = Header::FatOffset as usize;

        let fat = &data[fat_offset..fat_offset + file_count * 8];

    }

    /// Reads a u16 from `data` at the given offset.
    fn read_u16(&self, offset: usize) -> Result<u16, Error> {
        let value = (&self.data[offset..]).read_u16::<LittleEndian>()?;
        Ok(value)
    }

    /// Reads a u32 from `data` at the given offset.
    fn read_u32(&self, offset: usize) -> Result<u32, Error> {
        let value = (&self.data[offset..]).read_u32::<LittleEndian>()?;
        Ok(value)
    }
}
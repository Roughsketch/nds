use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;

use std::io::{Cursor, Read};

#[fail(display = "FAT data has invalid size.")]
#[derive(Clone, Debug, Fail)]
struct InvalidFatLen;

/// Represents an entry in the File Allocation Table.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AllocInfo {
    /// The offset to the start of the file relative to the ROM start.
    pub start: u32,
    /// Length of the file.
    pub end: u32,
}

impl AllocInfo {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Error> {
        Ok(Self {
            start: reader.read_u32::<LittleEndian>()?,
            end: reader.read_u32::<LittleEndian>()?,
        })
    }

    pub fn len(&self) -> u32 {
        self.end - self.start
    }
}

/// Wrapper for handling File Allocation Table stuff
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FileAllocTable {
    list: Vec<AllocInfo>,
}

impl FileAllocTable {
    pub fn new(fat: &[u8]) -> Result<Self, Error> {
        // Each entry is 8 bytes, so if not divisible by 8
        // then there is an issue with the passed data.
        ensure!(fat.len() % 8 == 0, InvalidFatLen);

        let mut list = Vec::new();
        let mut cursor = Cursor::new(fat);
        let entries = fat.len() / 8;

        for _ in 0..entries {
            list.push(AllocInfo::new(&mut cursor)?);
        }

        Ok(Self {
            list
        })
    }

    pub fn get(&self, id: u16) -> Option<AllocInfo> {
        if self.list.len() >= id as usize {
            return Some(self.list[id as usize]);
        }

        None
    }
}
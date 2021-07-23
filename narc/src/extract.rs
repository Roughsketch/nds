use byteorder::{LittleEndian, ReadBytesExt};
use memmap::Mmap;
use num::NumCast;
use rayon::prelude::*;

use std::fs::{create_dir_all, File};
use std::path::Path;

use anyhow::{Result, ensure};

// == Errors ==
#[derive(Fail, Debug)]
enum NarcError {
    #[error("Not enough data.")]
    NotEnoughData,

    #[error("NARC header size does not match length of data.")]
    SizeMismatch,

    #[error("Header is invalid.")]
    InvalidHeader,

    #[error("Could not write all files successfully: {0:?}")]
    WriteError(Vec<anyhow::Error>),
}

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
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
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

    pub fn extract<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        use nitro_fs::FileSystem;

        let file_count = self.read_u16(Header::FileCount as usize)? as usize;
        let fat_offset = Header::FatOffset as usize;

        let fat = &self.data[fat_offset..fat_offset + file_count * 8];

        create_dir_all(&path)?;

        let fs = FileSystem::default();
        let base = path.as_ref();

        // TODO: Grab FNT and create a NitroFS FileSystem.
        let errors = fs.files()
            .par_iter()
            .filter_map(|file| {
                match self.write(&base.join(&file.path), file.alloc.start, file.alloc.len()) {
                    Ok(_) => None,
                    Err(why) => Some(why),
                }
            })
            .collect::<Vec<Error>>();

        ensure!(errors.is_empty(), NarcError::WriteError(errors));
        Ok(())
    }

    /// Reads a u16 from `data` at the given offset.
    fn read_u16(&self, offset: usize) -> Result<u16> {
        let value = (&self.data[offset..]).read_u16::<LittleEndian>()?;
        Ok(value)
    }

    /// Reads a u32 from `data` at the given offset.
    fn read_u32(&self, offset: usize) -> Result<u32> {
        let value = (&self.data[offset..]).read_u32::<LittleEndian>()?;
        Ok(value)
    }

    /// A utility to make it easier to write chunks of the ROM to files.
    /// Copies `len` bytes from the ROM starting from `offset` into the file 
    /// denoted by `path`
    fn write<P, N1, N2>(&self, path: P, offset: N1, len: N2) -> Result<()>
        where
            P: AsRef<Path>,
            N1: NumCast,
            N2: NumCast
    {
        use std::fs::write;

        let offset: usize = NumCast::from(offset).unwrap();
        let len: usize = NumCast::from(len).unwrap();

        ensure!(self.data.len() >= offset + len, NarcError::NotEnoughData);

        {
            let parent = path.as_ref().parent().unwrap_or(Path::new(""));

            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        write(path, &self.data[offset..offset + len])?;

        Ok(())
    }
}

use byteorder::{LittleEndian, ReadBytesExt};
use failure::{fail, Error};
use memmap::Mmap;
use num::NumCast;
use rayon::prelude::*;

use std::fs::{create_dir_all, File};
use std::path::Path;

#[fail(display = "Invalid NDS rom or directory.")]
#[derive(Clone, Debug, Fail)]
pub struct InvalidRomError;

#[fail(display = "Not enough data.")]
#[derive(Clone, Debug, Fail)]
struct NotEnoughData;

#[fail(display = "Header checksum does not match contents.")]
#[derive(Clone, Debug, Fail)]
struct InvalidChecksum;

/// Extracts files from an NDS ROM.
#[derive(Debug)]
pub struct Extractor {
    /// A memmap of the ROM to allow easy reading for potentially large files.
    data: Mmap,
}

impl Extractor {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let root = path.as_ref();

        let file = File::open(root)?;
        let data = unsafe { Mmap::map(&file)? };
        let checksum = (&data[0x15E..]).read_u16::<LittleEndian>()?;

        ensure!(util::crc16(data[..0x15E]) == checksum, InvalidChecksum);

        Ok(Self {
            data,
        })
    }

    /// Extracts the ROM to the path given. An error is returned
    /// if there are issues with the ROM structure, or if there is
    /// an issue writing files.
    pub fn extract<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        use fs::FileSystem;

        let root = path.as_ref();

        create_dir_all(root)?;

        self.write(root.join("header.bin"), 0, 0x200)?;
        self.write(root.join("arm9.bin"), self.read_u32(0x20)?, self.read_u32(0x2C)?)?;
        self.write(root.join("arm7.bin"), self.read_u32(0x30)?, self.read_u32(0x3C)?)?;

        let overlay_path = root.join("overlay");
        let file_path = root.join("data");

        create_dir_all(&overlay_path)?;
        create_dir_all(&file_path)?;

        let fs = FileSystem::new(self.fnt()?, self.fat()?)?;

        fs.overlays()
            .par_iter()
            .for_each(|file| {
                if let Err(why) = self.write(&overlay_path.join(&file.path), file.alloc.start, file.alloc.len()) {
                    eprintln!("Could not write file: {}", why);
                }
            });

        fs.files()
            .par_iter()
            .for_each(|file| {
                if let Err(why) = self.write(&file_path.join(&file.path), file.alloc.start, file.alloc.len()) {
                    eprintln!("Could not write file: {}", why);
                }
            });

        Ok(())
    }

    /// A utility to make it easier to write chunks of the ROM to files.
    /// Copies `len` bytes from the ROM starting from `offset` into the file 
    /// denoted by `path`
    fn write<P, N1, N2>(&self, path: P, offset: N1, len: N2) -> Result<(), Error>
        where
            P: AsRef<Path>,
            N1: NumCast,
            N2: NumCast
    {
        use std::fs::write;

        let offset: usize = NumCast::from(offset).unwrap();
        let len: usize = NumCast::from(len).unwrap();

        {
            let parent = path.as_ref().parent().unwrap_or(Path::new(""));

            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        write(path, &self.data[offset..offset + len])?;

        Ok(())
    }

    /// Reads a u32 from `data` at the given offset.
    fn read_u32(&self, offset: usize) -> Result<u32, Error> {
        let value = (&self.data[offset..]).read_u32::<LittleEndian>()?;
        Ok(value)
    }

    fn fat(&self) -> Result<&[u8], Error> {
        let fat_start = self.read_u32(0x48)? as usize;
        let fat_len = self.read_u32(0x4C)? as usize;

        ensure!(self.data.len() > fat_start + fat_len, NotEnoughData);

        Ok(&self.data[fat_start..fat_start + fat_len])
    }

    fn fnt(&self) -> Result<&[u8], Error> {
        let fnt_start = self.read_u32(0x40)? as usize;
        let fnt_len = self.read_u32(0x44)? as usize;

        ensure!(self.data.len() > fnt_start + fnt_len, NotEnoughData);

        Ok(&self.data[fnt_start..fnt_start + fnt_len])
    }
}
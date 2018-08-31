use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use memmap::Mmap;
use num::{Num, NumCast};
use rayon::prelude::*;

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use fs::{fat::FileAllocTable, fnt::FileNameTable};

#[fail(display = "Invalid NDS rom or directory.")]
#[derive(Clone, Debug, Fail)]
pub struct InvalidRomError;

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
        
        Ok(Self {
            data: unsafe { Mmap::map(&file)? },
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
                let alloc = fs.alloc_info(file.id).unwrap();

                if let Err(why) = self.write(&overlay_path.join(&file.path), alloc.start, alloc.len()) {
                    eprintln!("Could not write file: {}", why);
                }
            });

        fs.files()
            .par_iter()
            .for_each(|file| {
                let alloc = fs.alloc_info(file.id).unwrap();

                if let Err(why) = self.write(&file_path.join(&file.path), alloc.start, alloc.len()) {
                    eprintln!("Could not write file: {}", why);
                }
            });

        Ok(())
    }

    /// A utility to make it easier to write chunks of the ROM to files.
    /// Copies `len` bytes from the ROM starting from `offset` into the file 
    /// denoted by `path`
    fn write<P, R1, R2>(&self, path: P, offset: R1, len: R2) -> Result<(), Error>
        where
            P: AsRef<Path>,
            R1: Num + NumCast,
            R2: Num + NumCast
    {
        let offset: usize = NumCast::from(offset).unwrap();
        let len: usize = NumCast::from(len).unwrap();

        {
            let parent = path.as_ref().parent().unwrap_or(Path::new(""));

            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        let mut file = File::create(path)?;
        file.write_all(&self.data[offset..offset + len])?;

        Ok(())
    }

    /// Reads a u32 from `data` at the given offset.
    fn read_u32(&self, offset: usize) -> Result<u32, Error> {
        let value = (&self.data[offset..]).read_u32::<LittleEndian>()?;
        Ok(value)
    }

    fn fat(&self) -> Result<FileAllocTable, Error> {
        let fat_start = self.read_u32(0x48)? as usize;
        let fat_len = self.read_u32(0x4C)? as usize;

        FileAllocTable::new(&self.data[fat_start..fat_start + fat_len])
    }

    fn fnt(&self) -> Result<FileNameTable, Error> {
        let fnt_start = self.read_u32(0x40)? as usize;
        let fnt_len = self.read_u32(0x44)? as usize;

        FileNameTable::new(&self.data[fnt_start..fnt_start + fnt_len])
    }
}
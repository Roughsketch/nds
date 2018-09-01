use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use rayon::prelude::*;

use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::path::Path;

pub mod fat;
pub mod fnt;

use self::fat::FileAllocTable;
use self::fnt::{Directory, FileEntry, ROOT_ID};

/// Represents a NitroROM file system.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileSystem {
    pub dirs: BTreeMap<u16, Directory>,
    overlays: Vec<FileEntry>,
}

impl FileSystem {
    pub fn new(fnt: &[u8], fat: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(fnt);
        let mut dirs = BTreeMap::new();

        cursor.set_position(6);
        let count = cursor.read_u16::<LittleEndian>()?;

        cursor.set_position(0);

        for index in 0..count {
            let id = ROOT_ID + index;
            dirs.insert(id, Directory::new(&mut cursor, id)?);
        }

        let fat = FileAllocTable::new(fat)?;

        let mut fnt = Self { 
            dirs,
            overlays: Vec::new(),
        };

        fnt.populate(&mut cursor, &fat)?;

        Ok(fnt)
    }

    /// How many directories there are
    pub fn count(&self) -> usize {
        self.dirs.len()
    }
    
    /// Get a Vec of all files
    pub fn files(&self) -> Vec<&FileEntry> {
        self.dirs.par_iter().flat_map(|(_, ref dir)| {
            &dir.files
        }).collect::<_>()
    }

    /// The lowest ID in the File System. Any ID lower than this in 
    /// the FAT is an overlay file.
    pub fn start_id(&self) -> u16 {
        self.dirs[&ROOT_ID].start_id()
    }

    /// Get a Vec of all overlays
    pub fn overlays(&self) -> &[FileEntry] {
        &self.overlays
    }

    fn populate(&mut self, cursor: &mut Cursor<&[u8]>, fat: &FileAllocTable) -> Result<(), Error> {
        self._populate(cursor, "", ROOT_ID, fat)?;

        self.overlays = (0..self.start_id())
            .into_par_iter()
            .map(|id| {
                let alloc_info = fat.get(id).unwrap();
                FileEntry::new(id, &format!("overlay_{:04}", id), alloc_info)
            })
            .collect::<_>();

        Ok(())
    }

    fn _populate<P: AsRef<Path>>(&mut self, mut cursor: &mut Cursor<&[u8]>, path: P, id: u16, fat: &FileAllocTable) -> Result<(), Error> {
        let mut file_id = {
            let dir = self.dirs.get_mut(&id).unwrap();
            dir.set_path(&path);
            cursor.set_position(dir.offset() as u64);
            dir.start_id()
        };

        let mut files = Vec::new();

        let mut len = cursor.read_u8()?;

        while len != 0 {
            let name = self.read_name(&mut cursor, len)?;

            if len > 0x80 {
                //  Read the directory ID that this name goes to
                let dir_id = cursor.read_u16::<LittleEndian>()?;

                let pos = cursor.position();
                let new_path = path.as_ref().join(name);
                
                self._populate(&mut cursor, new_path, dir_id, fat)?;

                cursor.set_position(pos);
            } else {
                let file_path = path.as_ref().join(name);
                let alloc_info = fat.get(file_id).unwrap();

                files.push(FileEntry::new(file_id, &file_path, alloc_info));
                file_id += 1;
            }

            len = cursor.read_u8()?;
        }

        let dir = self.dirs.get_mut(&id).unwrap();

        dir.append_files(&files);

        Ok(())
    }

    fn read_name<R: Read>(&self, cursor: &mut R, mut len: u8) -> Result<String, Error> {
        let mut name = String::new();

        if len > 0x80 {
            len -= 0x80;
        }

        cursor.take(u64::from(len))
            .read_to_string(&mut name)?;

        Ok(name)
    }
}
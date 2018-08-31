use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use rayon::prelude::*;

use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

/// The offset that directory IDs start at. The root
/// directory is ID 0xF000 and subsequent directories
/// are past that up to FFFF
static ROOT_ID: u16 = 0xF000;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FileEntry {
    pub id: u16,
    pub path: PathBuf,
}

impl FileEntry {
    pub fn new<P: AsRef<Path>>(id: u16, path: P) -> Self {
        Self {
            id,
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Directory {
    /// The id of the directory
    pub metadata: DirectoryInfo,
    /// Name of the directory
    pub path: PathBuf,
    // Files that are inside this directory
    pub files: Vec<FileEntry>,

}

impl Directory {
    pub fn new(metadata: &DirectoryInfo) -> Self {
        Self {
            metadata: metadata.clone(),
            path: PathBuf::new(),
            files: Vec::new(),
        }
    }

    pub fn set_path<P: AsRef<Path>>(&mut self, path: P) {
        self.path = path.as_ref().to_path_buf();
    }

    pub fn offset(&self) -> u32 {
        self.metadata.offset
    }

    pub fn id(&self) -> u16 {
        self.metadata.id
    }

    pub fn start_id(&self) -> u16 {
        self.metadata.start_id
    }

    pub fn parent_id(&self) -> u16 {
        self.metadata.parent_id()
    }

    pub fn is_root(&self) -> bool {
        self.metadata.is_root()
    }

    pub fn append_file(&mut self, file: FileEntry) {
        self.files.push(file);
    }

    pub fn append_files(&mut self, files: &[FileEntry]) {
        self.files.extend_from_slice(files);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DirectoryInfo {
    offset: u32,
    start_id: u16,
    value: u16,
    id: u16,
}

impl DirectoryInfo {
    pub fn new<R: Read>(reader: &mut R, id: u16) -> Result<Self, Error> {
        Ok(Self {
            offset: reader.read_u32::<LittleEndian>()?,
            start_id: reader.read_u16::<LittleEndian>()?,
            value: reader.read_u16::<LittleEndian>()?,
            id,
        })
    }

    pub fn parent_id(&self) -> u16 {
        if self.is_root() {
            ROOT_ID
        } else {
            self.value
        }
    }

    pub fn is_root(&self) -> bool {
        self.id == ROOT_ID
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DirectoryTable {
    pub dirs: BTreeMap<u16, Directory>,
}

impl DirectoryTable {
    pub fn new(mut cursor: &mut Cursor<&[u8]>) -> Result<Self, Error> {
        let mut dirs = BTreeMap::new();

        cursor.set_position(6);
        let count = cursor.read_u16::<LittleEndian>()?;

        cursor.set_position(0);

        for index in 0..count {
            let id = ROOT_ID + index;
            dirs.insert(id, Directory::new(&DirectoryInfo::new(&mut cursor, id)?));
        }

        Ok(Self {
            dirs,
        })
    }

    pub fn count(&self) -> usize {
        self.dirs.len()
    }

    pub fn populate<P: AsRef<Path>>(&mut self, cursor: &mut Cursor<&[u8]>, path: P) -> Result<(), Error> {
        self._populate(cursor, path, ROOT_ID)?;

        Ok(())
    }

    fn _populate<P: AsRef<Path>>(&mut self, mut cursor: &mut Cursor<&[u8]>, path: P, id: u16) -> Result<(), Error> {
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
                
                self._populate(&mut cursor, new_path, dir_id)?;

                cursor.set_position(pos);
            } else {
                let file_path = path.as_ref().join(name);
                files.push(FileEntry::new(file_id, &file_path));
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileNameTable {
    table: DirectoryTable,
}

impl FileNameTable {
    pub fn new(fnt: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(fnt);

        let mut table = DirectoryTable::new(&mut cursor)?;
        table.populate(&mut cursor, "")?;

        Ok(Self {
            table,
        })
    }

    pub fn files(&self) -> Vec<&FileEntry> {
        self.table.dirs.par_iter().flat_map(|(_, ref dir)| {
            &dir.files
        }).collect::<_>()
    }

    pub fn start_id(&self) -> u16 {
        self.table.dirs[&ROOT_ID].start_id()
    }
}
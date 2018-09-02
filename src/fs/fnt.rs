use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::fs::fat::AllocInfo;

/// The offset that directory IDs start at. The root
/// directory is ID 0xF000 and subsequent directories
/// are past that up to FFFF
pub static ROOT_ID: u16 = 0xF000;

/// Represents a NitroROM file entry.
/// 
/// # Notes
/// `path` will be the full path relative to the root of the
/// file name table. 
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FileEntry {
    pub id: u16,
    pub path: PathBuf,
    pub alloc: AllocInfo,
}

impl FileEntry {
    pub fn new<P: AsRef<Path>>(id: u16, path: P, alloc: AllocInfo) -> Self {
        Self {
            id,
            path: path.as_ref().to_path_buf(),
            alloc,
        }
    }
}

/// Represents a NitroROM directory.
/// 
/// # Notes
/// `path` will be the full path relative to the root of the
/// file name table. 
/// 
/// `value` for the root directory is the number of directories
/// in the file name table. For non-root directories, it is the
/// ID of the directory's parent.
/// 
/// Because the root directory has no parent ID value, a `parent_id`
/// call on the root directory will return `ROOT_ID`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Directory {
    /// Name of the directory
    pub path: PathBuf,
    // Files that are inside this directory
    pub files: Vec<FileEntry>,
    offset: u32,
    start_id: u16,
    value: u16,
    id: u16,

}

impl Directory {
    pub fn new<R: Read>(reader: &mut R, id: u16) -> Result<Self, Error> {
        Ok(Self {
            path: PathBuf::new(),
            files: Vec::new(),
            offset: reader.read_u32::<LittleEndian>()?,
            start_id: reader.read_u16::<LittleEndian>()?,
            value: reader.read_u16::<LittleEndian>()?,
            id
        })
    }

    /// Sets the full path that this directory is referenced by.
    pub fn set_path<P: AsRef<Path>>(&mut self, path: P) {
        self.path = path.as_ref().to_path_buf();
    }

    /// The offset into the FNT where this directory's info is stored.
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// The ID of the directory.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// The ID of the first file in the directory's subtable.
    pub fn start_id(&self) -> u16 {
        self.start_id
    }

    /// The ID of the parent directory.
    /// 
    /// If this is the root directory, `ROOT_ID` will be returned instead.
    pub fn parent_id(&self) -> u16 {
        if self.is_root() {
            ROOT_ID
        } else {
            self.value
        }
    }

    /// Whether this directory is the root.
    pub fn is_root(&self) -> bool {
        self.id == ROOT_ID
    }

    /// Appends a file to the file list associated with this directory.
    pub fn append_file(&mut self, file: FileEntry) {
        self.files.push(file);
    }

    /// Appends several files to the list associated with this directory.
    pub fn append_files(&mut self, files: &[FileEntry]) {
        self.files.extend_from_slice(files);
    }
}
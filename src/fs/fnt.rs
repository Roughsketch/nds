use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;

use std::io::Read;
use std::path::{Path, PathBuf};

use fs::fat::AllocInfo;

/// The offset that directory IDs start at. The root
/// directory is ID 0xF000 and subsequent directories
/// are past that up to FFFF
pub static ROOT_ID: u16 = 0xF000;

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
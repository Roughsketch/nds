use failure::Error;

pub mod fat;
pub mod fnt;

use self::fat::{AllocInfo, FileAllocTable};
use self::fnt::{FileEntry, FileNameTable};

/// Represents an entry in the File System Table.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FstEntry {
    /// The id of the FST node.
    id: u16,
    /// The name of the file or folder.
    name: String,
    /// If the entry is a directory, it will have child entries.
    children: Option<Vec<FstEntry>>,
    /// If the entry is a file, it will have an allocation table entry.
    alloc: Option<self::fat::AllocInfo>,
}

impl FstEntry {
    pub fn new(id: u16, name: &str, children: Option<Vec<FstEntry>>, alloc: Option<self::fat::AllocInfo>) -> Self {
        Self {
            id,
            name: name.into(),
            children,
            alloc,
        }
    }
}

/// Represents the File System Table. Holds the root node of a tree
/// which holds all the files and directories.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileSystem {
    fat: FileAllocTable,
    fnt: FileNameTable,
}

impl FileSystem {
    /// Creates a File System Table given the raw ROM data.
    /// In order to get all the information to find files,
    /// the File Allocation Table is passed in as well.
    pub fn new(fnt: FileNameTable, fat: FileAllocTable) -> Result<Self, Error> {
        Ok(Self {
            fat,
            fnt,
        })
    }

    pub fn files(&self) -> Vec<&FileEntry> {
        self.fnt.files()
    }

    pub fn overlays(&self) -> Vec<FileEntry> {
        let start = self.fnt.start_id();
        let mut overlays = Vec::new();

        for id in 0..start {
            overlays.push(FileEntry::new(id, &format!("overlay_{:04}", id)));
        }

        overlays
    }

    pub fn alloc_info(&self, id: u16) -> Option<AllocInfo> {
        self.fat.get(id)
    }
}
use failure::{bail, ensure, Error};

use std::fs::read;
use std::path::{Path, PathBuf};

use nitro_fs::FileSystem;

use crate::NdsError;

/// Builds an NDS ROM given a directory with valid structure.
/// A directory is valid if [`is_nds_dir`] returns `Ok`
/// 
/// [`is_nds_dir`]: struct.Builder.html#method.is_nds_dir
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Builder {
    root: PathBuf,
}

impl Builder {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let root = path.as_ref();
    
        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    /// Determines whether a given path is a valid NDS ROM.
    /// A valid NDS ROM directory is made when a ROM is extracted
    /// with an [`Extractor`] and includes the following:
    /// 
    /// ./data/
    /// ./overlay/
    /// ./header.bin
    /// ./arm9.bin
    /// ./arm7.bin
    /// 
    /// Due to race conditions, the validity is not a guarantee that 
    /// the directory is valid through the duration of program execution, 
    /// so errors can still be thrown for missing files.
    /// 
    /// [`Extractor`]: struct.Extractor.html#method.is_nds_dir
    pub fn is_nds_dir<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        let root = path.as_ref();

        //  Check root
        ensure!(root.is_dir(), NdsError::MissingFolderError("root"));

        //  Check system directories
        ensure!(root.join("data").is_dir(), NdsError::MissingFolderError("data"));
        ensure!(root.join("overlay").is_dir(), NdsError::MissingFolderError("overlay"));

        //  Check system files
        ensure!(root.join("arm9_overlay.bin").is_file(), NdsError::MissingFileError("arm9_overlay.bin"));
        ensure!(root.join("arm7_overlay.bin").is_file(), NdsError::MissingFileError("arm7_overlay.bin"));
        ensure!(root.join("arm9.bin").is_file(), NdsError::MissingFileError("arm9.bin"));
        ensure!(root.join("arm7.bin").is_file(), NdsError::MissingFileError("arm7.bin"));
        ensure!(root.join("header.bin").is_file(), NdsError::MissingFileError("header.bin"));

        Ok(())
    }

    /// Builds a ROM and saves it to the path given. This method will
    /// return an error when the directory is missing required files,
    /// or if there is an issue reading files or saving the ROM.
    pub fn build<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let output = path.as_ref();

        // let header = read(self.root.join("header.bin"))?;

        // let fs = FileSystem::build(self.root)?;

        Ok(())
    }
}


use failure::Error;
use std::path::{Path, PathBuf};

#[fail(display = "Missing required directory: '{}'.", _0)]
#[derive(Clone, Debug, Fail)]
struct MissingFolderError(&'static str);

#[fail(display = "Missing required file: '{}'.", _0)]
#[derive(Clone, Debug, Fail)]
struct MissingFileError(&'static str);

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
        ensure!(root.is_dir(), MissingFolderError("root"));

        //  Check system directories
        ensure!(root.join("data").is_dir(), MissingFolderError("data"));
        ensure!(root.join("overlay").is_dir(), MissingFolderError("overlay"));

        //  Check system files
        ensure!(root.join("arm9_overlay.bin").is_file(), MissingFileError("arm9_overlay.bin"));
        ensure!(root.join("arm7_overlay.bin").is_file(), MissingFileError("arm7_overlay.bin"));
        ensure!(root.join("arm9.bin").is_file(), MissingFileError("arm9.bin"));
        ensure!(root.join("arm7.bin").is_file(), MissingFileError("arm7.bin"));
        ensure!(root.join("header.bin").is_file(), MissingFileError("header.bin"));

        Ok(())
    }

    /// Builds a ROM and saves it to the path given. This method will
    /// return an error when the directory is missing required files,
    /// or if there is an issue reading files or saving the ROM.
    pub fn build<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let output = path.as_ref();


        Ok(())
    }
}


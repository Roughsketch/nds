use failure::{Error, Fail};

mod build;
mod extract;
pub mod util;

pub use crate::build::Builder;
pub use crate::extract::Extractor;

#[derive(Debug, Fail)]
enum NdsError {
    #[fail(display = "Invalid NDS rom or directory.")]
    InvalidRomError,

    #[fail(display = "Not enough data.")]
    NotEnoughData,

    #[fail(display = "Header checksum does not match contents.")]
    InvalidChecksum,

    #[fail(display = "Could not write all files successfully: {:?}", _0)]
    WriteError(Vec<Error>),

    #[fail(display = "Missing required directory: '{}'.", _0)]
    MissingFolderError(&'static str),

    #[fail(display = "Missing required file: '{}'.", _0)]
    MissingFileError(&'static str),
}

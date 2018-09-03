#![feature(custom_attribute)]

use failure::Fail;
mod extract;

pub use crate::extract::Extractor;

#[derive(Fail, Debug)]
enum NarcError {
    #[fail(display = "Not enough data.")]
    NotEnoughData,

    #[fail(display = "NARC header size does not match length of data.")]
    SizeMismatch,

    #[fail(display = "Header is invalid.")]
    InvalidHeader,
}
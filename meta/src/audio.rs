use thiserror::Error;
use std::io;


#[cfg(feature = "ape")]
pub use crate::ape::extract_meta as extract_ape_meta;

#[cfg(feature = "id3")]
pub use crate::id3::extract_meta as extract_id3_meta;


#[derive(Error, Debug)]
pub enum AudioMetaError {
    #[error("io error `{0}`")]
    IOError(#[from] io::Error),

    #[cfg(feature = "ape")]
    #[error("ape error `{0}`")]
    ApeError(#[from] ape::Error),

    #[cfg(feature = "id3")]
    #[error("id3 error `{0}`")]
    Id3Error(#[from] id3::Error),
}
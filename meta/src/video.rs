use std::io;
use thiserror::Error;
use mp4::Error as Mp4Error;


#[derive(Error, Debug)]
pub enum VideoMetaError {
    #[error("io error `{0}`")]
    IOError(#[from] io::Error),

    #[cfg(feature = "matroska")]
    #[error("matroska error `{0}`")]
    MatroskaError(#[from] matroska::MatroskaError),

    #[cfg(feature = "mp4")]
    #[error("mp4 error `{0}`")]
    Mp4Error(#[from] Mp4Error),
}


#[cfg(feature = "matroska")]
pub use crate::matroska::extract_meta as extract_mkv_meta;

#[cfg(feature = "mp4")]
pub use crate::mp4::extract_meta as extract_mp4_meta;

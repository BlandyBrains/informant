use log::error;
use thiserror::Error;

use crate::{VideoMetaError, AudioMetaError, ImageMetaError, exif::ExifMetaError};

#[derive(Error, Debug)]
pub enum MetaError {
    #[error("video meta error `{0}`")]
    VideoError(#[from] VideoMetaError),

    #[error("audio meta error `{0}`")]
    AudioError(#[from] AudioMetaError),

    #[error("audio meta error `{0}`")]
    ImageError(#[from] ImageMetaError),

    #[error("exif error `{0}`")]
    ExifError(#[from] ExifMetaError),

    #[cfg(feature = "heic")]
    #[error("heic error `{0}`")]
    HeicMetaError(#[from] crate::heic::HeicMetaError),

    #[cfg(feature = "mp4")]
    #[error("mp4 error `{0}`")]
    MP4Error(#[from] mp4::Error),
}
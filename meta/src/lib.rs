use std::fmt;
use std::path::Path;

// Common meta objects
mod meta;
mod image;

use general::General;
use image::CommonImageMeta;
pub use meta::Meta;

pub use crate::meta::{MetaClass, MetaAttribute, MetaValue, MetaSource, MetaType};

type MetaError = Box<dyn std::error::Error + 'static>;

#[derive(Debug, Clone)]
struct NoExtractorError {
    message: String,
}

// Implement the std::fmt::Display trait for the error type
impl fmt::Display for NoExtractorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for NoExtractorError {}

/// Fundamental trait for extracting metadata.
pub trait Extractor{
    fn extract(&self, meta: &mut Meta) -> Result<(), MetaError>;
}

pub trait FromFile {
    fn file(path: &str) -> Self;
}

mod general;

#[cfg(feature="libheif-rs")]
mod heic;

#[cfg(feature = "ape")]
mod ape;

#[cfg(feature = "id3")]
mod id3;

#[cfg(feature = "kamadak-exif")]
mod exif;

#[cfg(feature = "matroska")]
mod matroska;

#[cfg(feature = "mp4")]
mod mp4;

#[cfg(feature = "blake3")]
mod hash;


/// Search and collect extractors by file extension.
pub fn get_extractors(file_path: &str) -> Result<Vec<Box<dyn Extractor + 'static>>, MetaError> {
    let extension: String = match Path::new(file_path).extension() {
        Some(x) => { x.to_str().unwrap().trim().to_ascii_lowercase() },
        _ => { panic!("missing file extension") }
    };

    let mut extractors: Vec<Box<dyn Extractor>> = vec![
        // universal extractors
        Box::new(General::file(file_path)),

        #[cfg(feature = "hash")]
        {
            use crate::hash::MetaHash;
            Box::new(MetaHash::file(file_path))
        }
    ];

    match extension.as_str() {
        "mkv" => {
            #[cfg(feature = "matroska")]
            {
                use crate::matroska::Matroska;
                extractors.push(Box::new(Matroska::file(file_path)));
            }
        },
        "m4a" => {
            #[cfg(feature = "mp4")]
            {
                use crate::mp4::MP4;
                extractors.push(Box::new(MP4::file(file_path)))
            }

            #[cfg(feature = "ape")]
            {
                use crate::ape::Ape;
                extractors.push(Box::new(Ape::file(file_path)))
            }

            #[cfg(feature = "id3")]
            {
                use crate::id3::ID3;
                extractors.push(Box::new(ID3::file(file_path)))
            }
        }
        "mp4" | "mov" | "m4v" => {
            #[cfg(feature = "mp4")]
            {
                use crate::mp4::MP4;
                extractors.push(Box::new(MP4::file(file_path)))
            }
        },
        "amr" | "mp3" | "wav" | "flac"  | "wma" | "m4r" => {
            #[cfg(feature = "ape")]
            {
                use crate::ape::Ape;
                extractors.push(Box::new(Ape::file(file_path)))
            }

            #[cfg(feature = "id3")]
            {
                use crate::id3::ID3;
                extractors.push(Box::new(ID3::file(file_path)))
            }
        },
        "heic" | "heif" | "jpeg" | "jpg" | "png" | "raf" | "tif" | "tiff" | "cr2" | "jfif" => {

            extractors.push(Box::new(CommonImageMeta::file(file_path)));

            #[cfg(feature = "heic")]
            {
                use crate::heic::Heic;
                extractors.push(Box::new(Heic::file(file_path)))
            }

            // EXIF could be in almost any format.
            // Optimistically, we'll try to extract for each format.
            #[cfg(feature = "exif")]
            {
                use crate::exif::ExifExtractor;
                extractors.push(Box::new(ExifExtractor::file(file_path)))
            }
        },
        _ => {
            println!("reverting to universal extractors {:#?}", extension);
        }
    }
    Ok(extractors)
}


#[cfg(test)]
mod test {
    use std::fs;
    use crate::{get_extractors, Meta};

    #[test]
    fn test_get_extrators() {
        let extractors = get_extractors("some/path/to/asset.mp3").unwrap();
        assert_eq!(extractors.len(), 2);

        let extractors = get_extractors("some/path/to/asset.mkv").unwrap();
        assert_eq!(extractors.len(), 1);

        let extractors = get_extractors("some/path/to/asset.mp4").unwrap();
        assert_eq!(extractors.len(), 1);

        let extractors = get_extractors("some/path/to/asset.m4a").unwrap();
        assert_eq!(extractors.len(), 3);

        let extractors = get_extractors("some/path/to/asset.jpeg").unwrap();
        assert_eq!(extractors.len(), 3);

        let _ = match get_extractors("some/path/to/asset.123") {
            Ok(_) => { panic!("this should've failed...")},
            Err(e) => {
                assert_eq!(e.to_string(), "no extractors for extension: \"123\"");
            }
        };
    }

    #[test]
    fn test_audio_extractors() {
        let directory_path = "../testdata/Audio";
 
        if let Ok(entries) = fs::read_dir(directory_path) {
            // Iterate over the entries in the directory
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                        continue;
                    }

                    if let Some(file_path) = entry.path().to_str() {
                        let extractors = get_extractors(file_path).unwrap();
                        let mut meta: Meta = Meta::new();

                        for ex in extractors {
                            match ex.extract(&mut meta) {
                                Ok(_) => {},
                                Err(e) => {
                                    println!("error {:#?}", e);
                                }
                            }
                        }
                        println!("\n\n");
                        println!("File: {:#?}", file_path);
                        println!("{:#?}", meta);
                        println!("\n\n");
                    }                    
                }
            }
        } 
    }

    #[test]
    fn test_video_extractors() {
        let directory_path = "../testdata/Video";
 
        if let Ok(entries) = fs::read_dir(directory_path) {
            // Iterate over the entries in the directory
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                        continue;
                    }

                    if let Some(file_path) = entry.path().to_str() {
                        let extractors = get_extractors(file_path).unwrap();
                        let mut meta: Meta = Meta::new();

                        for ex in extractors {
                            match ex.extract(&mut meta) {
                                Ok(_) => {},
                                Err(e) => {
                                    println!("error {:#?}", e);
                                }
                            }
                        }
                        println!("\n\n");
                        println!("File: {:#?}", file_path);
                        println!("{:#?}", meta);
                        println!("\n\n");
                    }                    
                }
            }
        } 
    }
}

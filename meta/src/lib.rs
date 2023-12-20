use std::fmt;
use std::path::Path;

// Common meta objects
mod meta;
mod image;

use image::CommonImageMeta;

pub use crate::meta::{MetaClass, MetaAttribute, MetaValue, MetaFormat, MetaSource, MetaType};

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
trait Extractor{
    fn extract(&self, meta: &mut Vec<MetaAttribute>) -> Result<(), MetaError>;
}

trait Detail {
    fn new(file_path: &str) -> Self;
}

#[cfg(feature="heic")]
mod heic;

#[cfg(feature = "ape")]
mod ape;

#[cfg(feature = "id3")]
mod id3;

#[cfg(feature = "exif")]
mod exif;

#[cfg(feature = "matroska")]
mod matroska;

#[cfg(feature = "mp4")]
mod mp4;


/// Search and collect extractors by file extension.
fn get_extractors(file_path: &str) -> Result<Vec<Box<dyn Extractor + 'static>>, MetaError> {
    let extension: String = match Path::new(file_path).extension() {
        Some(x) => { x.to_str().unwrap().trim().to_ascii_lowercase() },
        _ => { panic!("missing file extension") }
    };

    match extension.as_str() {
        "mkv" => {
            #[cfg(feature = "matroska")]
            {
                use crate::matroska::Matroska;
                Ok(vec![Box::new(Matroska::new(file_path))])
            }
        },
        "m4a" => {
            Ok(vec![
                #[cfg(feature = "mp4")]
                {
                    use crate::mp4::MP4;
                    Box::new(MP4::new(file_path))
                },

                #[cfg(feature = "ape")]
                {
                    use crate::ape::Ape;
                    Box::new(Ape::new(file_path))
                },

                #[cfg(feature = "id3")]
                {
                    use crate::id3::ID3;
                    Box::new(ID3::new(file_path))
                }
            ])
        }
        "mp4" | "mov" | "m4v" => {
            #[cfg(feature = "mp4")]
            {
                use crate::mp4::MP4;
                Ok(vec![Box::new(MP4::new(file_path))])
            }
        },
        "amr" | "mp3" | "wav" | "flac"  | "wma" | "m4r" => {
            Ok(vec![
                #[cfg(feature = "ape")]
                {
                    use crate::ape::Ape;
                    Box::new(Ape::new(file_path))
                },

                #[cfg(feature = "id3")]
                {
                    use crate::id3::ID3;
                    Box::new(ID3::new(file_path))
                }
            ])
        },
        "heic" | "heif" | "jpeg" | "jpg" | "png" | "raf" | "tif" | "tiff" | "cr2" | "jfif" => {
            Ok(vec![
                Box::new(CommonImageMeta::new(file_path)),

                #[cfg(feature = "heic")]
                {
                    use crate::heic::Heic;
                    Box::new(Heic::new(file_path))
                },

                // EXIF could be in almost any format.
                // Optimistically, we'll try to extract for each format.
                #[cfg(feature = "exif")]
                {
                    use crate::exif::ExifExtractor;
                    Box::new(ExifExtractor::new(file_path))
                }
            ])
        },
        _ => {
            Err(Box::new(NoExtractorError{message: format!("no extractors for extension: {:#?}", extension)}))
        }
    }
}


#[cfg(test)]
mod test {
    use std::fs;
    use crate::{get_extractors, MetaAttribute};

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
                        let mut meta: Vec<MetaAttribute> = Vec::new();

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
                        let mut meta: Vec<MetaAttribute> = Vec::new();

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

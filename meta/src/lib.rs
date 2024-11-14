use std::path::Path;

// Common meta objects
mod meta;
mod image;

use general::General;
use image::CommonImageMeta;
pub use meta::Meta;

pub use crate::meta::{MetaClass, MetaAttribute, MetaValue, MetaSource, MetaType};

type MetaError = Box<dyn std::error::Error + 'static>;

/// Fundamental trait for extracting metadata.
pub trait Extractor{
    fn name(&self) -> String;
    fn extract(&self, meta: &mut Meta) -> Result<(), MetaError>;
}

pub trait FromFile {
    fn file(path: &str) -> Self;
}

pub struct Extractors(Vec<Box<dyn Extractor>>);

impl Extractors {
    pub fn size(&self) -> usize {
        return self.0.len();
    }
    pub fn find(&self, name: &str) -> Result<&Box<dyn Extractor>, MetaError> {
        match self.0
            .iter()
            .filter(|x| x.name() == name)
            .collect::<Vec<_>>()
            .first() {
                Some(e) => {
                    Ok(e.to_owned())
                },
                None => {
                    Err(format!("could not find extractor {}", name).into())
                }
            }
    }

    pub fn extract(&self) -> Result<Meta, MetaError> {
        let mut s:Meta = Meta::new();
        
        for e in self.0.iter() {
            match e.extract(&mut s) {
                Ok(_) => (),
                // todo - convert to debug error
                Err(ee) => {
                    // todo - improve for better handling
                    // detach the Meta creation from the extractor processing.
                    eprintln!("extractor error ({}): {:#?}", e.name(), ee);
                    return Err(ee);
                }
            }
        }
        Ok(s)
    }
}


mod general;

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

#[cfg(feature = "hash")]
mod hash;


/// Search and collect extractors by file extension.
pub fn get_extractors(file_path: &str) -> Result<Extractors, MetaError> {
    let extension: String = match Path::new(file_path).extension() {
        Some(x) => { x.to_str().unwrap().trim().to_ascii_lowercase() },
        _ => { panic!("missing file extension") }
    };

    let mut extractors: Vec<Box<dyn Extractor>> = vec![
        // universal extractors
        Box::new(General::file(file_path)),
    ];

    match extension.as_str() {
        "mkv" => {
            #[cfg(feature = "matroska")]
            {
                use crate::matroska::Matroska;
                extractors.push(Box::new(Matroska::file(file_path)));
            }

            #[cfg(feature = "hash")]
            {
                use crate::hash::MetaHash;
                extractors.push(Box::new(MetaHash::file(file_path)))
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

            #[cfg(feature = "hash")]
            {
                use crate::hash::MetaHash;
                extractors.push(Box::new(MetaHash::file(file_path)))
            }
        }
        "mp4" | "mov" | "m4v" => {
            #[cfg(feature = "mp4")]
            {
                use crate::mp4::MP4;
                extractors.push(Box::new(MP4::file(file_path)))
            }

            #[cfg(feature = "hash")]
            {
                use crate::hash::MetaHash;
                extractors.push(Box::new(MetaHash::file(file_path)))
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

            #[cfg(feature = "hash")]
            {
                use crate::hash::MetaHash;
                extractors.push(Box::new(MetaHash::file(file_path)))
            }
        },
        "heic" | "heif" => {
            // not supported in heif
            // extractors.push(Box::new(CommonImageMeta::file(file_path)));

            // EXIF could be in almost any format.
            // Optimistically, we'll try to extract for each format.
            #[cfg(feature = "exif")]
            {
                use crate::exif::ExifExtractor;
                extractors.push(Box::new(ExifExtractor::file(file_path)))
            }

            #[cfg(feature = "heic")]
            {
                use crate::heic::Heic;
                extractors.push(Box::new(Heic::file(file_path)))
            }
        },
        "png" => {
            extractors.push(Box::new(CommonImageMeta::file(file_path)));

            #[cfg(feature = "hash")]
            {
                use crate::hash::MetaHash;
                extractors.push(Box::new(MetaHash::file(file_path).use_pixel()))
            }
        },
        "jpeg" | "jpg" | "tif" | "tiff" | "cr2" | "jfif" => {
            extractors.push(Box::new(CommonImageMeta::file(file_path)));

            // EXIF could be in almost any format.
            // Optimistically, we'll try to extract for each format.
            #[cfg(feature = "exif")]
            {
                use crate::exif::ExifExtractor;
                extractors.push(Box::new(ExifExtractor::file(file_path)))
            }

            #[cfg(feature = "hash")]
            {
                use crate::hash::MetaHash;
                extractors.push(Box::new(MetaHash::file(file_path).use_pixel()))
            }
        },
        _ => {
            println!("reverting to universal extractors {:#?}", extension);

            #[cfg(feature = "hash")]
            {
                use crate::hash::MetaHash;
                extractors.push(Box::new(MetaHash::file(file_path)))
            }
        }
    }
    Ok(Extractors(extractors))
}


#[cfg(test)]
mod test {
    use std::fs;
    use crate::{get_extractors, Extractors, Meta};

    #[test]
    fn test_get_extractors_hash() {
        let extractors: Extractors = get_extractors("../testdata/original/bad_jpeg.jpeg").unwrap();
        let hash = extractors.find("HASH").unwrap();
        let mut meta = Meta::new();
        hash.extract(&mut meta).unwrap();
        println!("{:#?}", meta);
    }

    #[test]
    fn test_get_extractors_hash_3gp() {
        let extractors: Extractors = get_extractors("../testdata/hash_compare/3gp_one.3gp").unwrap();
        let hash = extractors.find("HASH").unwrap();
        let mut meta = Meta::new();
        hash.extract(&mut meta).unwrap();
        println!("{:#?}", meta);
    }


    #[test]
    fn test_get_extractors_hash_png() {
        let extractors: Extractors = get_extractors("../testdata/png/00bd9af4-cf55-4186-99e4-c3526aa86279.png").unwrap();
        let hash = extractors.find("HASH").unwrap();
        let mut meta = Meta::new();
        hash.extract(&mut meta).unwrap();
        println!("{:#?}", meta);
    }

    #[test]
    fn test_get_extractors_hash_mp4() {
        let extractors: Extractors = get_extractors("../testdata/why/ad8c8a26-9290-4484-8219-642ba1775b9e.mp4").unwrap();
        let hash = extractors.find("HASH").unwrap();
        let mut meta = Meta::new();
        hash.extract(&mut meta).unwrap();
        println!("{:#?}", meta);
    }

    #[test]
    fn test_get_extractors_exif() {
        let extractors: Extractors = get_extractors("../testdata/original/bad_jpeg.jpeg").unwrap();
        let exif = extractors.find("EXIF").unwrap();
        let mut meta = Meta::new();
        exif.extract(&mut meta).unwrap();
        println!("{:#?}", meta);
    }

    #[test]
    fn test_get_extractors_all() {
        let extractors: Extractors = get_extractors("../testdata/original/bad_jpeg.jpeg").unwrap();
        println!("{:#?}", extractors.extract().unwrap());
    }

    // #[test]
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
                        let meta = extractors.extract();
                        println!("\n\n");
                        println!("File: {:#?}", file_path);
                        println!("{:#?}", meta);
                        println!("\n\n");
                    }                    
                }
            }
        } 
    }

    // #[test]
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
                        let meta = extractors.extract();
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

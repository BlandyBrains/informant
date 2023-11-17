// Common meta objects
mod meta;
mod error;
mod types;

#[cfg(feature="heic")]
mod heic;
#[cfg(feature="heic")]
pub use crate::heic::extract_meta as extract_heic_meta;

#[cfg(feature = "ape")]
mod ape;
#[cfg(feature = "ape")]
pub use crate::audio::extract_ape_meta;

#[cfg(feature = "id3")]
mod id3;
#[cfg(feature = "id3")]
pub use crate::audio::extract_id3_meta;

#[cfg(feature = "exif")]
mod exif;
#[cfg(feature = "exif")]
pub use crate::exif::extract_meta as extract_exif_meta;

#[cfg(feature = "matroska")]
mod matroska;
#[cfg(feature = "matroska")]
use crate::matroska::extract_meta as extract_mkv_meta;

#[cfg(feature = "mp4")]
mod mp4;
#[cfg(feature = "mp4")]
use crate::mp4::extract_meta as extract_mp4_meta;

#[cfg(feature = "hash")]
mod hash;

mod audio;
use crate::audio::AudioMetaError;

mod image;
use crate::image::{ImageMetaError, extract_basic_meta};

mod video;
use crate::video::VideoMetaError;

pub use crate::meta::{MetaClass, MetaAttribute, MetaValue, MetaFormat, MetaSource, MetaType};

pub use error::MetaError;
pub use types::{audio_types, image_types, video_types};
use crate::image::ExifMetaError;
use log::warn;


/*
**   Single entrypoint for exporting meta from all data types.
*/
pub fn extract_meta(extension: &str, location: &str) -> Result<Vec<MetaAttribute>, MetaError> {
    let mut meta: Vec<MetaAttribute> = Vec::new();

    // Video Format
    #[cfg(feature = "matroska")]
    if extension.eq_ignore_ascii_case("mkv") {
        match extract_mkv_meta(location) {
            Ok(meta) => {
                return Ok(meta);
            },
            Err(e) => {
                warn!("video meta error [matroska] {} {:#?}", location, e);
                return Ok(meta);
            }
        }
    }

    // Audio & Video Formats
    #[cfg(feature = "mp4")]
    let mp4_fmts :Vec<&str> = vec!["mp4", "mov", "m4v", "m4a"]; // m4a is MP4 audio
    if mp4_fmts.iter().any(|x| x.eq_ignore_ascii_case(extension)) {
        match extract_mp4_meta(location) {
            Ok(meta) => {
                return Ok(meta);
            },
            Err(e) => {
                warn!("video meta error [mp4] {} {:#?}", location, e);
                return Ok(meta);
            }
        }
    }

    if video_types().iter().any(|x| x.eq_ignore_ascii_case(extension)){
        warn!("no meta extractor available for this type {}", extension);
        return Ok(meta);
    }
    
    // Audio Format
    if audio_types().iter().any(|x| x.eq_ignore_ascii_case(extension)) {
        #[cfg(feature = "ape")]
        match extract_ape_meta(location, &mut meta) {
            Ok(_) => {},
            Err(e) => {
                warn!("audio meta error [ape] {} {:#?}", location, e);
            }
        }

        #[cfg(feature = "id3")]
        match extract_id3_meta(location, &mut meta) {
            Ok(_) => {},
            Err(e) => {
                warn!("audio meta error [id3] {} {:#?}", location, e);
            }
        }  
    }

    // Image Format
    if image_types().iter().any(|x| x.eq_ignore_ascii_case(extension)) {
        let supported_img_fmts :Vec<&str> = vec!["png", "jpeg", "jpg", "gif", "bmp", "ico", "tiff", "pnm", "dds", "tga"];
        if supported_img_fmts.iter().any(|x| x.eq_ignore_ascii_case(extension)) {
            match extract_basic_meta(location, &mut meta){
                Ok(_) => {},
                Err(e) => {
                    warn!("image meta error [basic] {} {:#?}", location, e);
                }
            }
        }
    
        #[cfg(feature = "heic")]
        if extension.eq_ignore_ascii_case("heic"){
            match extract_heic_meta(location, &mut meta) {
                Ok(_) => {},
                Err(e) => {
                    warn!("image meta error [heic] {} {:#?}", location, e);
                }
            }
        }
        
        // EXIF could be in almost any format.
        // Optimistically, we'll try to extract for each format.
        #[cfg(feature = "exif")]
        match extract_exif_meta(location, &mut meta) {
            Ok(_) => {},
            // ignore meta errors
            Err(ExifMetaError::ExifError(_)) => {},
            Err(ExifMetaError::InvalidFormat(_)) => {},
            Err(ExifMetaError::NotFound(_)) => {},
            Err(e) => {
                warn!("unaddressed EXIF meta error {} {:#?}", location, e);
                return Err(e.into());
            },
        }
    }

    return Ok(meta);
}



#[cfg(test)]
mod test {
    use log::{error, info};
    use walkdir::WalkDir;
    use crate::extract_meta;

    #[test]
    fn test_extract_image_files() {
        const TESTDATA_IMAGE: &str = "../testdata/image/";

        // walk directory
        for entry in WalkDir::new(TESTDATA_IMAGE).into_iter() {

            match entry {
                Ok(e) => {
                    if e.path().is_dir(){
                        continue;
                    }
                    let path = e.path();
                    let extension = path.extension().unwrap().to_str().unwrap();
        
                    println!("extension {:#?}", extension);
                    match extract_meta(extension, path.to_str().unwrap()) {
                        Ok(_m) => {
                            info!("successfully extracted meta for {:#?}", path);
                        },
                        Err(e) => {
                            error!("failed to extract meta from {:#?}, {:#?}", path, e);
                        }
                    }
                },
                Err(e) => {
                    error!("error walking directory {:#?}", e);
                }
            }   
        }
    }

    #[test]
    fn test_extract_video_files() {
        const TESTDATA_VIDEO: &str = "../testdata/broken_test.mkv";

        // walk directory
        for entry in WalkDir::new(TESTDATA_VIDEO).into_iter() {

            match entry {
                Ok(e) => {
                    if e.path().is_dir(){
                        continue;
                    }
                    let path = e.path();
                    let extension = path.extension().unwrap().to_str().unwrap();
        
                    println!("extension {:#?}", extension);
                    match extract_meta(extension, path.to_str().unwrap()) {
                        Ok(m) => {
                            println!("successfully extracted meta for {:#?}\n {:#?}", path, m);
                        },
                        Err(e) => {
                            println!("failed to extract meta from {:#?}, {:#?}", path, e);
                        }
                    }
                },
                Err(e) => {
                    println!("error walking directory {:#?}", e);
                }
            }   
        }
    }


    // #[test]
    // fn test_extract_meta_image() {
    //     const TEST_IMAGE: &str = "../testdata/jenna_test.jpg";

    //     match extract_meta(&get_format(TEST_IMAGE), TEST_IMAGE) {
    //         Ok(meta) => {
    //             println!("meta {:#?}", meta);
    //             assert!(meta.len() > 0);
    //         },
    //         Err(e) => {
    //             panic!("{:#?}", e);
    //         }
    //     }
    // }

    // #[test]
    // #[ignore = "Incomplete development, could not convert process to using a single buffer."]
    // fn test_image_operations() {
    //     let format = ImageFormat::from_path(TEST_IMAGE.clone().to_string()).unwrap();

    //     // sync 

    //     // basic image load
    //     let img = image::open(TEST_IMAGE.clone().to_string()).unwrap();
    //     println!("io open - result {:#?}", img.dimensions());
        
    //     // io -> buffers
    //     // let mut image_sync: std::fs::File = std::fs::File::open(TEST_IMAGE.clone().to_string()).unwrap();
    //     // let mut buf_sync: std::io::BufReader<std::fs::File> = std::io::BufReader::new(image_sync);
    
    //     // todo -- this doesn't work =( 
    //     // println!("{:#?}", buf_sync);
    //     // buf_sync.rewind();
        
    //     let result = image::load_from_memory_with_format(img.as_bytes(), format).unwrap();
    //     println!("from memory w/ format - result {:#?}", result.dimensions());

    //     // async 
    //     // let mut image_async: File = tokio::fs::File::open(TEST_IMAGE.clone().to_string()).await.unwrap();
        
    //     // basic image load

    //     // image.seek(SeekFrom::Start(0)).await.unwrap();

    //     // let mut buf = BufReader::new(image);

    //     // let result = ImageReader::new(Cursor::new(buf.buffer()))
    //     //     .with_guessed_format()
    //     //     .unwrap().decode().unwrap();

    //     // println!("result {:#?}", result);
    // }
}
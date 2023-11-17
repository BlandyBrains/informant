use std::result::Result;
use image::GenericImageView;
use thiserror::Error;
use crate::meta::{MetaAttribute, MetaSource, MetaValue, MetaType, MetaFormat};

#[cfg(feature = "exif")]
pub use crate::exif::{ExifMetaError, extract_meta as extract_exif_meta};

#[cfg(feature = "heic")]
pub use crate::heic::{HeicMetaError, extract_meta as extract_heic_meta};

#[cfg(feature = "hash")]
use crate::hash::extract_hash;


#[derive(Error, Debug)]
pub enum ImageMetaError {
    #[error("image error `{0}`")]
    ImageError(#[from] image::ImageError),

    #[error("exif error `{0}`")]
    ExifError(#[from] ExifMetaError),

    #[cfg(feature = "heic")]
    #[error("heic error `{0}`")]
    HeicError(#[from] HeicMetaError),
}

/// Extract image metadata from traditional image files. 
pub fn extract_basic_meta(location: &str, meta: &mut Vec<MetaAttribute>) -> Result<(), ImageMetaError> {
    // general image metadata
    return match image::open(location) {
        Ok(g) => {
            let (width, height) = g.dimensions();
        
            meta.push(MetaAttribute { 
                format: MetaFormat::Image,
                source: MetaSource::Basic,
                tag: "height".to_owned(), 
                value: MetaType::UInt64(MetaValue::from(u64::from(height)))
            });

            meta.push(MetaAttribute { 
                format: MetaFormat::Image, 
                source: MetaSource::Basic,
                tag: "width".to_owned(), 
                value: MetaType::UInt64(MetaValue::from(u64::from(width)))
            });                 

            #[cfg(feature = "hash")]
            extract_hash(g, meta);   

            Ok(())
        },
        Err(e) => { Err(ImageMetaError::ImageError(e)) }
    };
}


// #[test]
// #[ignore = "skipping hash"]
// fn test_hash_review() {
//     let mut config = HasherConfig::new();
//     config = HasherConfig::hash_alg(config, HashAlg::Gradient);
//     config = HasherConfig::hash_size(config, 5, 5);
//     // let hasher = HasherConfig::hash_alg(config, HashAlg::Mean);

//     let hasher = config.to_hasher();

//     // let asset_location = "../testdata/hash_check/4b7ac0e6-b5a4-425b-8a90-4b933ab66375.heic";
//     // let asset_location = "../testdata/hash_check/4b76efab-19b5-400e-8325-0ef134ba4b31.heic";
//     let asset_location = "../testdata/hash_check/2a0e3c29-475f-4562-a81d-7373b2e41a8f.png";
    
//     // match ImageReader::open(asset_location.to_string()).unwrap().decode() {
//     match image::open(asset_location.to_string()){
//         Ok(g) => {
//             // let (width, height) = g.dimensions();
//             let image_hash = hasher.hash_image(&g);
//             println!("hash {:#?}", image_hash);
//         },
//         Err(e) => {
//             panic!("error {:#?}", e);
//         }
//     }  
// }
use std::io;
use thiserror::Error;
use libheif_rs::HeifContext;
use crate::meta::{MetaAttribute, MetaFormat, MetaSource, MetaType, MetaValue};

#[derive(Error, Debug)]
pub enum HeicMetaError {
    #[error("io error `{0}`")]
    IOError(#[from] io::Error),

    #[error("heic error `{0}`")]
    HeifError(#[from] libheif_rs::HeifError)
}

/// Extract image metadata from HEIF image type 
pub fn extract_meta(location: &str, meta: &mut Vec<MetaAttribute>) -> Result<(), HeicMetaError> {
    return match HeifContext::read_from_file(location){
        Ok(ctx) => {
            let handle = match ctx.primary_image_handle() {
                Ok(h) => h,
                Err(e) => {
                    return Err(HeicMetaError::HeifError(e));
                }
            };

            meta.push(MetaAttribute { 
                format: MetaFormat::Image,
                source: MetaSource::Heic, 
                tag: "height".to_owned(), 
                value: MetaType::UInt64(MetaValue::from(u64::from(handle.height())))
            });

            meta.push(MetaAttribute { 
                format: MetaFormat::Image,
                source: MetaSource::Heic, 
                tag: "width".to_owned(), 
                value: MetaType::UInt64(MetaValue::from(u64::from(handle.width())))
            });      

            Ok(())
        }, 
        Err(e) => {
            Err(HeicMetaError::HeifError(e))
        }
    }
}


#[cfg(test)]
mod test {
    use crate::heic::{HeicMetaError, extract_meta};
    use crate::meta::MetaAttribute;

    const TEST_IMAGE: &str = "../testdata/raegan_thumb.heic"; 
    
    #[test]
    fn test_parse_empty() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let result: Result<(), HeicMetaError> = extract_meta("", &mut meta);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_parse() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let result: Result<(), HeicMetaError> = extract_meta(TEST_IMAGE, &mut meta);
        match result {
            Ok(_) => {
                // todo confirm we can serde
                // println!("{:#?}", meta);
                let j = match serde_json::to_string(&meta){
                    Ok(x) => x,
                    Err(e) => {
                        panic!("{}", e);
                    }
                };

                for x in meta { 
                    if x.tag == "Model" {
                        println!("WTF: {:#?}", x);
                    }
                }

                // Print, write to a file, or send to an HTTP server.
                println!("{:#?}", j);
            },
            Err(e) => {
                println!("test error {:#?}", e);
                panic!("{:#?}", e);
            }
        }
    }
}
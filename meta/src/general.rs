use std::{fs, path::Path};

use crate::{Extractor, FromFile, Meta, MetaAttribute, MetaClass, MetaError, MetaSource, MetaType, MetaValue};


pub struct General { path: String }

impl General {
    fn get_class(&self, extension: &str) -> MetaClass {
        match extension {
            "mp4" | "mov" | "m4v" | "mkv" => {
                return MetaClass::Video;
            },
            "amr" | "m4a" | "mp3" | "wav" | "flac" | "wma" | "m4r" => {
                return MetaClass::Audio;
            },
            "heic" | "heif" | "jpeg" | "jpg" | "png" | "raf" | "tif" | "tiff" | "cr2" | "jfif" => {
                return MetaClass::Image;
            },
            "doc" | "docx" | "pdf" => {
                return MetaClass::Document;
            }
            _ => {
                return MetaClass::Unknown;
            }
        }
    }
}

impl FromFile for General {
    fn file(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl Extractor for General {
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {

        // File Size
        let file_size = match fs::metadata(&self.path) {
            Ok(x) => x.len(),
            Err(e) => {
                return Err(MetaError::from(format!("error fetching metadata {:#?}", e)));
            }
        };
        
        // Extension
        let path: &Path = Path::new(&self.path);
        let extension: &str = path.extension().and_then(|ext| ext.to_str()).unwrap();

        // MetaType
        let meta_class: MetaClass = self.get_class(extension);

        meta.add(MetaAttribute { 
            source: MetaSource::Basic, 
            tag: "size".to_owned(), 
            value: MetaType::UInt64(MetaValue::from(u64::from(file_size)))
        });

        meta.add(MetaAttribute { 
            source: MetaSource::Basic, 
            tag: "extension".to_owned(), 
            value: MetaType::String(MetaValue::from(extension.to_owned()))
        });  

        meta.add(MetaAttribute { 
            source: MetaSource::Basic, 
            tag: "class".to_owned(), 
            value: MetaType::String(MetaValue::from(String::from(meta_class.to_owned())))
        });   
        
        Ok(())
    }
}

// todo - test ape extraction
#[cfg(test)]
mod test {
    use crate::{FromFile, Extractor, MetaError, Meta};
    use super::General;

    const TEST_FILE: &str = "../testdata/Audio/test.mp3"; 

    #[test]
    fn test_parse() {
        let mut meta: Meta = Meta::new();
        let extractor: General = General::file(TEST_FILE);

        let result: Result<(), MetaError> = extractor.extract(&mut meta);
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
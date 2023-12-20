use std::result::Result;
use image::{GenericImageView, DynamicImage};
use crate::{meta::{MetaAttribute, MetaSource, MetaValue, MetaType, MetaFormat}, Detail, Extractor};

pub struct CommonImageMeta {
    file_path: String
}
impl Detail for CommonImageMeta {
    fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_string() }
    }
}
impl Extractor for CommonImageMeta {
    fn extract(&self, meta: &mut Vec<MetaAttribute>) -> Result<(), crate::MetaError> {
        let dyn_img: DynamicImage = image::open(self.file_path.to_string())?;

        let (width, height) = dyn_img.dimensions();
    
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

        Ok(())
    }
}

#[cfg(test)]
mod test {
    // use crate::meta::MetaAttribute;

    use crate::{MetaAttribute, MetaError, Detail, Extractor};

    use super::CommonImageMeta;

    const TEST_IMAGE: &str = "../testdata/Image/test.jpg"; 

    #[test]
    fn test_parse() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let extractor: CommonImageMeta = CommonImageMeta::new(TEST_IMAGE);
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
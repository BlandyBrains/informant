use image::{GenericImageView, DynamicImage};
use crate::{meta::{MetaAttribute, MetaSource, MetaValue, MetaType}, FromFile, Extractor, Meta};

pub struct CommonImageMeta {
    path: String
}
impl FromFile for CommonImageMeta {
    fn file(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}
impl Extractor for CommonImageMeta {
    fn name(&self) -> String {
        return "IMAGE".to_string();
    }
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {
        let dyn_img: DynamicImage = image::open(self.path.to_string())?;

        let (width, height) = dyn_img.dimensions();
    
        meta.add(MetaAttribute { 
            source: MetaSource::Basic,
            tag: "height".to_owned(), 
            value: MetaType::UInt64(MetaValue::from(u64::from(height)))
        });

        meta.add(MetaAttribute { 
            source: MetaSource::Basic,
            tag: "width".to_owned(), 
            value: MetaType::UInt64(MetaValue::from(u64::from(width)))
        });                 

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{MetaError, FromFile, Extractor, Meta};
    type TestError = Box<dyn std::error::Error + 'static>;

    use super::CommonImageMeta;

    const TEST_IMAGE: &str = "../testdata/Image/test.jpg"; 

    fn get_file_meta(file: &str) -> Result<Meta, TestError> {
        let mut meta: Meta = Meta::new();
        let extractor: CommonImageMeta = CommonImageMeta::file(file);
        extractor.extract(&mut meta).unwrap();
        Ok(meta)
    }

    #[test]
    fn test_parse() {
        let mut meta: Meta = Meta::new();
        let extractor: CommonImageMeta = CommonImageMeta::file(TEST_IMAGE);
        let result: Result<(), MetaError> = extractor.extract(&mut meta);
        match result {
            Ok(_) => {
                // todo confirm we can serde
                // println!("{:#?}", meta);
                let j: String = match serde_json::to_string(&meta){
                    Ok(x) => x,
                    Err(e) => {
                        panic!("{}", e);
                    }
                };

                match meta.find("Model").first() {
                    Some(x) => {
                        println!("WTF: {:#?}", &x);
                    },
                    None => {}
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

    #[test]
    fn test_heif_file() {
        let meta: Meta = get_file_meta("../testdata/original/fails_common_image_extractor.heic").unwrap();

    }
}
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

    use super::CommonImageMeta;

    const TEST_IMAGE: &str = "../testdata/Image/test.jpg"; 

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
}
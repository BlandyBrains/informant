use libheif_rs::{HeifContext, ImageHandle};
use crate::{meta::{MetaAttribute, MetaSource, MetaType, MetaValue}, FromFile, Extractor, Meta};


pub struct Heic{ path: String }

impl Heic {}

impl FromFile for Heic {
    fn file(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl Extractor for Heic {
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {
        let ctx: HeifContext = HeifContext::read_from_file(&self.path)?;

        let handle: ImageHandle<'_> = ctx.primary_image_handle()?;

        meta.add(MetaAttribute { 
            source: MetaSource::Heic, 
            tag: "height".to_owned(), 
            value: MetaType::UInt64(MetaValue::from(u64::from(handle.height())))
        });

        meta.add(MetaAttribute { 
            source: MetaSource::Heic, 
            tag: "width".to_owned(), 
            value: MetaType::UInt64(MetaValue::from(u64::from(handle.width())))
        });      

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::{MetaError, FromFile, Extractor, Meta};

    use super::Heic;

    const TEST_IMAGE: &str = "../testdata/Image/test.heic"; 

    #[test]
    fn test_parse() {
        let mut meta: Meta = Meta::new();
        let extractor: Heic = Heic::file(TEST_IMAGE);
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

                meta
                    .find("Model")
                    .first()
                    .map(|x| println!("Model: {:#?}", x));
    
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
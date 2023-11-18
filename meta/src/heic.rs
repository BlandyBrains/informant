use libheif_rs::{HeifContext, ImageHandle};
use crate::{meta::{MetaAttribute, MetaFormat, MetaSource, MetaType, MetaValue}, Detail, Extractor};


pub struct Heic{ file_path: String }

impl Heic {}

impl Detail for Heic {
    fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_string() }
    }
}

impl Extractor for Heic {
    fn extract(&self, meta: &mut Vec<MetaAttribute>) -> Result<(), crate::MetaError> {
        let ctx: HeifContext = HeifContext::read_from_file(&self.file_path)?;

        let handle: ImageHandle<'_> = ctx.primary_image_handle()?;

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
    }
}


#[cfg(test)]
mod test {
    // use crate::meta::MetaAttribute;

    // const TEST_IMAGE: &str = "../testdata/raegan_thumb.heic"; 
    
    // #[test]
    // fn test_parse_empty() {
    //     let mut meta: Vec<MetaAttribute> = Vec::new();
    //     let result: Result<(), HeicMetaError> = extract_meta("", &mut meta);
    //     assert_eq!(true, result.is_err());
    // }

    // #[test]
    // fn test_parse() {
    //     let mut meta: Vec<MetaAttribute> = Vec::new();
    //     let result: Result<(), HeicMetaError> = extract_meta(TEST_IMAGE, &mut meta);
    //     match result {
    //         Ok(_) => {
    //             // todo confirm we can serde
    //             // println!("{:#?}", meta);
    //             let j = match serde_json::to_string(&meta){
    //                 Ok(x) => x,
    //                 Err(e) => {
    //                     panic!("{}", e);
    //                 }
    //             };

    //             for x in meta { 
    //                 if x.tag == "Model" {
    //                     println!("WTF: {:#?}", x);
    //                 }
    //             }

    //             // Print, write to a file, or send to an HTTP server.
    //             println!("{:#?}", j);
    //         },
    //         Err(e) => {
    //             println!("test error {:#?}", e);
    //             panic!("{:#?}", e);
    //         }
    //     }
    // }
}
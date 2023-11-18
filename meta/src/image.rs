use std::result::Result;
use image::{GenericImageView, DynamicImage};
use crate::{meta::{MetaAttribute, MetaSource, MetaValue, MetaType, MetaFormat}, Detail, Extractor};

#[cfg(feature = "hash")]
use crate::hash::extract_hash;

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

        #[cfg(feature = "hash")]
        extract_hash(dyn_img, meta);   
        Ok(())
    }
}
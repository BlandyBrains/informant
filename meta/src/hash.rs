use crate::meta::{MetaAttribute, MetaFormat, MetaSource, MetaType, MetaValue};
use image::DynamicImage;
use img_hash::{HasherConfig, HashAlg};


pub fn extract_hash(img: DynamicImage, meta: &mut Vec<MetaAttribute>){
    let mut config:HasherConfig = HasherConfig::new();
    config = HasherConfig::hash_alg(config, HashAlg::Gradient);
    config = HasherConfig::hash_size(config, 32, 32);
    let hasher = config.to_hasher();

    let image_hash = hasher.hash_image(&img);

    meta.push(MetaAttribute{ 
        format: MetaFormat::Image,
        source: MetaSource::Hash, 
        tag: "image_hash_32x32".to_owned(), 
        value: MetaType::String(MetaValue::from(image_hash.to_base64()))
    });
}

#[cfg(test)]
mod test {
    use img_hash::{HasherConfig, HashAlg};

    #[test]
    fn test_hash_review() {
        let mut config = HasherConfig::new();
        config = HasherConfig::hash_alg(config, HashAlg::Gradient);
        config = HasherConfig::hash_size(config, 5, 5);
        // let hasher = HasherConfig::hash_alg(config, HashAlg::Mean);

        let hasher = config.to_hasher();
        let asset_location = "../testdata/jenna_test.jpg";
        
        // match ImageReader::open(asset_location.to_string()).unwrap().decode() {
        match image::open(asset_location.to_string()){
            Ok(g) => {
                let image_hash = hasher.hash_image(&g);
                println!("hash {:#?}", image_hash);
            },
            Err(e) => {
                panic!("error {:#?}", e);
            }
        }  
    }
}    
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
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};
    use std::hash::Hash;

    use img_hash::{HasherConfig, HashAlg};

    // #[test]
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

    struct MyFile{
        content: Vec<u8>
    }

    impl Hash for MyFile {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.content.hash(state);
        }
    }

    #[test]
    fn test_file_comparison() {
        let mut s = DefaultHasher::new();

        let file_one: Vec<u8> = std::fs::read("../testdata/Video/0b63387d-5b88-4222-aa8c-2fdc9c18c067.mov").expect("failed to open file");
        let file_copy: Vec<u8> = std::fs::read("../testdata/Video/0b63387d-5b88-4222-aa8c-2fdc9c18c067 copy.mov").expect("failed to open file");

        let file_left: MyFile = MyFile { content: file_one };
        let file_right: MyFile = MyFile { content: file_copy };

        file_left.hash(&mut s);
        let left_hash: u64 = s.finish();

        file_right.hash(&mut s);
        let right_hash: u64 = s.finish();

        assert_eq!(left_hash, right_hash);
        println!("left_hash {}", left_hash);
        println!("right_hash {}", right_hash);
    }

}    
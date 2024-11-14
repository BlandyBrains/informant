use std::{fs::File, io::Read};

use image::DynamicImage;

use crate::{
    meta::{MetaAttribute, MetaSource, MetaType, MetaValue}, 
    FromFile, Extractor, Meta};


pub struct MetaHash {
    path: String,
    pixel_only: bool
}

impl MetaHash { 
    pub fn use_pixel(mut self) -> Self {
        self.pixel_only = true;
        return self;
    }
}

impl FromFile for MetaHash {
    fn file(path: &str) -> Self {
        Self { path: path.to_string(), pixel_only: false }
    }
}


impl Extractor for MetaHash {
    fn name(&self) -> String {
        return "HASH".to_string();
    }
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {
        let mut content: Vec<u8> = Vec::new();

        if self.pixel_only {
            println!("Using pixel only algorithm for calculating hash...");

            let img: DynamicImage = image::open(&self.path)?;
            content.append(&mut img.as_bytes().to_vec().to_owned());

        } else {
            let mut file: File = File::open(&self.path)?;
            file.read_to_end(&mut content)?;
        }
        
        let hash: blake3::Hash = blake3::hash(&content);

        meta.add(MetaAttribute{
            source: MetaSource::Hash,
            tag: "hash".to_string(),
            value: MetaType::String(MetaValue::from(hash.to_string())),
        });

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{FromFile, Extractor, Meta};
    use super::MetaHash;

    fn get_hash(file: &str, use_pixel: bool) -> String {
        let mut mh: MetaHash = MetaHash::file(file);
        if use_pixel {
            mh = mh.use_pixel();
        }

        let mut meta: Meta = Meta::new();
        
        let _ = mh.extract(&mut meta).unwrap();

        match meta.find("hash").first() {
            Some(h) => {
                String::from(h.value.to_owned())
            },
            None => {
                panic!("could not get hash");
            }
        }
    }

    
    #[test]
    fn test_blake3_hash_pixel_only() {
        let expected: &str = "a57d1e0b40f011673b40784fdf1e7e27a2f0d1e574b8ad0801b6281323b67b65";
        let hash: String = get_hash("../testdata/hash_compare/original.jpg", true);
        assert_eq!(expected, hash);
    }

    #[test]
    fn test_blake3_hash_3gp() {
        let hash_one: String = get_hash("../testdata/hash_compare/3gp_one.3gp", false);
        let hash_two: String = get_hash("../testdata/hash_compare/3gp_two.3gp", false);

        assert_ne!(hash_one, hash_two)
    }
}    
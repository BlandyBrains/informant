use std::{fs::File, io::Read};

use crate::{
    meta::{MetaAttribute, MetaFormat, MetaSource, MetaType, MetaValue}, 
    FromFile, Extractor, Meta};


pub struct MetaHash{path: String }
impl MetaHash { }


impl FromFile for MetaHash {
    fn file(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl Extractor for MetaHash {
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {

        let mut file = File::open(&self.path)?;

        let mut content: Vec<u8> = Vec::new();

        file.read_to_end(&mut content)?;

        let hash: blake3::Hash = blake3::hash(&content);

        meta.add(MetaAttribute{
            source: MetaSource::Hash,
            tag: "hash".to_string(),
            format: MetaFormat::Generic,
            value: MetaType::String(MetaValue::from(hash.to_string())),
        });

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{FromFile, Extractor, Meta};
    use super::MetaHash;


    #[test]
    fn test_blake3_hash() {
        let expected: &str = "bf6bb1d6e958bcad72985514fda0b62cf18583ad5f50a91c0fe1e9e35d6d81f0";

        let mh: MetaHash = MetaHash::file("../testdata/Audio/Razorblade.mp3");

        let mut meta: Meta = Meta::new();
        
        let _ = mh.extract(&mut meta).unwrap();

        match meta.find("hash").first() {
            Some(h) => {
                let actual: String = String::from(h.value.to_owned());
                assert_eq!(expected, actual);
            },
            None => {
                panic!("could not get hash");
            }
        }
    }
}    
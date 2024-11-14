use libheif_rs::{ColorSpace, HeifContext, ImageHandle, LibHeif, Plane, Planes, RgbChroma};
use crate::{meta::{MetaAttribute, MetaSource, MetaType, MetaValue}, FromFile, Extractor, Meta};


pub struct Heic{ path: String }

impl Heic {}

impl FromFile for Heic {
    fn file(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl Extractor for Heic {
    fn name(&self) -> String {
        return "HEIF".to_string();
    }
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {
        let ctx: HeifContext = HeifContext::read_from_file(&self.path)?;

        let handle: ImageHandle = ctx.primary_image_handle()?;

        meta.add(MetaAttribute { 
            source: MetaSource::Basic, 
            tag: "height".to_owned(), 
            value: MetaType::UInt64(MetaValue::from(u64::from(handle.height())))
        });

        meta.add(MetaAttribute { 
            source: MetaSource::Basic, 
            tag: "width".to_owned(), 
            value: MetaType::UInt64(MetaValue::from(u64::from(handle.width())))
        });      

        let lib_heif = LibHeif::new();

        let image: libheif_rs::Image = lib_heif.decode(
            &handle,
            ColorSpace::Rgb(RgbChroma::Rgb), 
            None
        )?;
        let planes: Planes<&[u8]> = image.planes();
        let interleaved_plane: Plane<&[u8]> = planes.interleaved.unwrap();
        let hash: blake3::Hash = blake3::hash(interleaved_plane.data);

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
    type TestError = Box<dyn std::error::Error + 'static>;

    use crate::{FromFile, Extractor, Meta};
    use super::Heic;

    const TEST_IMAGE: &str = "../testdata/original/0CF43F0D-1606-4C1C-BA22-DD1C59EF5F60.heic";  // 00FBFB79-6EBC-47F3-825E-32A8DB5A70A2.heic
    const TEST_IMAGE_CMP: &str = "../testdata/original/00FBFB79-6EBC-47F3-825E-32A8DB5A70A2.heic"; // 00E0B929-31E0-405F-8563-D4B1676D7F13.heic

    fn get_file_meta(file: &str) -> Result<Meta, TestError> {
        let mut meta: Meta = Meta::new();
        let extractor: Heic = Heic::file(file);
        extractor.extract(&mut meta).unwrap();
        Ok(meta)
    }

    #[test]
    fn test_parse() {
        let meta: Meta = get_file_meta(TEST_IMAGE).unwrap();

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

    }

    #[test]
    fn test_compare_hash_consistent() {
        let meta_one: Meta = get_file_meta(TEST_IMAGE).unwrap();
        let meta_two: Meta = get_file_meta(TEST_IMAGE).unwrap();
        
        let hash_one: String = meta_one
            .find("hash")
            .first()
            .unwrap()
            .value.clone().into();

        let hash_two: String = meta_two
            .find("hash")
            .first()
            .unwrap()
            .value.clone().into();

        println!("{:#?}", hash_one);
        println!("{:#?}", hash_two);
        assert_eq!(hash_one, hash_two);
    }

    #[test]
    fn test_compare_hash() {
        let meta_one: Meta = get_file_meta(TEST_IMAGE).unwrap();
        let meta_two: Meta = get_file_meta(TEST_IMAGE_CMP).unwrap();
        
        let hash_one: String = meta_one
            .find("hash")
            .first()
            .unwrap()
            .value.clone().into();

        let hash_two: String = meta_two
            .find("hash")
            .first()
            .unwrap()
            .value.clone().into();

        println!("{:#?}", hash_one);
        println!("{:#?}", hash_two);
        assert_ne!(hash_one, hash_two);
    }


}
use ape::{Item, Tag};
use crate::{meta::{MetaAttribute, MetaValue}, MetaSource, MetaType, FromFile, Extractor, Meta};


pub struct Ape { path: String }
impl Ape {
    fn convert_str(item: Option<&Item>) -> Option<String> {
        match item {
            None => None,
            Some(x) => Some(format!("{:?}", x.value))
        }
    }

    fn get_meta(&self, tag: ape::Tag, meta: &mut Meta) {
        match Self::convert_str(tag.item("title")) {
            Some(title) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "title".to_string(),
                    value: MetaType::String(MetaValue::from(title)),
                });
            }, 
            _ => ()
        }

        match Self::convert_str(tag.item("subtitle")) {
            Some(subtitle) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "subtitle".to_string(),
                    value: MetaType::String(MetaValue::from(subtitle)),
                });
            }, 
            _ => ()
        }

        match Self::convert_str(tag.item("artist")) {
            Some(artist) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "artist".to_string(),
                    value: MetaType::String(MetaValue::from(artist)),
                });
            }, 
            _ => ()
        }

        match Self::convert_str(tag.item("album")) {
            Some(album) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "album".to_string(),
                    value: MetaType::String(MetaValue::from(album)),
                });
            }, 
            _ => ()
        }        

        match Self::convert_str(tag.item("debut_album")) {
            Some(debut_album) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "debut_album".to_string(),
                    value: MetaType::String(MetaValue::from(debut_album)),
                });
            }, 
            _ => ()
        }   
        
        match Self::convert_str(tag.item("publisher")) {
            Some(publisher) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "publisher".to_string(),
                    value: MetaType::String(MetaValue::from(publisher)),
                });
            }, 
            _ => ()
        }        

        match Self::convert_str(tag.item("conductor")) {
            Some(conductor) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "conductor".to_string(),
                    value: MetaType::String(MetaValue::from(conductor)),
                });
            }, 
            _ => ()
        }   
        
        match Self::convert_str(tag.item("composer")) {
            Some(composer) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "composer".to_string(),
                    value: MetaType::String(MetaValue::from(composer)),
                });
            }, 
            _ => ()
        }   

        match Self::convert_str(tag.item("comment")) {
            Some(comment) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "comment".to_string(),
                    value: MetaType::String(MetaValue::from(comment)),
                });
            }, 
            _ => ()
        }         
        
        match Self::convert_str(tag.item("copyright")) {
            Some(copyright) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "copyright".to_string(),
                    value: MetaType::String(MetaValue::from(copyright)),
                });
            }, 
            _ => ()
        }  

        match Self::convert_str(tag.item("publication_right")) {
            Some(publication_right) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "publication_right".to_string(),
                    value: MetaType::String(MetaValue::from(publication_right)),
                });
            }, 
            _ => ()
        }  
        
        match Self::convert_str(tag.item("isbn")) {
            Some(isbn) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "isbn".to_string(),
                    value: MetaType::String(MetaValue::from(isbn)),
                });
            }, 
            _ => ()
        }       

        match Self::convert_str(tag.item("catalog")) {
            Some(catalog) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "catalog".to_string(),
                    value: MetaType::String(MetaValue::from(catalog)),
                });
            }, 
            _ => ()
        }      

        match Self::convert_str(tag.item("label_code")) {
            Some(label_code) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "label_code".to_string(),
                    value: MetaType::String(MetaValue::from(label_code)),
                });
            }, 
            _ => ()
        }      

        match Self::convert_str(tag.item("related")) {
            Some(related) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "related".to_string(),
                    value: MetaType::String(MetaValue::from(related)),
                });
            }, 
            _ => ()
        }           

        match Self::convert_str(tag.item("isrc")) {
            Some(isrc) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "isrc".to_string(),
                    value: MetaType::String(MetaValue::from(isrc)),
                });
            }, 
            _ => ()
        }        
        
        match Self::convert_str(tag.item("language")) {
            Some(language) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "language".to_string(),
                    value: MetaType::String(MetaValue::from(language)),
                });
            }, 
            _ => ()
        }            

        match Self::convert_str(tag.item("bibliography")) {
            Some(bibliography) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "bibliography".to_string(),
                    value: MetaType::String(MetaValue::from(bibliography)),
                });
            }, 
            _ => ()
        }           

        match Self::convert_str(tag.item("year")) {
            Some(year) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "year".to_string(),
                    value: MetaType::String(MetaValue::from(year)),
                });
            }, 
            _ => ()
        }      

        match Self::convert_str(tag.item("record_date")) {
            Some(record_date) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "record_date".to_string(),
                    value: MetaType::String(MetaValue::from(record_date)),
                });
            }, 
            _ => ()
        }

        match Self::convert_str(tag.item("genre")) {
            Some(genre) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "genre".to_string(),
                    value: MetaType::String(MetaValue::from(genre)),
                });
            }, 
            _ => ()
        }        

        match Self::convert_str(tag.item("track")) {
            Some(track) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "track".to_string(),
                    value: MetaType::String(MetaValue::from(track)),
                });
            }, 
            _ => ()
        }        

        match Self::convert_str(tag.item("media")) {
            Some(media) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "media".to_string(),
                    value: MetaType::String(MetaValue::from(media)),
                });
            }, 
            _ => ()
        }            

        match Self::convert_str(tag.item("upc")) {
            Some(upc) => {
                meta.add(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "upc".to_string(),
                    value: MetaType::String(MetaValue::from(upc)),
                });
            }, 
            _ => ()
        }                
    }
}

impl FromFile for Ape {
    fn file(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl Extractor for Ape {
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {
        let tag: Tag = ape::read_from_path(self.path.to_string())?;
        self.get_meta(tag, meta);
        Ok(())
    }
}

// todo - test ape extraction
#[cfg(test)]
mod test {
    use crate::{FromFile, Extractor, MetaError, Meta};
    use super::Ape;

    const TEST_FILE: &str = "../testdata/Audio/test.mp3"; 

    // #[test]
    fn test_parse() {
        let mut meta: Meta = Meta::new();
        let extractor: Ape = Ape::file(TEST_FILE);

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
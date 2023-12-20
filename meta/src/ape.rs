use ape::{Item, Tag};
use crate::{meta::{MetaAttribute, MetaValue, MetaFormat}, MetaSource, MetaType, Detail, Extractor};


pub struct Ape { file_path: String }
impl Ape {
    fn convert_str(item: Option<&Item>) -> Option<String> {
        match item {
            None => None,
            Some(x) => Some(format!("{:?}", x.value))
        }
    }

    fn get_meta(&self, tag: ape::Tag, values: &mut Vec<MetaAttribute>) {
        match Self::convert_str(tag.item("title")) {
            Some(title) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "title".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(title)),
                });
            }, 
            _ => ()
        }

        match Self::convert_str(tag.item("subtitle")) {
            Some(subtitle) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "subtitle".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(subtitle)),
                });
            }, 
            _ => ()
        }

        match Self::convert_str(tag.item("artist")) {
            Some(artist) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "artist".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(artist)),
                });
            }, 
            _ => ()
        }

        match Self::convert_str(tag.item("album")) {
            Some(album) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "album".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(album)),
                });
            }, 
            _ => ()
        }        

        match Self::convert_str(tag.item("debut_album")) {
            Some(debut_album) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "debut_album".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(debut_album)),
                });
            }, 
            _ => ()
        }   
        
        match Self::convert_str(tag.item("publisher")) {
            Some(publisher) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "publisher".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(publisher)),
                });
            }, 
            _ => ()
        }        

        match Self::convert_str(tag.item("conductor")) {
            Some(conductor) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "conductor".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(conductor)),
                });
            }, 
            _ => ()
        }   
        
        match Self::convert_str(tag.item("composer")) {
            Some(composer) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "composer".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(composer)),
                });
            }, 
            _ => ()
        }   

        match Self::convert_str(tag.item("comment")) {
            Some(comment) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "comment".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(comment)),
                });
            }, 
            _ => ()
        }         
        
        match Self::convert_str(tag.item("copyright")) {
            Some(copyright) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "copyright".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(copyright)),
                });
            }, 
            _ => ()
        }  

        match Self::convert_str(tag.item("publication_right")) {
            Some(publication_right) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "publication_right".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(publication_right)),
                });
            }, 
            _ => ()
        }  
        
        match Self::convert_str(tag.item("isbn")) {
            Some(isbn) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "isbn".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(isbn)),
                });
            }, 
            _ => ()
        }       

        match Self::convert_str(tag.item("catalog")) {
            Some(catalog) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "catalog".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(catalog)),
                });
            }, 
            _ => ()
        }      

        match Self::convert_str(tag.item("label_code")) {
            Some(label_code) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "label_code".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(label_code)),
                });
            }, 
            _ => ()
        }      

        match Self::convert_str(tag.item("related")) {
            Some(related) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "related".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(related)),
                });
            }, 
            _ => ()
        }           

        match Self::convert_str(tag.item("isrc")) {
            Some(isrc) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "isrc".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(isrc)),
                });
            }, 
            _ => ()
        }        
        
        match Self::convert_str(tag.item("language")) {
            Some(language) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "language".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(language)),
                });
            }, 
            _ => ()
        }            

        match Self::convert_str(tag.item("bibliography")) {
            Some(bibliography) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "bibliography".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(bibliography)),
                });
            }, 
            _ => ()
        }           

        match Self::convert_str(tag.item("year")) {
            Some(year) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "year".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(year)),
                });
            }, 
            _ => ()
        }      

        match Self::convert_str(tag.item("record_date")) {
            Some(record_date) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "record_date".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(record_date)),
                });
            }, 
            _ => ()
        }

        match Self::convert_str(tag.item("genre")) {
            Some(genre) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "genre".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(genre)),
                });
            }, 
            _ => ()
        }        

        match Self::convert_str(tag.item("track")) {
            Some(track) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "track".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(track)),
                });
            }, 
            _ => ()
        }        

        match Self::convert_str(tag.item("media")) {
            Some(media) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "media".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(media)),
                });
            }, 
            _ => ()
        }            

        match Self::convert_str(tag.item("upc")) {
            Some(upc) => {
                values.push(MetaAttribute{
                    source: MetaSource::Ape,
                    tag: "upc".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(upc)),
                });
            }, 
            _ => ()
        }                
    }
}

impl Detail for Ape {
    fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_string() }
    }
}

impl Extractor for Ape {
    fn extract(&self, meta: &mut Vec<MetaAttribute>) -> Result<(), crate::MetaError> {
        let tag: Tag = ape::read_from_path(self.file_path.to_string())?;
        self.get_meta(tag, meta);
        Ok(())
    }
}

// todo - test ape extraction
#[cfg(test)]
mod test {
    use crate::{MetaAttribute, Detail, Extractor, MetaError};
    use super::Ape;

    const TEST_FILE: &str = "../testdata/Audio/test.mp3"; 

    #[test]
    fn test_parse() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let extractor: Ape = Ape::new(TEST_FILE);

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
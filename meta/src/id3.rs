use id3::{TagLike, Timestamp, v1v2, Tag};
use crate::{
    MetaAttribute, MetaSource, 
    meta::{MetaFormat, MetaValue}, 
    MetaType, Detail, Extractor};


pub struct ID3 { file_path: String }
impl ID3 {
    fn convert_str(value: Option<&str>) -> Option<String> {
        match value {
            None => None,
            Some(x) => Some(String::from(x))
        }
    }
    
    fn convert_vec(value: Option<Vec<&str>>) -> Option<Vec<String>> {
        match value{
            None => None,
            Some(x) => {
                Some(x.iter().map(|x| String::from(*x)).collect())
            }
        }
    }

    fn convert_timestamp(value: Option<Timestamp>) -> Option<String> {
        match value {
            None => None,
            Some(x) => {
                Some(x.to_string())
            }
        }
    }

    fn get_meta(&self, tag: id3::Tag, values: &mut Vec<MetaAttribute>) {
        match Self::convert_str(tag.artist()) {
            Some(artist) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "artist".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(artist)),
                });
            },
            None => ()
        }

        match Self::convert_vec(tag.artists()) {
            Some(artists) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "artists".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(artists.join(","))),
                });
            },
            None => ()
        }

        match Self::convert_str(tag.title()) {
            Some(title) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "title".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(title)),
                });
            },
            None => ()
        }

        match Self::convert_str(tag.album()) {
            Some(album) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "album".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(album)),
                });
            },
            None => ()
        }

        match Self::convert_str(tag.album_artist()) {
            Some(album_artist) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "album_artist".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(album_artist)),
                });
            },
            None => ()
        }  

        match tag.year() {
            Some(year) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "year".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::Int64(MetaValue::from(year as i64)),
                });
            },
            None => ()
        }         
        
        match Self::convert_timestamp(tag.date_recorded()){
            Some(date) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "date_recorded".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(date)),
                });
            },
            None => ()
        } 

        match Self::convert_timestamp(tag.date_released()){
            Some(date) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "date_released".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(date)),
                });
            },
            None => ()
        }           
        
        match Self::convert_timestamp(tag.original_date_released()){
            Some(date) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "original_date_released".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(date)),
                });
            },
            None => ()
        }         
        
        match tag.duration(){
            Some(duration) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "duration".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::UInt64(MetaValue::from(duration as u64)),
                });
            },
            None => ()
        } 

        // TODO - convert ID3 genre to human readable value
        // https://en.wikipedia.org/wiki/List_of_ID3v1_Genres
        match Self::convert_str(tag.genre()){
            Some(genre) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "genre".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(genre)),
                });
            },
            None => ()
        } 
        
        match Self::convert_vec(tag.genres()){
            Some(genres) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "genres".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::String(MetaValue::from(genres.join(","))),
                });
            },
            None => ()
        }    
        
        match tag.disc(){
            Some(disc) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "disc".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::UInt64(MetaValue::from(disc as u64)),
                });
            },
            None => ()
        }         

        match tag.total_discs(){
            Some(discs) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "total_discs".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::UInt64(MetaValue::from(discs as u64)),
                });
            },
            None => ()
        }   

        match tag.track(){
            Some(track) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "track".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::UInt64(MetaValue::from(track as u64)),
                });
            },
            None => ()
        }  

        match tag.total_tracks(){
            Some(tracks) => {
                values.push(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "total_tracks".to_string(),
                    format: MetaFormat::Audio,
                    value: MetaType::UInt64(MetaValue::from(tracks as u64)),
                });
            },
            None => ()
        } 
    }
}

impl Detail for ID3 {
    fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_string() }
    }
}

impl Extractor for ID3 {
    fn extract(&self, meta: &mut Vec<MetaAttribute>) -> Result<(), crate::MetaError> {
        let tag: Tag =  v1v2::read_from_path(self.file_path.to_owned())?;
        self.get_meta(tag, meta);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{meta::MetaAttribute, Detail, Extractor};
    use super::ID3;


    #[test]
    fn test_no_tag() {
        let mut meta: Vec<MetaAttribute> = Vec::new();

        let extractor: ID3 = ID3::new("../testdata/Audio/test.mp3");
        match extractor.extract(&mut meta) {
            Ok(_) => {
                let j = match serde_json::to_string(&meta){
                    Ok(x) => x,
                    Err(e) => {
                        panic!("{}", e);
                    }
                };
                println!("{:#?}", j);
            },
            Err(e) => {
                if e.to_string().starts_with("NoTag:") {
                    println!("test error {:#?}", e);
                    return;
                }
                panic!("{:#?}", e);
            }
        }
    }

    #[test]
    fn test_valid_tag() {
        let mut meta: Vec<MetaAttribute> = Vec::new();

        let extractor: ID3 = ID3::new("../testdata/Audio/Razorblade.mp3");
        match extractor.extract(&mut meta) {
            Ok(_) => {
                let j = match serde_json::to_string(&meta){
                    Ok(x) => x,
                    Err(e) => {
                        panic!("{}", e);
                    }
                };
                println!("{:#?}", j);
            },
            Err(e) => {
                if e.to_string().starts_with("NoTag:") {
                    println!("test error {:#?}", e);
                    return;
                }
                panic!("{:#?}", e);
            }
        }
    }
}
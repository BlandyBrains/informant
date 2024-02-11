use id3::{TagLike, Timestamp, v1v2, Tag};
use crate::{
    MetaAttribute, MetaSource, 
    meta::MetaValue, 
    MetaType, FromFile, Extractor, Meta};


pub struct ID3 { path: String }
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

    fn get_meta(&self, tag: id3::Tag, meta: &mut Meta) {
        match Self::convert_str(tag.artist()) {
            Some(artist) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "artist".to_string(),
                    value: MetaType::String(MetaValue::from(artist)),
                });
            },
            None => ()
        }

        match Self::convert_vec(tag.artists()) {
            Some(artists) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "artists".to_string(),
                    value: MetaType::String(MetaValue::from(artists.join(","))),
                });
            },
            None => ()
        }

        match Self::convert_str(tag.title()) {
            Some(title) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "title".to_string(),
                    value: MetaType::String(MetaValue::from(title)),
                });
            },
            None => ()
        }

        match Self::convert_str(tag.album()) {
            Some(album) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "album".to_string(),
                    value: MetaType::String(MetaValue::from(album)),
                });
            },
            None => ()
        }

        match Self::convert_str(tag.album_artist()) {
            Some(album_artist) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "album_artist".to_string(),
                    value: MetaType::String(MetaValue::from(album_artist)),
                });
            },
            None => ()
        }  

        match tag.year() {
            Some(year) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "year".to_string(),
                    value: MetaType::Int64(MetaValue::from(year as i64)),
                });
            },
            None => ()
        }         
        
        match Self::convert_timestamp(tag.date_recorded()){
            Some(date) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "date_recorded".to_string(),
                    value: MetaType::String(MetaValue::from(date)),
                });
            },
            None => ()
        } 

        match Self::convert_timestamp(tag.date_released()){
            Some(date) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "date_released".to_string(),
                    value: MetaType::String(MetaValue::from(date)),
                });
            },
            None => ()
        }           
        
        match Self::convert_timestamp(tag.original_date_released()){
            Some(date) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "original_date_released".to_string(),
                    value: MetaType::String(MetaValue::from(date)),
                });
            },
            None => ()
        }         
        
        match tag.duration(){
            Some(duration) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "duration".to_string(),
                    value: MetaType::UInt64(MetaValue::from(duration as u64)),
                });
            },
            None => ()
        } 

        // TODO - convert ID3 genre to human readable value
        // https://en.wikipedia.org/wiki/List_of_ID3v1_Genres
        match Self::convert_str(tag.genre()){
            Some(genre) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "genre".to_string(),
                    value: MetaType::String(MetaValue::from(genre)),
                });
            },
            None => ()
        } 
        
        match Self::convert_vec(tag.genres()){
            Some(genres) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "genres".to_string(),                    
                    value: MetaType::String(MetaValue::from(genres.join(","))),
                });
            },
            None => ()
        }    
        
        match tag.disc(){
            Some(disc) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "disc".to_string(),
                    value: MetaType::UInt64(MetaValue::from(disc as u64)),
                });
            },
            None => ()
        }         

        match tag.total_discs(){
            Some(discs) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "total_discs".to_string(),
                    value: MetaType::UInt64(MetaValue::from(discs as u64)),
                });
            },
            None => ()
        }   

        match tag.track(){
            Some(track) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "track".to_string(),
                    value: MetaType::UInt64(MetaValue::from(track as u64)),
                });
            },
            None => ()
        }  

        match tag.total_tracks(){
            Some(tracks) => {
                meta.add(MetaAttribute{
                    source: MetaSource::ID3,
                    tag: "total_tracks".to_string(),
                    value: MetaType::UInt64(MetaValue::from(tracks as u64)),
                });
            },
            None => ()
        } 
    }
}

impl FromFile for ID3 {
    fn file(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl Extractor for ID3 {
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {
        let tag: Tag =  v1v2::read_from_path(self.path.to_owned())?;
        self.get_meta(tag, meta);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{FromFile, Extractor, Meta};
    use super::ID3;


    #[test]
    fn test_no_tag() {
        let mut meta: Meta = Meta::new();

        let extractor: ID3 = ID3::file("../testdata/Audio/test.mp3");
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
        let mut meta: Meta = Meta::new();

        let extractor: ID3 = ID3::file("../testdata/Audio/Razorblade.mp3");
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
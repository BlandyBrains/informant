use id3::{TagLike, Timestamp, Error as Id3Error, v1v2};
use log::info;
use crate::{MetaAttribute, MetaSource, meta::{MetaFormat, MetaValue}, MetaType};



pub struct ID3 {}
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

    fn get_meta(tag: id3::Tag, values: &mut Vec<MetaAttribute>) {
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


pub fn extract_meta(location: &str, meta: &mut Vec<MetaAttribute>) -> Result<(), Id3Error>{
    match v1v2::read_from_path(location){
        Ok(t) => {
            info!("ID3 version: {}", t.version());
            Ok(ID3::get_meta(t, meta))
        },
        Err(e) => {
            Err(e)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::id3::{Id3Error, extract_meta};
    use crate::meta::MetaAttribute;

    const TEST_FILE: &str = "../testdata/intake/audio/Joe Jackson.mp3"; 
    
    #[test]
    fn test_parse_empty() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let result: Result<(), Id3Error> = extract_meta("", &mut meta);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_parse() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let result: Result<(), Id3Error> = extract_meta(TEST_FILE, &mut meta);
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
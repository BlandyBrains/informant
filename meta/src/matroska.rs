use log::{warn, error};
use matroska::{Tracktype, Settings};
use std::result::Result;
use crate::{meta::{MetaSource, MetaAttribute, MetaType, MetaValue, MetaFormat}, VideoMetaError};


pub struct Matroska {}
impl Matroska {
    fn get_info(m: &matroska::Matroska, values: &mut Vec<MetaAttribute>){
        match &m.info.date_utc {
            Some(x) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::Matroska,
                    tag: "info.date_utc".to_string(),
                    value: MetaType::Int64(MetaValue::from(i64::from(x.clone()))),
                });
            },
            _ => ()
        }

        match m.info.duration {
            Some(x) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::Matroska,
                    tag: "info.duration".to_string(),
                    value: MetaType::UInt64(MetaValue::from(x.as_secs())),
                });
            },
            _ => ()
        }

        match m.info.title.clone() {
            Some(x) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::Matroska,
                    tag: "info.title".to_string(),
                    value: MetaType::String(MetaValue::from(x)),
                });
            }
            _ => ()
        }

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::Matroska,
            tag: "info.muxing_app".to_string(),
            value: MetaType::String(MetaValue::from(m.info.muxing_app.clone())),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::Matroska,
            tag: "info.writing_app".to_string(),
            value: MetaType::String(MetaValue::from(m.info.writing_app.clone())),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::Matroska,
            tag: "track_count".to_string(),
            value: MetaType::Int64(MetaValue::from(m.tracks.len() as i64)),
        });

    }

    fn get_audio(m: &matroska::Matroska, values: &mut Vec<MetaAttribute>) {
        let audio_track = m.tracks.iter()
        .find(|t| t.tracktype == Tracktype::Audio);

        if audio_track.is_none(){
            return;
        }

        match &audio_track.unwrap().settings {
            Settings::Audio(settings) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::Matroska,
                    tag: "audio.settings.channels".to_string(),
                    value: MetaType::UInt64(MetaValue::from(settings.channels)),
                });

                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::Matroska,
                    tag: "audio.settings.sample_rate".to_string(),
                    value: MetaType::Rational(MetaValue::from(settings.sample_rate)),
                });
            }
            _ => ()
        }
    }

    fn get_video(m: &matroska::Matroska, values: &mut Vec<MetaAttribute>){
        let video_track = m.tracks.iter()
        .find(|t| t.tracktype == Tracktype::Video);

        if video_track.is_none() {
            return;
        }

        match &video_track.unwrap().settings {
            Settings::Video(settings) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::Matroska,
                    tag: "video.settings.pixel_height".to_string(),
                    value: MetaType::UInt64(MetaValue::from(settings.pixel_height)),
                });
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::Matroska,
                    tag: "video.settings.pixel_width".to_string(),
                    value: MetaType::UInt64(MetaValue::from(settings.pixel_width)),
                });
            }
            _ => ()
        }
    }

    pub fn from_path(path: &str, values: &mut Vec<MetaAttribute>) -> Result<(), VideoMetaError> {
        match matroska::open(path) {
            Ok(m) => {
                Self::get_info(&m, values);
                Self::get_audio(&m, values);
                Self::get_video(&m, values);
            },
            Err(e) => {
                error!("matroska error {:#?}", e);
            }
        }
        return Ok(());
    }
}

pub fn extract_meta(location: &str) -> Result<Vec<MetaAttribute>, VideoMetaError> {
    let mut values: Vec<MetaAttribute> = Vec::new();

    match Matroska::from_path(location, &mut values){
        Ok(_) => (),
        Err(e) => {
            warn!("meta parse Matroska {:#?} {:#?}", location, e);
        }
    }
    
    return Ok(values);
}


#[cfg(test)]
mod test {
    use chrono::{NaiveDateTime, Datelike};
    use crate::VideoMetaError;
    use crate::matroska::extract_meta;
    use crate::meta::MetaAttribute;

    const TEST_IMAGE: &str = "../testdata/matroska_test.mkv"; 
    
    #[test]
    fn test_parse_empty() {
        let result: Result<Vec<MetaAttribute>, VideoMetaError> = extract_meta("");
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn test_parse() {
        let result: Result<Vec<MetaAttribute>, VideoMetaError> = extract_meta(TEST_IMAGE);
        match result {
            Ok(meta) => {
                // todo confirm we can serde
                println!("{:#?}", meta);

                for attr in &meta {
                    if attr.tag == "info.date_utc" {
                        println!("{:#?}", attr);
                        let ts: i64 = i64::from(attr.value.clone());
                        if ts > 0 {
                            let dt: NaiveDateTime = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();
                            assert_eq!(2019, dt.year());
                            assert_eq!(1, dt.month());
                        }
                        else {
                            panic!("SHOULD NOT BE HERE ");
                        }
                    }
                }
            },
            Err(e) => {
                println!("test error {:#?}", e);
            }
        }
    }
}
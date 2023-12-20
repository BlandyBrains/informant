use matroska::{Tracktype, Settings};
use std::result::Result;
use crate::{
    meta::{MetaSource, MetaAttribute, MetaType, MetaValue, MetaFormat}, 
    Extractor, MetaError, Detail};


pub struct Matroska {
    file_path: String
}
impl Matroska {
    fn get_info(&self, m: &matroska::Matroska, values: &mut Vec<MetaAttribute>){
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

    fn get_audio(&self, m: &matroska::Matroska, values: &mut Vec<MetaAttribute>) {
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

    fn get_video(&self, m: &matroska::Matroska, values: &mut Vec<MetaAttribute>){
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

    pub fn from_path(&self, path: &str, values: &mut Vec<MetaAttribute>) -> Result<(), Box<dyn std::error::Error + 'static>> {
        match matroska::open(path) {
            Ok(m) => {
                self.get_info(&m, values);
                self.get_audio(&m, values);
                self.get_video(&m, values);
            },
            Err(e) => {
                return Err(Box::new(e));
            }
        }
        return Ok(());
    }
}

impl Detail for Matroska {
    fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_owned() }
    }
}
impl Extractor for Matroska {
    fn extract(&self, meta: &mut Vec<MetaAttribute>) -> Result<(), MetaError> {
        Ok(self.from_path(&self.file_path, meta)?)
    }
}

#[cfg(test)]
mod test {
    use super::Matroska;
    use crate::{MetaAttribute, MetaError, Detail, Extractor};

    const TEST_VIDEO: &str = "../testdata/Video/test.mkv"; 

    #[test]
    fn test_parse() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let extractor: Matroska = Matroska::new(TEST_VIDEO);
        let result: Result<(), MetaError> = extractor.extract(&mut meta);
        match result {
            Ok(_) => {
                // todo confirm we can serde
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
            }
        }
    }
}
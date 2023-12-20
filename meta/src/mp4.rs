use mp4::{Mp4Track, MoovBox, Metadata, creation_time};
use std::{result::Result, io::BufReader, fs::File};
use crate::{meta::{MetaSource, MetaAttribute, MetaType, MetaValue, MetaFormat}, Extractor, MetaError, Detail};

pub struct MP4 { file_path: String}
impl MP4 {
    fn get_meta(&self, moov: &MoovBox, values: &mut Vec<MetaAttribute>){
        match &moov.udta {
            None => (),
            Some(x) => {
                match &x.meta {
                    None => (),
                    Some(y) => {
                        match &y.ilst {
                            None => (),
                            Some(z) => {
                                match z.title() {
                                    Some(title) => {
                                        values.push(MetaAttribute{
                                            format: MetaFormat::Video,
                                            source: MetaSource::MP4,
                                            tag: "title".to_string(),
                                            value: MetaType::String(MetaValue::from(title.as_ref().to_owned())),
                                        });
                                    },
                                    _ => ()
                                }

                                match z.year() {
                                    Some(year) => {
                                        values.push(MetaAttribute{
                                            format: MetaFormat::Video,
                                            source: MetaSource::MP4,
                                            tag: "year".to_string(),
                                            value: MetaType::UInt64(MetaValue::from(year as u64)),
                                        })
                                    },
                                    _ => ()
                                }

                                match z.summary() {
                                    Some(summary) => {
                                        values.push(MetaAttribute{
                                            format: MetaFormat::Video,
                                            source: MetaSource::MP4,
                                            tag: "summary".to_string(),
                                            value: MetaType::String(MetaValue::from(summary.as_ref().to_owned())),
                                        })
                                    },
                                    _ => ()
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_track(&self, track_no: &u32, track: &Mp4Track, values: &mut Vec<MetaAttribute>){
        match track.audio_profile(){
            Ok(x) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::MP4,
                    tag: format!("track_{}.audio_profile", track_no),
                    value: MetaType::String(MetaValue::from(x.to_string())),
                });
            },
            Err(_) => (),
        };

        match track.media_type(){
            Ok(x) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::MP4,
                    tag: format!("track_{}.media_type", track_no),
                    value: MetaType::String(MetaValue::from(x.to_string())),
                });
            },
            Err(_) => (),
        };

        match track.track_type() {
            Ok(x) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::MP4,
                    tag: format!("track_{}.track_type", track_no),
                    value: MetaType::String(MetaValue::from(x.to_string())),
                });
            },
            Err(_) => (),
        };

        match track.video_profile(){
            Ok(x) => {
                values.push(MetaAttribute{
                    format: MetaFormat::Video,
                    source: MetaSource::MP4,
                    tag: format!("track_{}.video_profile", track_no),
                    value: MetaType::String(MetaValue::from(x.to_string())),
                });
            }
            Err(_) => (),
        };

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.bitrate", track_no),
            value: MetaType::UInt64(MetaValue::from(track.bitrate() as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.default_sample_duration", track_no),
            value: MetaType::UInt64(MetaValue::from(track.default_sample_duration as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.duration", track_no),
            value: MetaType::UInt64(MetaValue::from(track.duration().as_secs() as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.frame_rate", track_no),
            value: MetaType::Rational(MetaValue::from(track.frame_rate())),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.height", track_no),
            value: MetaType::UInt64(MetaValue::from(track.height() as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.width", track_no),
            value: MetaType::UInt64(MetaValue::from(track.width() as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.language", track_no),
            value: MetaType::String(MetaValue::from(track.language().to_string())),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.sample_count", track_no),
            value: MetaType::UInt64(MetaValue::from(track.sample_count() as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.timescale", track_no),
            value: MetaType::UInt64(MetaValue::from(track.timescale() as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: format!("track_{}.track_id", track_no),
            value: MetaType::UInt64(MetaValue::from(track.track_id() as u64)),
        });
    }

    fn get_mvex(&self, moov: &MoovBox, values: &mut Vec<MetaAttribute>){
        return match &moov.mvex{
            None => (),
            Some(x) => {
                match &x.mehd {
                    None => (),
                    Some(y) => {

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "mvex.version".to_string(),
                            value: MetaType::UInt64(MetaValue::from(y.version as u64)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "fragment_duration".to_string(),
                            value: MetaType::UInt64(MetaValue::from(y.fragment_duration)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "flags".to_string(),
                            value: MetaType::UInt64(MetaValue::from(y.flags as u64)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "trex.version".to_string(),
                            value: MetaType::UInt64(MetaValue::from(x.trex.version as u64)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "trex.flags".to_string(),
                            value: MetaType::UInt64(MetaValue::from(x.trex.flags as u64)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "trex.track_id".to_string(),
                            value: MetaType::UInt64(MetaValue::from(x.trex.track_id as u64)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "trex.default_sample_description_index".to_string(),
                            value: MetaType::UInt64(MetaValue::from(x.trex.default_sample_description_index as u64)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "trex.default_sample_duration".to_string(),
                            value: MetaType::UInt64(MetaValue::from(x.trex.default_sample_duration as u64)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "trex.default_sample_size".to_string(),
                            value: MetaType::UInt64(MetaValue::from(x.trex.default_sample_size as u64)),
                        });

                        values.push(MetaAttribute{
                            format: MetaFormat::Video,
                            source: MetaSource::MP4,
                            tag: "trex.default_sample_flags".to_string(),
                            value: MetaType::UInt64(MetaValue::from(x.trex.default_sample_flags as u64)),
                        });
                    }
                }
            }
        };
    }

    fn get_mvhd(&self, moov: &MoovBox, values: &mut Vec<MetaAttribute>){
        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "mvhd.version".to_string(),
            value: MetaType::UInt64(MetaValue::from(moov.mvhd.version as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "mvhd.flags".to_string(),
            value: MetaType::UInt64(MetaValue::from(moov.mvhd.flags as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "creation_time".to_string(),
            value: MetaType::UInt64(MetaValue::from(creation_time(moov.mvhd.creation_time) as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "modification_time".to_string(),
            value: MetaType::UInt64(MetaValue::from(creation_time(moov.mvhd.modification_time) as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "timescale".to_string(),
            value: MetaType::UInt64(MetaValue::from(moov.mvhd.timescale as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "mvhd.duration".to_string(),
            value: MetaType::UInt64(MetaValue::from(moov.mvhd.duration as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "mvhd.rate".to_string(),
            value: MetaType::UInt64(MetaValue::from(moov.mvhd.rate.value() as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "mvhd.volume".to_string(),
            value: MetaType::UInt64(MetaValue::from(moov.mvhd.volume.value() as u64)),
        });

        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "mvhd.next_track_id".to_string(),
            value: MetaType::UInt64(MetaValue::from(moov.mvhd.next_track_id as u64)),
        });
    }

    fn from_reader(&self, reader: &mut BufReader<File>, size: u64, values: &mut Vec<MetaAttribute>) -> Result<(), MetaError> {
        let mp4 = mp4::Mp4Reader::read_header(reader, size)?;

        // compatible brands
        let comp_brands: String = mp4.ftyp.compatible_brands.iter().map(|x| String::from_utf8(x.value.to_vec()).unwrap_or("".to_string())).collect::<Vec<String>>().join(",");
        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "compatible_brands".to_string(),
            value: MetaType::String(MetaValue::from(comp_brands)),
        });

        // major brand
        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "major_brand".to_string(),
            value: MetaType::String(MetaValue::from(mp4.ftyp.major_brand.to_string())),
        });

        // minor version
        values.push(MetaAttribute{
            format: MetaFormat::Video,
            source: MetaSource::MP4,
            tag: "minor_version".to_string(),
            value: MetaType::String(MetaValue::from(mp4.ftyp.minor_version.to_string())),
        });

        // HashMap<u32, Mp4Track>
        mp4.tracks().into_iter().for_each(|x| self.get_track(x.0, x.1, values));

        self.get_meta(&mp4.moov, values);
        self.get_mvex(&mp4.moov, values);
        self.get_mvhd(&mp4.moov, values);

        Ok(())
    }
}

impl Detail for MP4 {
    fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_string() }
    }
}
impl Extractor for MP4 {
    fn extract(&self, meta: &mut Vec<MetaAttribute>) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let file: File = std::fs::File::open(self.file_path.to_string())?;
        let size: u64 = file.metadata()?.len();
        let mut buf_reader: BufReader<File> = std::io::BufReader::new(file);

        self.from_reader(&mut buf_reader, size, meta)
    }
}

#[cfg(test)]
mod test {
    use super::MP4;
    use crate::{MetaAttribute, MetaError, Detail, Extractor};

    const TEST_VIDEO_MP4: &str = "../testdata/Video/test.mp4"; 
    const TEST_VIDEO_MOV: &str = "../testdata/Video/test.mov"; 

    #[test]
    fn test_parse_mp4() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let extractor: MP4 = MP4::new(TEST_VIDEO_MP4);
        let result: Result<(), MetaError> = extractor.extract(&mut meta);
        match result {
            Ok(_) => {
                // todo confirm we can serde
                let j: String = match serde_json::to_string(&meta){
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

    #[test]
    fn test_parse_mov() {
        let mut meta: Vec<MetaAttribute> = Vec::new();
        let extractor: MP4 = MP4::new(TEST_VIDEO_MOV);
        let result: Result<(), MetaError> = extractor.extract(&mut meta);
        match result {
            Ok(_) => {
                // todo confirm we can serde
                let j: String  = match serde_json::to_string(&meta){
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
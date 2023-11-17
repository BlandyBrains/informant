use log::warn;
use mp4::{Mp4Track, MoovBox, Metadata, creation_time};
use std::{result::Result, io::BufReader, fs::File};
use crate::{meta::{MetaSource, MetaAttribute, MetaType, MetaValue, MetaFormat}, VideoMetaError};

pub struct MP4 {}
impl MP4 {
    fn get_meta(moov: &MoovBox, values: &mut Vec<MetaAttribute>){
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

    fn get_track(track_no: &u32, track: &Mp4Track, values: &mut Vec<MetaAttribute>){
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

    fn get_mvex(moov: &MoovBox, values: &mut Vec<MetaAttribute>){
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

    fn get_mvhd(moov: &MoovBox, values: &mut Vec<MetaAttribute>){
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

    fn from_reader(reader: &mut BufReader<File>, size: u64, values: &mut Vec<MetaAttribute>) -> Result<Vec<MetaAttribute>, VideoMetaError> {
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
        mp4.tracks().into_iter().for_each(|x| Self::get_track(x.0, x.1, values));

        Self::get_meta(&mp4.moov, values);
        Self::get_mvex(&mp4.moov, values);
        Self::get_mvhd(&mp4.moov, values);

        Ok(values.to_vec())
    }
}

pub fn extract_meta(location: &str) -> Result<Vec<MetaAttribute>, VideoMetaError> {
    let file: File = std::fs::File::open(location.to_string())?;
    let size: u64 = file.metadata()?.len();
    let mut buf_reader: BufReader<File> = std::io::BufReader::new(file);

    let mut values: Vec<MetaAttribute> = Vec::new();
    match MP4::from_reader(&mut buf_reader, size, &mut values){
        Ok(_) => (),
        Err(e) => {
            warn!("meta parse MP4 {:#?} {:#?}", location, e);
        }
    }
    
    return Ok(values);
}

#[cfg(test)]
mod test {
    use crate::mp4::{VideoMetaError, extract_meta};
    use crate::meta::MetaAttribute;

    const TEST_ASSET: &str = "../testdata/intake/audio/Shame.m4a"; 
    
    #[test]
    fn test_parse_empty() {
        let result: Result<Vec<MetaAttribute>, VideoMetaError> = extract_meta("");
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_parse() {
        let result: Result<Vec<MetaAttribute>, VideoMetaError> = extract_meta(TEST_ASSET);
        match result {
            Ok(meta) => {
                println!("{:#?}", meta);
                // todo confirm we can serde
                // for attr in &meta {
                //     if attr.tag == "creation_time" {
                //         println!("{:#?}", attr);
                //         let ts: i64 = i64::from(attr.value.clone());
                //         if ts > 0 {
                //             let dt: NaiveDateTime = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();
                //             assert_eq!(2021, dt.year());
                //             assert_eq!(5, dt.month());
                //         }
                //         else {
                //             panic!("SHOULD NOT BE HERE ");
                //         }
                //     }
                // }
            },
            Err(e) => {
                println!("test error {:#?}", e);
            }
        }
    }
}
use std::{io::BufReader, fs::File};
use std::result::Result;
use exif::{Value, Tag, In, Exif, Reader};

use crate::{FromFile, Extractor as CoreExtractor, Meta};
use crate::meta::{MetaAttribute, MetaSource, MetaType, MetaValue, MetaFormat};


/// Common EXIF Extractor Func
type Extractor<T> = dyn Fn(&Exif, Tag) -> Option<T>;

fn extract_str(exif: &Exif, tag: Tag) -> Option<MetaValue<String>> {
    match get_value_str(exif.get_field(tag, In::PRIMARY)) {
        Some(x) => Some(MetaValue::from(x)),
        None => None
    }
}

fn extract_u64(exif: &Exif, tag: Tag) -> Option<MetaValue<u64>> {
    match get_value_u64(exif.get_field(tag, In::PRIMARY)) {
        Some(x) => Some(MetaValue::from(x)),
        None => None
    }
}

fn extract_f64(exif: &Exif, tag: Tag) -> Option<MetaValue<f64>> {
    match get_value_f64(exif.get_field(tag, In::PRIMARY)) {
        Some(x) => Some(MetaValue::from(x)),
        None => None
    }
}

fn extract<T>(exif: &Exif, tag: Tag, extractor: &Extractor<MetaValue<T>>, meta: &mut Meta)
where T:Clone, MetaType: From<MetaValue<T>> {
    match extractor(exif, tag){
        Some(m) => {
            meta.add(MetaAttribute { 
                format: MetaFormat::Image,
                source: MetaSource::Exif,
                tag: tag.to_string(),
                value: MetaType::from(m),  
            });
        },
        None => ()
    }
}

fn get_value_str(field: Option<&exif::Field>) -> Option<String> {
    match field {
        Some(f) => {
            match f.value {
                Value::Ascii(ref vec) if !vec.is_empty() => {
                    Some(f.display_value().to_string())
                },
                Value::Double(ref vec) if !vec.is_empty() => {
                    Some(f.display_value().to_string())
                },
                Value::Float(ref vec) if !vec.is_empty() => {
                    Some(f.display_value().to_string())
                },
                Value::Rational(ref vec) if !vec.is_empty() => {
                    Some(f.display_value().to_string())
                },
                Value::Byte(ref vec) if !vec.is_empty() => {
                    Some(f.display_value().to_string())
                },
                Value::Short(ref vec) if !vec.is_empty() => {
                    Some(f.display_value().to_string())
                }, 
                Value::Long(ref vec) if !vec.is_empty() => {
                    Some(f.display_value().to_string())
                },               
                Value::Unknown(ref __, ref ___, ref ____) => {
                    // warn!("EXIF: unable to extract string field, value {:#?} {:#?} {:#?}", &v1, &v2, &v3);
                    None
                },         
                Value::SShort(ref vec) if !vec.is_empty()=> {
                    Some(f.display_value().to_string())
                },         
                Value::SLong(ref vec) if !vec.is_empty()=> {
                    Some(f.display_value().to_string())
                },            
                Value::SByte(ref vec) if !vec.is_empty() => {
                    Some(f.display_value().to_string())
                },            
                Value::SRational(ref vec) if !vec.is_empty()=> {
                    Some(f.display_value().to_string())
                },
                Value::Undefined(ref _vec, ref _x) => {
                    Some(f.display_value().to_string())
                },                                                                        
                _ => {
                    Some(f.display_value().to_string())
                },
            }
        },
        None => {
            None
        },
    }
}

fn get_value_u64(field: Option<&exif::Field>) -> Option<u64> {
    match field {
        Some(f) => {
            match f.value {
                Value::Double(ref vec) if !vec.is_empty() => {
                    Some(f64::from(vec.first().unwrap().to_owned()) as u64)
                },
                Value::Float(ref vec) if !vec.is_empty() => {
                    Some(f64::from(vec.first().unwrap().to_owned()) as u64)
                },
                Value::Rational(ref vec) if !vec.is_empty() => {
                    Some(f64::from(vec.first().unwrap().to_owned()) as u64)
                },
                Value::Byte(ref vec) if !vec.is_empty() => {
                    Some(vec.first().unwrap().to_owned() as u64)
                },
                Value::Short(ref vec) if !vec.is_empty() => {
                    Some(vec.first().unwrap().to_owned() as u64)
                }, 
                Value::Long(ref vec) if !vec.is_empty() => { 
                    Some(vec.first().unwrap().to_owned() as u64)
                },                                                                                      
                _ => {
                    // warn!("EXIF: unable to extract u64 field, value {:#?}", f.value);
                    None
                },
            }
        },
        None => {
            None
        },
    }
}

fn get_value_f64(field: Option<&exif::Field>) -> Option<f64> {
    match field {
        Some(f) => {
            match f.value {
                Value::Double(ref vec) if !vec.is_empty() => {
                    Some(vec.first().unwrap().to_owned() as f64)
                },
                Value::Float(ref vec) if !vec.is_empty() => {
                    Some(vec.first().unwrap().to_owned() as f64)
                },
                Value::Rational(ref vec) if !vec.is_empty() => {
                    Some(f64::from(vec.first().unwrap().to_owned()))
                },
                Value::Byte(ref vec) if !vec.is_empty() => {
                    None
                },
                Value::Short(ref vec) if !vec.is_empty() => {
                    Some(vec.first().unwrap().to_owned() as f64)
                }, 
                Value::Long(ref vec) if !vec.is_empty() => {
                    Some(vec.first().unwrap().to_owned() as f64)
                },                                                                            
                _ => {
                    // warn!("EXIF: unable to extract f64 field: value {:#?}, description: {:#?}", 
                    //     f.value, 
                    //     f.tag.description().unwrap());
                    None
                },
            }
        },
        None => {
            None
        },
    }
}


pub struct ExifExtractor { path: String }

impl FromFile for ExifExtractor {
    fn file(path: &str) -> Self {
        Self {path: path.to_string()}
    }
}

impl CoreExtractor for ExifExtractor {
    fn extract(&self, meta: &mut Meta) -> Result<(), crate::MetaError> {
        let file: File = std::fs::File::open(&self.path)?;
        let mut buf_reader: BufReader<&File> = std::io::BufReader::new(&file);
        let exif_reader: Reader = exif::Reader::new();
        let exif: Exif = exif_reader.read_from_container(&mut buf_reader)?;
    
        // Tiff Details
        extract(&exif, Tag::ImageWidth, &extract_str, meta);
        extract(&exif, Tag::ImageLength, &extract_str, meta);

        extract(&exif, Tag::BitsPerSample, &extract_str, meta);
        extract(&exif, Tag::Compression, &extract_str, meta);
        extract(&exif, Tag::PhotometricInterpretation, &extract_str, meta);
        extract(&exif, Tag::ImageDescription, &extract_str, meta);
        extract(&exif, Tag::Make, &extract_str, meta);
        extract(&exif, Tag::Model, &extract_str, meta);
        extract(&exif, Tag::StripOffsets, &extract_str, meta);
        extract(&exif, Tag::Orientation, &extract_str, meta);
        extract(&exif, Tag::SamplesPerPixel, &extract_str, meta);
        extract(&exif, Tag::RowsPerStrip, &extract_str, meta);
        extract(&exif, Tag::StripByteCounts, &extract_str, meta);
        extract(&exif, Tag::PlanarConfiguration, &extract_str, meta);
        extract(&exif, Tag::ResolutionUnit, &extract_str, meta);
        extract(&exif, Tag::TransferFunction, &extract_str, meta);
        extract(&exif, Tag::Software, &extract_str, meta);
        extract(&exif, Tag::Artist, &extract_str, meta);
        extract(&exif, Tag::TileOffsets, &extract_str, meta);                
        extract(&exif, Tag::TileByteCounts, &extract_str, meta);
        extract(&exif, Tag::JPEGInterchangeFormat, &extract_str, meta);
        extract(&exif, Tag::JPEGInterchangeFormatLength, &extract_str, meta);
        extract(&exif, Tag::YCbCrSubSampling, &extract_str, meta);            
        extract(&exif, Tag::YCbCrPositioning, &extract_str, meta);            
        extract(&exif, Tag::Copyright, &extract_str, meta);
        extract(&exif, Tag::XResolution, &extract_f64, meta);
        extract(&exif, Tag::YResolution, &extract_f64, meta);
        extract(&exif, Tag::WhitePoint, &extract_f64, meta);
        extract(&exif, Tag::PrimaryChromaticities, &extract_f64, meta);
        extract(&exif, Tag::YCbCrCoefficients, &extract_f64, meta);
        extract(&exif, Tag::ReferenceBlackWhite, &extract_f64, meta);
        extract(&exif, Tag::DateTime, &extract_str, meta);

        // Exif Details
        extract(&exif, Tag::ExifVersion, &extract_str, meta);   
        extract(&exif, Tag::ExposureProgram, &extract_str, meta);
        extract(&exif, Tag::SpectralSensitivity, &extract_str, meta);
        extract(&exif, Tag::PhotographicSensitivity, &extract_str, meta);
        extract(&exif, Tag::OECF, &extract_str, meta);
        extract(&exif, Tag::SensitivityType, &extract_str, meta);
        extract(&exif, Tag::StandardOutputSensitivity, &extract_str, meta);

        extract(&exif, Tag::RecommendedExposureIndex, &extract_str, meta);
        extract(&exif, Tag::ISOSpeed, &extract_str, meta);
        extract(&exif, Tag::ISOSpeedLatitudeyyy, &extract_str, meta);
        extract(&exif, Tag::ISOSpeedLatitudezzz, &extract_str, meta);
        extract(&exif, Tag::OffsetTime, &extract_str, meta);
        extract(&exif, Tag::OffsetTimeOriginal, &extract_str, meta);
        extract(&exif, Tag::OffsetTimeDigitized, &extract_str, meta);
        extract(&exif, Tag::ComponentsConfiguration, &extract_str, meta);
        extract(&exif, Tag::BrightnessValue, &extract_str, meta);
        extract(&exif, Tag::SubjectDistance, &extract_str, meta);
        extract(&exif, Tag::MeteringMode, &extract_str, meta);
        extract(&exif, Tag::LightSource, &extract_str, meta);
        extract(&exif, Tag::Flash, &extract_str, meta);
        extract(&exif, Tag::SubjectArea, &extract_str, meta);
        extract(&exif, Tag::MakerNote, &extract_str, meta);
        extract(&exif, Tag::UserComment, &extract_str, meta);
        extract(&exif, Tag::SubSecTime, &extract_str, meta);
        extract(&exif, Tag::SubSecTimeOriginal, &extract_str, meta);
        extract(&exif, Tag::SubSecTimeDigitized, &extract_str, meta);
        extract(&exif, Tag::Temperature, &extract_str, meta);
        extract(&exif, Tag::Humidity, &extract_str, meta);
        extract(&exif, Tag::Pressure, &extract_str, meta);
        extract(&exif, Tag::WaterDepth, &extract_str, meta);
        extract(&exif, Tag::Acceleration, &extract_str, meta);
        extract(&exif, Tag::CameraElevationAngle, &extract_str, meta);

        extract(&exif, Tag::ColorSpace, &extract_str, meta);
        extract(&exif, Tag::FlashpixVersion, &extract_str, meta);
        extract(&exif, Tag::RelatedSoundFile, &extract_str, meta);
        extract(&exif, Tag::SpatialFrequencyResponse, &extract_str, meta);
        extract(&exif, Tag::FocalPlaneResolutionUnit, &extract_str, meta);
        extract(&exif, Tag::SubjectLocation, &extract_str, meta);
        extract(&exif, Tag::SensingMethod, &extract_str, meta);
        extract(&exif, Tag::FileSource, &extract_str, meta);
        extract(&exif, Tag::SceneType, &extract_str, meta);
        extract(&exif, Tag::CFAPattern, &extract_str, meta);
        extract(&exif, Tag::CustomRendered, &extract_str, meta);
        extract(&exif, Tag::ExposureMode, &extract_str, meta);
        extract(&exif, Tag::WhiteBalance, &extract_str, meta);
        extract(&exif, Tag::DigitalZoomRatio, &extract_str, meta);
        extract(&exif, Tag::FocalLengthIn35mmFilm, &extract_str, meta);
        extract(&exif, Tag::SceneCaptureType, &extract_str, meta);
        extract(&exif, Tag::GainControl, &extract_str, meta);
        extract(&exif, Tag::Contrast, &extract_str, meta);
        extract(&exif, Tag::Saturation, &extract_str, meta);
        extract(&exif, Tag::Sharpness, &extract_str, meta);
        extract(&exif, Tag::DeviceSettingDescription, &extract_str, meta);
        extract(&exif, Tag::SubjectDistanceRange, &extract_str, meta);
        extract(&exif, Tag::ImageUniqueID, &extract_str, meta);
        extract(&exif, Tag::CameraOwnerName, &extract_str, meta);
        extract(&exif, Tag::BodySerialNumber, &extract_str, meta);
        extract(&exif, Tag::LensSpecification, &extract_str, meta);
        extract(&exif, Tag::LensMake, &extract_str, meta);
        extract(&exif, Tag::LensModel, &extract_str, meta);
        extract(&exif, Tag::LensSerialNumber, &extract_str, meta);
        extract(&exif, Tag::CompositeImage, &extract_str, meta);
        extract(&exif, Tag::SourceImageNumberOfCompositeImage, &extract_str, meta);
        extract(&exif, Tag::SourceExposureTimesOfCompositeImage, &extract_str, meta);
        extract(&exif, Tag::DateTimeOriginal, &extract_str, meta);
        extract(&exif, Tag::DateTimeDigitized, &extract_str, meta);    
        
        // u64
        extract(&exif, Tag::PixelYDimension, &extract_u64, meta);
        extract(&exif, Tag::PixelXDimension, &extract_u64, meta);
        extract(&exif, Tag::XResolution, &extract_u64, meta);
        extract(&exif, Tag::YResolution, &extract_u64, meta);
        
        // decimal
        extract(&exif, Tag::ExposureTime, &extract_f64, meta);
        extract(&exif, Tag::FNumber, &extract_f64, meta);
        extract(&exif, Tag::CompressedBitsPerPixel, &extract_f64, meta);
        extract(&exif, Tag::ShutterSpeedValue, &extract_str, meta);
        extract(&exif, Tag::ApertureValue, &extract_f64, meta);
        extract(&exif, Tag::ExposureBiasValue, &extract_str, meta);
        extract(&exif, Tag::MaxApertureValue, &extract_f64, meta);
        extract(&exif, Tag::FocalLength, &extract_f64, meta);
        extract(&exif, Tag::FlashEnergy, &extract_f64, meta);
        extract(&exif, Tag::FocalPlaneXResolution, &extract_f64, meta);
        extract(&exif, Tag::FocalPlaneYResolution, &extract_f64, meta);
        extract(&exif, Tag::ExposureIndex, &extract_f64, meta);
        extract(&exif, Tag::Gamma, &extract_f64, meta);

        // Interop Details
        extract(&exif, Tag::InteroperabilityIndex, &extract_str, meta);
        extract(&exif, Tag::InteroperabilityVersion, &extract_str, meta);
        extract(&exif, Tag::RelatedImageFileFormat, &extract_str, meta);
        extract(&exif, Tag::RelatedImageWidth, &extract_str, meta);
        extract(&exif, Tag::RelatedImageLength, &extract_str, meta);

        // GPS Details (String Only)
        extract(&exif, Tag::GPSVersionID, &extract_str, meta);
        extract(&exif, Tag::GPSLatitudeRef, &extract_str, meta);
        extract(&exif, Tag::GPSLatitude, &extract_str, meta);
        extract(&exif, Tag::GPSLongitudeRef, &extract_str, meta);
        extract(&exif, Tag::GPSLongitude, &extract_str, meta);
        extract(&exif, Tag::GPSAltitudeRef, &extract_str, meta);
        extract(&exif, Tag::GPSAltitude, &extract_str, meta);
        extract(&exif, Tag::GPSTimeStamp, &extract_str, meta);
        extract(&exif, Tag::GPSSatellites, &extract_str, meta);
        extract(&exif, Tag::GPSStatus, &extract_str, meta);
        extract(&exif, Tag::GPSMeasureMode, &extract_str, meta);
        extract(&exif, Tag::GPSDOP, &extract_str, meta);
        extract(&exif, Tag::GPSSpeedRef, &extract_str, meta);
        extract(&exif, Tag::GPSSpeed, &extract_str, meta);
        extract(&exif, Tag::GPSTrackRef, &extract_str, meta);
        extract(&exif, Tag::GPSTrack, &extract_str, meta);
        extract(&exif, Tag::GPSImgDirectionRef, &extract_str, meta);
        extract(&exif, Tag::GPSImgDirection, &extract_str, meta);
        extract(&exif, Tag::GPSMapDatum, &extract_str, meta);
        extract(&exif, Tag::GPSDestLatitudeRef, &extract_str, meta);
        extract(&exif, Tag::GPSDestLatitude, &extract_str, meta);
        extract(&exif, Tag::GPSDestLongitudeRef, &extract_str, meta);
        extract(&exif, Tag::GPSDestLongitude, &extract_str, meta);
        extract(&exif, Tag::GPSDestBearingRef, &extract_str, meta);
        extract(&exif, Tag::GPSDestDistanceRef, &extract_str, meta);
        extract(&exif, Tag::GPSDestDistance, &extract_str, meta);
        extract(&exif, Tag::GPSProcessingMethod, &extract_str, meta);
        extract(&exif, Tag::GPSAreaInformation, &extract_str, meta);
        extract(&exif, Tag::GPSDateStamp, &extract_str, meta);
        extract(&exif, Tag::GPSDifferential, &extract_str, meta);
        extract(&exif, Tag::GPSHPositioningError, &extract_str, meta);
        
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use chrono::{NaiveDateTime, Datelike};
    use crate::{Meta, FromFile, Extractor};
    use crate::exif::ExifExtractor;

    const TEST_IMAGE: &str = "../testdata/Image/test.jpg"; 

    #[test]
    fn test_parse() {
        let mut meta: Meta = Meta::new();
        let extractor: ExifExtractor = ExifExtractor::file(TEST_IMAGE);

        match extractor.extract(&mut meta) {
            Ok(_) => {
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
                    .map(|x| println!("{:#?}", x));

                match meta.find("DateTimeOriginal").first() {
                    Some(dto) => {
                        println!("{:#?}", dto);
                        let dt: NaiveDateTime = NaiveDateTime::parse_from_str(&String::from(dto.value.clone()), "%Y-%m-%d %H:%M:%S").unwrap();
                        assert_eq!(2023, dt.year());
                        assert_eq!(10, dt.month());
                    }, 
                    None => {
                        panic!("should have found a value!");
                    }
                }

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
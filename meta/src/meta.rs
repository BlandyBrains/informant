use std::hash::{Hash, Hasher};
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MetaClass {
    Unknown,
    Audio,
    Image,
    Video,
    Document
}
impl Default for MetaClass {
    fn default() -> Self {
        MetaClass::Unknown
    }
}
impl From<MetaClass> for String {
    fn from(value: MetaClass) -> Self {
        String::from(match value {
            MetaClass::Unknown => "Unknown",
            MetaClass::Audio => "Audio",
            MetaClass::Image => "Image",
            MetaClass::Video => "Video",
            MetaClass::Document => "Document",
        })
    }
}

impl Hash for MetaClass {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl Eq for MetaClass { }

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum MetaFormat {
    Image,
    Video,
    Audio
}
impl From<MetaFormat> for String {
    fn from(value: MetaFormat) -> Self {
        String::from(match value {
            MetaFormat::Image => "Image",
            MetaFormat::Video => "Video",
            MetaFormat::Audio => "Audio",
        })
    }
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug, )]
pub enum MetaSource {
    Basic,
    Exif,
    Matroska,
    MP4,
    ID3,
    Ape,
    Heic,
    Hash,
}
impl From<MetaSource> for String {
    fn from(value: MetaSource) -> Self {
        String::from(match value {
            MetaSource::Basic => "Basic",
            MetaSource::Hash => "Hash",
            
            // Image Meta 
            MetaSource::Exif => "Exif",
            MetaSource::Heic => "Heic",

            // Video Meta
            MetaSource::Matroska => "Matroska",
            MetaSource::MP4 => "MP4",

            // Audio Meta
            MetaSource::ID3 => "ID3",
            MetaSource::Ape => "Ape",
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaAttribute {
    pub format: MetaFormat,
    pub source: MetaSource,
    pub tag: String,
    pub value: MetaType,
}

impl Hash for MetaAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let source: String = String::from(self.source.clone());
        source.hash(state);
        self.tag.hash(state);
        match &self.value{
            MetaType::String(x) => x.value.to_owned().hash(state),
            MetaType::Rational(x) => f64::from(x.value).to_string().hash(state),
            MetaType::Int64(x) => i64::from(x.value).to_string().hash(state),
            MetaType::UInt64(x) => u64::from(x.value).to_string().hash(state),
        }
    }
}

/// TODO - Untag the MetaType and MetaValue enums
/// This will clean up the value from this
/// {
///     "format": "Image",
///     "source": "Basic",
///     "tag": "width",
///     "value": {
///         "UInt64": {
///             "value": 2048
///         }
///     }
/// }
/// 
/// to this
/// {
///     "format": "Image",
///     "source": "Basic",
///     "tag": "width",
///     "value": 2048
/// }
/// 
/// However, we'll need to make an adjustment to how we handle i64 and u64 values.
/// The deserializer will not know how to map these values. Also, this refactor
/// would require migrating all of the data at once. For now, we'll do nothing and
/// this will be something we keep in mind for the future when we utilize the metadata.
/// 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MetaType {
    String(MetaValue<String>),
    Rational(MetaValue<f64>),
    Int64(MetaValue<i64>),
    UInt64(MetaValue<u64>),
}

impl From<MetaValue<String>> for MetaType {
    fn from(value: MetaValue<String>) -> Self {
        return Self::String(value);
    }
}

impl From<MetaValue<f64>> for MetaType {
    fn from(value: MetaValue<f64>) -> Self {
        return Self::Rational(value);
    }
}

impl From<MetaValue<u64>> for MetaType {
    fn from(value: MetaValue<u64>) -> Self {
        return Self::UInt64(value);
    }
}

impl From<MetaValue<i64>> for MetaType {
    fn from(value: MetaValue<i64>) -> Self {
        return Self::Int64(value);
    }
}

impl From<MetaType> for String {
    fn from(t: MetaType) -> Self {
        let result = match t {
            MetaType::String(x) => x.value,
            _ => "".to_owned()
        };
        return result;
    }
}

impl From<MetaType> for i64 {
    fn from(t: MetaType) -> Self {
        let result = match t {
            MetaType::Int64(x) => x.value,
            MetaType::UInt64(x) => x.value as i64,
            _ => 0
        };
        return result;
    }
}

impl From<MetaType> for u32 {
    fn from(t: MetaType) -> Self {
        let result = match t {
            MetaType::Int64(x) => x.value as u32,
            MetaType::UInt64(x) => x.value as u32,
            _ => 0
        };
        return result;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaValue<T> {
    pub value: T
}

impl From<String> for MetaValue<String> {
    fn from(value: String) -> Self {
        let re: Regex = Regex::new(r"([\w\d\-])+.*([\w\d\.])|[\w\d]{1}").unwrap();
        
        let mut v: &str = value.as_str();
        v = v.split(",").collect::<Vec<&str>>().first().unwrap();
        
        match re.find(v) {
            Some(m) => {
                return Self{value: m.as_str().to_owned()};
            },
            None => {
                println!("failed meta regex {:#?}", v);
                return Self{value: v.to_owned()};
            }
        }
    }
}

impl From<f64> for MetaValue<f64> {
    fn from(value: f64) -> Self {
        return Self{value};
    }
}

impl From<i64> for MetaValue<i64> {
    fn from(value: i64) -> Self {
        return Self{value};
    }
}

impl From<u64> for MetaValue<u64> {
    fn from(value: u64) -> Self {
        return Self{value};
    }
}

#[cfg(test)]
mod test {
    use crate::meta::MetaValue;

    #[test]
    fn test_regex(){
        let mut samples: Vec<(&str, &str)> = Vec::new();
        
        // subject, expected
        samples.push(("\"MAKER\"", "MAKER"));
        samples.push(("\\Maker\\", "Maker"));
        samples.push(("!@#$123FIRST123", "123FIRST123"));
        samples.push(("Asahi Optical Co.", "Asahi Optical Co."));
        samples.push(("DMC-LC33", "DMC-LC33"));
        samples.push(("Canon EOS-1D Mark II", "Canon EOS-1D Mark II"));
        samples.push(("            SOME PADDED VALUE           ", "SOME PADDED VALUE"));
        samples.push(("\"-5\"", "-5"));
        
        for s in samples {
            let actual: MetaValue<String> = MetaValue::from(s.0.to_owned());
            assert_eq!(s.1.to_owned(), actual.value);
        }
    }
}
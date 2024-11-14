use std::hash::{Hash, Hasher};
use regex::Regex;
use serde::{Serialize, Deserialize};

use crate::{get_extractors, Extractors, MetaError};

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
impl From<String> for MetaClass {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Audio" => MetaClass::Audio,
            "Image" => MetaClass::Image,
            "Video" => MetaClass::Video,
            "Document" => MetaClass::Document,
            _ => MetaClass::Unknown,
        }
    }
}
impl Hash for MetaClass {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
impl From<MetaType> for MetaClass {
    fn from(value: MetaType) -> Self {
        let class: String = String::from(value);
        return String::into(class);
    }
}

impl Eq for MetaClass { }

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

impl Default for MetaSource {
    fn default() -> Self {
        MetaSource::Basic
    }
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
    pub source: MetaSource,
    pub tag: String,
    pub value: MetaType,
}

impl Default for MetaAttribute {
    fn default() -> Self {
        Self { 
            source: Default::default(), 
            tag: Default::default(), 
            value: Default::default() 
        }
    }
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Meta(pub Vec<MetaAttribute>);
impl Meta {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn capture(file: &str) -> Result<Self, MetaError> {
        let extractors: Extractors = get_extractors(file)?;
        Ok(extractors.extract()?)
    }

    pub fn add(&mut self, attribute: MetaAttribute){
        self.0.push(attribute)
    }

    pub fn update(&mut self, attribute: MetaAttribute) {
        match self.0.iter_mut().find(|attr| attr.source == attribute.source &&
            attr.tag == attribute.tag){
                Some(a) => {
                    a.value = attribute.value;
                },
                _ => ()
            }
    }

    pub fn find(&self, tag: &str) -> Vec<MetaAttribute> {
        let values: Vec<MetaAttribute> = self.0.clone();

        let matches: Vec<MetaAttribute> = values
            .into_iter()
            .filter(|x| x.tag == tag)
            .collect();

        return matches;
    }

    pub fn find_one(&self, source: MetaSource, tag: &str) -> Result<MetaAttribute, MetaError> {
        let values: Vec<MetaAttribute> = self.0.clone();

        let matches: Vec<MetaAttribute> = values
            .into_iter()
            .filter(|x|
                x.source == source && 
                x.tag == tag)
            .collect();

        if matches.len() == 0 {
            return Err(MetaError::from(format!("could not find meta for tag: {}", tag)));
        }
        
        return Ok(matches.first().cloned().unwrap());
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Iterator for Meta {
    type Item = MetaAttribute;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl From<String> for Meta {
    fn from(_value: String) -> Self {
        todo!()
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

impl Default for MetaType {
    fn default() -> Self {
        MetaType::String(MetaValue{ value: "".to_owned() })
    }
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
            MetaType::Int64(x) => x.value.to_string(),
            MetaType::UInt64(x) => x.value.to_string(),
            MetaType::Rational(x) => {
                if x.value.is_nan(){
                    return "NaN".to_owned();
                }
                x.value.to_string()
            }
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
        // General regex - for?
        let re: Regex = Regex::new(r#"([\w\d\-])+.*([\w\d\.\)])|[\w\d]{1}"#).unwrap();
        
        let mut v: &str = value.as_str();
        v = v.split(",").collect::<Vec<&str>>().first().unwrap();

        // replace null character \0
        // replace \"
        let v2: String = v
            .replace("\0", "")
            .replace("\"", "")
            .replace("`", "")
            .trim()
            .to_owned();
        
        match re.find(&v2) {
            Some(m) => {
                return Self{value: m.as_str().to_owned()}; 
            },
            None => {
                if v2.len() > 0 {
                    // println!("failed meta regex {} {}", v, v2);
                }
                
                return Self{value: "".to_owned()};
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

impl From<MetaType> for u64 {
    fn from(value: MetaType) -> Self {
        match value {
            crate::meta::MetaType::UInt64(x) => x.value,
            _ => 0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::meta::MetaValue;

    use super::Meta;

    #[test]
    fn test_meta() {
        let meta: Meta = Meta::new();
        assert_eq!(meta.0.len(), 0);
    }

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
        samples.push(("\"\"", ""));
        
        for s in samples {
            let actual: MetaValue<String> = MetaValue::from(s.0.to_owned());
            assert_eq!(s.1.to_owned(), actual.value);
        }
    }
}
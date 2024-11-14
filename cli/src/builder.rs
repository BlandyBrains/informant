use std::{collections::HashSet, fs, path::PathBuf, sync::Arc};

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use meta::{Extractors, Meta, MetaClass, MetaSource};
use storage::{Asset, Database, Partition};
use uuid::Uuid;

pub (crate) type BuilderError = Box<dyn std::error::Error>;
pub (crate) type Builder = AssetBuilder<NoValue, NoValue>;


pub struct NoValue;
impl Default for NoValue {
    fn default() -> Self {
        Self {  }
    }
}

struct XmpMeta(rexiv2::Metadata);

#[derive(Debug)]
pub struct AssetParts {
    pub extension: String,
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i32>,
    pub month: Option<u32>
}

impl AssetParts {

    pub fn is_device_mapped(&self) -> bool {
        return self.make.is_some() && self.model.is_some();
    }

    pub fn to_path(&self, prefix: Option<&str>, partition: Option<Partition>) -> Result<String, BuilderError> {
        let mut path: PathBuf = PathBuf::from(prefix.map_or("", |x| x));

        let is_device_mapped: bool = self.is_device_mapped();
        if is_device_mapped {
            path.push("Devices");
            path.push(self.make.clone().unwrap());
            path.push(self.model.clone().unwrap());
        }

        match self.year {
            Some(year) => {
                match self.month {
                    Some(month) => {
                        path.push(format!("{:02}", year));
                        path.push(format!("{:02}", month));
                    },
                    None => ()
                }
            },
            None => ()
        }
        
        if !is_device_mapped{
            path.push("Format");
            path.push(format!("{:05}", partition.unwrap().partition));
        }

        path.push(self.extension.to_lowercase());

        Ok(path.display().to_string())
    }

    pub fn to_tags(&self) -> Vec<String> {
        return vec![
            self.extension.to_lowercase(),
            self.make.clone().map_or("".to_string(), |m| m),
            self.model.clone().map_or("".to_string(), |m| m),
            self.year.clone().map_or("".to_string(), |y| y.to_string()),
            format!("{:02}", self.month.clone().map_or("".to_string(), |m| m.to_string())),
        ]
    }
}

pub struct AssetBuilder<A, P>{
    file: String,
    extractors: Arc<Extractors>,
    meta: Meta,
    xmp: Arc<XmpMeta>,
    parts: P,
    pub asset: A
}

impl<A, P> AssetBuilder<A, P> {
    pub fn open(file: &str) -> Result<AssetBuilder<NoValue, NoValue>, BuilderError> {
        if !fs::metadata(&file).is_ok() {
            return Err("file not found".into());
        }

        // 1. Is supported type?
        let meta: rexiv2::Metadata = rexiv2::Metadata::new_from_path(&file)?;
        // if !meta.supports_exif() || !meta.supports_iptc() || !meta.supports_xmp() {
        //     return Err("not supported (yet)".into());
        // }

        // Get Extractors for Assets
        let extractors: Extractors = meta::get_extractors(&file)?;

        Ok(AssetBuilder{ 
            file: file.to_string(),
            extractors: Arc::new(extractors),
            meta: Meta::new(),
            xmp: Arc::new(XmpMeta(meta)),
            parts: NoValue,
            asset: NoValue
        })
    }

    pub fn get_hash(&self) -> Result<String, BuilderError> {
        let mut meta: Meta = Meta::new();
        
        let extractor = self.extractors.find("HASH")?;
        extractor.extract(&mut meta).unwrap();

        let hash: String = match meta.find_one(MetaSource::Hash, "hash") {
            Ok(a) => String::from(a.value),
            Err(e) => {
                return Err(format!("error extracting hash {}", e).into());
            }
        };
        return Ok(hash);
    }

    pub fn scrub(&self) -> Result<(), BuilderError> {
        self.xmp.0.clear();
        self.xmp.0.erase_thumbnail();
        self.xmp.0.delete_gps_info();
        self.xmp.0.save_to_file(&self.file)?;
        Ok(())
    }

    pub fn build(&mut self, db: &Database) -> Result<AssetBuilder<Asset, AssetParts>, BuilderError> {
        println!("Extracting meta from {}...", &self.file);
        let meta: Meta = self.extractors.extract()?;
        
        self.meta = meta.clone();

        let hash: String = String::from(self.meta.find_one(MetaSource::Hash, "hash").unwrap().value);
        let extension: String = String::from(self.meta.find_one( MetaSource::Basic,"extension").unwrap().value);
        let size: i64 = i64::from(self.meta.find_one( MetaSource::Basic,"size").unwrap().value);
        let meta_class: MetaClass = MetaClass::from(self.meta.find_one(MetaSource::Basic,"class").unwrap().value);

        let parts: AssetParts = self.path_parts()?;

        let mut asset: Asset = Asset{
            id: 0,
            parent_id: None,
            name: format!("{}.{}", Uuid::new_v4(), extension.to_lowercase()),
            extension,
            class: meta_class,
            hash,
            size_in_bytes: size,
            status: storage::Status::Created,
            path: None,
            partition: None,
        };

        let parent_asset: Option<Asset> = Asset::find_by_hash(&db, &asset.hash)?;
        asset.parent_id = parent_asset.map_or(None, |x| Some(x.id));

        let prefix: String = asset.parent_id.map_or("Primary".to_string(), |_| "Secondary".to_string());

        match parts.is_device_mapped() {
            true => {
                asset.path = Some(parts.to_path(Some(&prefix), None)?);
            }, 
            false => {
                // we don't have device data, but we could have year+month.
                // in this case, we do not want a partition.
                if parts.month.is_some() && parts.year.is_some() {
                    asset.path = Some(parts.to_path(Some(&prefix), None)?);
                }
                else {
                    // partitions should only apply to /Format/PARTITION/file.name
                    // get partition for the asset
                    let partition: Partition = Partition::get_partition(&db, &asset)?;
                    asset.partition = Some(partition.partition);
                    asset.path = Some(parts.to_path(Some(&prefix), Some(partition))?);
                }
            }
        }
        
        asset.create(&db)?;

        Ok(AssetBuilder{ 
            file: self.file.to_string(),
            extractors: self.extractors.clone(),
            meta,
            xmp: self.xmp.clone(),
            parts,
            asset
        })
    }

    fn path_parts(&self) -> Result<AssetParts, BuilderError> {
        let make: String = String::from(self.meta.find_one( MetaSource::Exif,"Make").unwrap_or_default().value);
        let model: String = String::from(self.meta.find_one( MetaSource::Exif,"Model").unwrap_or_default().value);

        let datetime_str: String = String::from(self.meta.find_one( MetaSource::Exif,"DateTimeOriginal").unwrap_or_default().value);

        if datetime_str.len() > 0 {
            println!("DATE {}", datetime_str);
            let naive_datetime: NaiveDateTime = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                .expect(&format!("Failed to parse date time {:?}", datetime_str));
    
            let date: NaiveDate = naive_datetime.date();
    
            return Ok(AssetParts{
                extension: String::from(self.meta.find_one( MetaSource::Basic,"extension").unwrap_or_default().value),
                make: (!make.is_empty()).then(|| make),
                model: (!model.is_empty()).then(|| model),
                year: Some(date.year()),
                month: Some(date.month0() + 1)
            });
        }

        Ok(AssetParts{
            extension: String::from(self.meta.find_one( MetaSource::Basic,"extension").unwrap_or_default().value),
            make: (!make.is_empty()).then(|| make),
            model: (!model.is_empty()).then(|| model),
            year: None,
            month: None
        })

    }
}

impl AssetBuilder<Asset, AssetParts> {
    pub fn normalize_tags(&self) -> Result<(), BuilderError>{
        let mut tags: Vec<String> = Vec::new();

        // convert parts to tags
        let parts: &AssetParts = &self.parts;
        tags.append(&mut parts.to_tags());

        let xmp_keyword_tag: &str = "Xmp.dc.subject";
        let exif_keyword_tag: &str = "Exif.Image.ImageDescription";
        let iptc_keyword_tag: &str = "Iptc.Application2.Keywords";

        // extract all other tags (exif, iptc, xmp)
        let mut exif_tags: Vec<String> = self.xmp.0.get_tag_multiple_strings(exif_keyword_tag)?;
        let mut xmp_tags: Vec<String> = self.xmp.0.get_tag_multiple_strings(xmp_keyword_tag)?;
        let mut iptc_tags: Vec<String> = self.xmp.0.get_tag_multiple_strings(iptc_keyword_tag)?;

        tags.append(&mut exif_tags);
        tags.append(&mut xmp_tags);
        tags.append(&mut iptc_tags);

        // deduped list
        let mut final_tags: Vec<String> = tags.into_iter().collect::<HashSet<_>>().into_iter().collect();

        // additional cleanup (remove empty strings)
        final_tags = final_tags.into_iter().filter(|x| x.trim().len() > 0).collect();

        let tags: Vec<&str> = final_tags.iter().map(|x| x.as_str()).collect();
        self.xmp.0.set_tag_multiple_strings(&exif_keyword_tag, tags.as_slice())?;
        self.xmp.0.set_tag_multiple_strings(&iptc_keyword_tag, tags.as_slice())?;
        self.xmp.0.set_tag_multiple_strings(&xmp_keyword_tag, tags.as_slice())?;
        
        self.xmp.0.save_to_file(&self.file)?;
        Ok(())
    }
}
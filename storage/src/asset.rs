use std::path::PathBuf;

use meta::MetaClass;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::{Database, StorageError};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    /// The asset record is created but
    /// the location is not guaranteed. 
    /// Asset may not have been completely written.
    Created,

    /// The asset as been completely written to the location.
    Available,

    Hidden,

    /// The asset is labeled for deletion.
    Discarded,

    /// The asset has been deleted and the location does not exist.
    Deleted,

    Unknown
}

impl From<Status> for String {
    fn from(value: Status) -> Self {
        String::from(match value {
            Status::Created => "Created",
            Status::Available => "Available",
            Status::Hidden => "Hidden",
            Status::Discarded => "Discarded",
            Status::Deleted => "Deleted",
            Status::Unknown => "Unknown"
        })
    }
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "Created" => Status::Created,
            "Available" => Status::Available,
            "Hidden" => Status::Hidden,
            "Discarded" => Status::Discarded,
            "Deleted" => Status::Deleted,
            _ => Status::Unknown
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub class: MetaClass,
    pub extension: String,
    pub hash: String,
    pub size_in_bytes: i64,
    pub status: Status,
    pub path: Option<String>,
    pub partition: Option<usize>
}

impl Asset {
    pub fn find_by_hash(db: &Database, hash: &str) -> Result<Option<Asset>, StorageError> {
        let mut stmt = db.conn.prepare(r#"
            SELECT 
                id, 
                name,
                class,
                extension,
                hash,
                size,
                status,
                path,
                partition
            FROM asset where hash = ?1
            "#)?;

        let asset: Option<Asset> = stmt.query_row([hash.to_string()], |row| {
            let class: String = row.get(2)?;
            let status: String = row.get(6)?;
            
            Ok(Asset {
                id: row.get(0)?,
                parent_id: None,
                name: row.get(1)?,
                class: class.into(),
                extension: row.get(3)?,
                hash: row.get(4)?,
                size_in_bytes: row.get(5)?,
                status: status.into(),
                path: row.get(7)?,
                partition: row.get(8)?
            })
        }).optional()?;

        Ok(asset)
    }

    pub fn create(&mut self, db: &Database) -> Result<(), StorageError> {
        let mut stmt = db.conn.prepare(r#"
            INSERT INTO asset (parent_id, name, class, extension, hash, size, status, path, partition)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#)?;

        let status: String = self.status.clone().into();
        let class: String = self.class.clone().into();

        stmt.execute(params![
            self.parent_id,
            self.name,
            class,
            self.extension,
            self.hash,
            self.size_in_bytes,
            status,
            self.path,
            self.partition
        ])?;

        self.id = db.conn.last_insert_rowid();
        
        Ok(())
    }

    pub fn available(&mut self, db: &Database) -> Result<(), StorageError> {
        let mut stmt = db.conn.prepare(r#"
            UPDATE asset
            SET
                status=?2
            WHERE 
                id=?1"#)?;

        self.status = Status::Available;
        let status: String = self.status.clone().into();
        
        stmt.execute(params![
            self.id,
            status
        ])?;

        Ok(())
    }

    pub fn full_path(&self) -> String {
        let mut path: PathBuf = PathBuf::from(&self.path.clone().unwrap());
        path.push(&self.name);
        return path.display().to_string();
    }
}
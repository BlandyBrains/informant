use rusqlite::{params, OptionalExtension};

use crate::{Asset, Database, StorageError};

#[derive(Debug)]
pub struct Partition {
    pub is_primary: bool,
    pub extension: String,
    pub partition: usize,
    pub size: usize
}

impl Partition {

    fn get_max_partition(db: &Database, extension: &str, is_primary: bool) -> Result<usize, StorageError> {
        let mut primary_statement = db.conn.prepare(r#"
            SELECT
                COALESCE(max(partition), 0)
            FROM asset 
            WHERE extension = ?1 and parent_id is null
        "#)?;

        let mut secondary_statement = db.conn.prepare(r#"
            SELECT
                COALESCE(max(partition), 0)
            FROM asset 
            WHERE extension = ?1 and parent_id is not null
        "#)?;


        return match is_primary {
            true => {
                Ok(primary_statement.query_row(params![&extension], |row| {
                    let max: usize = row.get(0).unwrap_or(1);
                    Ok(max)
                })?)
            },
            false => {
                Ok(secondary_statement.query_row(params![&extension], |row| {
                    let max: usize = row.get(0).unwrap_or(1);
                    Ok(max)
                })?)
            }
        };
    }

    pub fn get_partition(db: &Database, asset: &Asset) -> Result<Partition, StorageError> {
        let mut primary_statement = db.conn.prepare(r#"
            SELECT
                extension,
                COALESCE(PARTITION, 0),
                count() as count
            FROM asset 
            WHERE extension = ?1 and parent_id is null
            GROUP BY 
                extension,
                partition
            ORDER BY
                count asc
            LIMIT 1
        "#)?;

        let mut secondary_statement = db.conn.prepare(r#"
            SELECT
                extension,
                COALESCE(PARTITION, 0),
                count() as count
            FROM asset 
            WHERE extension = ?1 and parent_id is not null
            GROUP BY 
                extension,
                partition
            ORDER BY
                count asc
            LIMIT 1;
        "#)?;
        
        let partition: Option<Partition> = match asset.parent_id {
            Some(_) => {
                secondary_statement.query_row(params![&asset.extension], |row| {
                    Ok(Partition{
                        is_primary: false,
                        extension: row.get(0)?,
                        partition: row.get(1)?,
                        size: row.get(2)?
                    })
                }).optional()?
            },
            None => {
                primary_statement.query_row(params![&asset.extension], |row| {
                    Ok(Partition{
                        is_primary: true,
                        extension: row.get(0)?,
                        partition: row.get(1)?,
                        size: row.get(2)?
                    })
                }).optional()?
            }
        };

        return match partition {
            Some(mut p) => {
                if p.size >= 100 {
                    p.size = 0;
                    p.partition = Self::get_max_partition(&db, &asset.extension, asset.parent_id.is_none())? + 1;
                    println!("New Partition: is_primary={}, extension={}, partition={}", p.is_primary, p.extension, p.partition);
                }
                Ok(p)
            },
            None => {
                Ok(Partition{
                    is_primary: asset.parent_id.is_none(),
                    extension: asset.extension.clone(),
                    partition: 0,
                    size: 0
                })
            }
        };
    }
}


#[cfg(test)]
mod test {
    use meta::MetaClass;
    use rand::{rngs::ThreadRng, Rng};
    use crate::{partition::Partition, Asset, Database, Status};

    fn build_asset(parent_id: Option<i64>, partition: Option<usize>) -> Asset {
        Asset { 
            id: 0, 
            parent_id, 
            name: "fake".to_string(), 
            class: MetaClass::Image,
            extension: "jpeg".to_string(), 
            hash: uuid::Uuid::new_v4().to_string(), 
            size_in_bytes: 0, 
            status: Status::Available, 
            path: None, 
            partition: partition,
        }
    }
    
    #[test]
    fn provision_db(){
        let db: Database = Database::open("./../db_test_partition.db3").unwrap();
        db.migrate().unwrap();

        // insert a bunch of random assets
        let mut rng: ThreadRng = rand::thread_rng();

        for _ in 0..100 {
            let partition: usize = rng.gen_range(1..=5);

            let mut asset: Asset = build_asset(None, Some(partition));
            asset.create(&db).unwrap();
        }

        print!("MAX PARTITION {}", Partition::get_max_partition(&db, "random", true).unwrap());
    }

    #[test]
    fn get_partition_fake() {
        let db: Database = Database::open("./../db_test_partition.db3").unwrap();
        let mut asset: Asset = build_asset(None, None);
        asset.extension = "FAKE".to_owned();

        let partition = Partition::get_partition(&db, &asset).unwrap();
        println!("Partition: {:#?}", partition);
    }

    #[test]
    fn get_partition_jpeg() {
        let db: Database = Database::open("./../dad_backup_v1.db3").unwrap();
        let mut asset: Asset = build_asset(Some(1), None);
        asset.extension = "jpeg".to_owned();

        let partition = Partition::get_partition(&db, &asset).unwrap();
        println!("Partition: {:#?}", partition);
    }

    #[test]
    fn get_max_partition() {
        let db: Database = Database::open("./../dad_backup_v1.db3").unwrap();
        let mut asset: Asset = build_asset(None, None);
        asset.extension = "some_random_extension".to_owned();
    
        let max_partition = Partition::get_max_partition(&db, &asset.extension, true).unwrap();
        // let partition = Partition::get_partition(&db, &asset).unwrap();
        println!("Max Partition: {:#?}", max_partition);
    }
}

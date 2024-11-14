mod asset;
mod partition;

use log::info;
use rusqlite::Connection;
pub use asset::{Asset, Status};
pub use partition::Partition;

pub (crate) type StorageError = Box<dyn std::error::Error>;


pub struct Database {
    pub conn: Connection
}

impl Database {
    pub fn open(path: &str) -> Result<Database, StorageError> {
        env_logger::init();

        let mut conn: Connection = Connection::open(path)?;
        conn.trace(Some(|sql| info!("Executing SQL: {}", sql)));

        Ok(Self {
            conn
        })
    }

    pub fn migrate(&self) -> Result<(), StorageError> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS 
                asset(
	                id INTEGER primary key autoincrement,
                    parent_id INTEGER NULL,
                    name TEXT NOT NULL,
	                class TEXT NOT NULL,
	                extension TEXT NOT NULL,
	                hash TEXT NOT NULL,
	                size BIGINT NOT NULL,
                    status TEXT NOT NULL,
                    path TEXT NULL,
                    partition INTEGER NULL,
	                created_at timestamp DEFAULT CURRENT_TIMESTAMP NULL
                )",
            (),
        )?;
        
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use rusqlite::Connection;

    #[test]
    fn provision_db() {
        let conn: Connection = Connection::open("./../db_test.db3").unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS 
                asset(
	                id INTEGER primary key autoincrement,
	                class TEXT NOT NULL,
	                extension TEXT NOT NULL,
	                hash TEXT NOT NUL LCHECK (hash != ''),
	                size BIGINT NOT NULL,
                    status TEXT NOT NULL,
                    path TEXT NOT NULL,
	                created_at timestamp DEFAULT CURRENT_TIMESTAMP NULL
                )",
            (),
        ).unwrap();

        println!("OKAY")
    }


}

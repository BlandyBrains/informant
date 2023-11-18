use std::{path::{Path, PathBuf}, io};

#[derive(Clone)]
pub struct Store {
    root: String,
}
impl Store {
    pub fn new(root: String) -> Self {
        Self{ root }
    }

    pub fn root(&self) -> PathBuf {
        return Path::new(&self.root).to_path_buf();
    }
    
    pub fn join(&self, suffix: &str) -> PathBuf {
        return self.root().join(suffix);
    }

    pub fn exists(&self, location: &str) -> bool {
        return self.join(location).exists();
    }

    pub fn delete(&self, location: &str) -> Result<(), io::Error> {
        return Ok(std::fs::remove_file(self.join(location))?);
    }
    
    pub fn rename(&self, from: &str, to: &str) -> Result<(), io::Error> {
        let f: PathBuf = self.join(from);
        let t: PathBuf = self.join(to);

        if !self.exists(f.parent().unwrap().to_str().unwrap()) {
            dbg!("missing destination, creating {:#?}", f.parent());
            std::fs::create_dir_all(f.parent().unwrap())?;
        }

        if !self.exists(t.parent().unwrap().to_str().unwrap()) {
            dbg!("missing destination, creating {:#?}", t.parent());
            std::fs::create_dir_all(t.parent().unwrap())?;
        }
        
        dbg!("renaming {:#?} to {:#?}", &f, &t);
        Ok(std::fs::rename(f, t)?)
    }

    pub fn relocate(&self, from: &str, to: &str) -> Result<(), io::Error> {
        let f: PathBuf = Path::new(from).to_path_buf();
        let t: PathBuf = self.join(to);

        if !self.exists(f.parent().unwrap().to_str().unwrap()) {
            dbg!("missing destination, creating {:#?}", f.parent());
            std::fs::create_dir_all(f.parent().unwrap())?;
        }

        if !self.exists(t.parent().unwrap().to_str().unwrap()) {
            dbg!("missing destination, creating {:#?}", t.parent());
            std::fs::create_dir_all(t.parent().unwrap())?;
        }
        
        dbg!("renaming {:#?} to {:#?}", &f, &t);
        Ok(std::fs::rename(f, t)?)
    }

}

#[cfg(test)]
mod test {
    use uuid::Uuid;
    use super::Store;

    fn root() -> String {
        return "../testdata/".to_owned();
    }

    fn create_file() -> String {
        let name = format!("{0}{1}", root(), Uuid::new_v4().to_string());
        std::fs::File::create(&name).unwrap();
        return name;
    }

    #[test]
    fn test_exists() {
        let location: String = create_file();
        let store: Store = Store::new(root());

        assert_eq!(false, store.exists(&location[..10]));
    }

    #[test]
    fn test_raw_rename() {
        let test_file: String = create_file();
        let new_location: String = format!("{0}{1}", root(), Uuid::new_v4().to_string());

        let store: Store = Store::new(root());
        store.rename(&test_file, &new_location).unwrap();
    }    
}
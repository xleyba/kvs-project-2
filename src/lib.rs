use std::collections::HashMap;
use std::path::Path;
use error::Result;

pub mod error;

#[derive(Default)]
pub struct KvStore {
 map: HashMap<String, String>,
}

impl KvStore {
    
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        panic!()
    }

}
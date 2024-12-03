use lru::LruCache;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use crate::traits::Cache;

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistentCache<K, V> {
    cache: LruCache<K, V>,
}

impl<K: Eq + std::hash::Hash + Serialize + for<'de> Deserialize<'de>, V: Serialize + for<'de> Deserialize<'de>> Cache<K, V> for PersistentCache<K, V> {
    fn new(capacity: usize) -> Self {
        PersistentCache {
            cache: LruCache::new(capacity),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.cache.put(key, value);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    fn persist(&self, file_path: &str) -> io::Result<()> {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
        let data = serde_json::to_string(&self.cache)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        self.cache = serde_json::from_str(&data)?;
        Ok(())
    }
}
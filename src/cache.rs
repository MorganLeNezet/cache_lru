use lru::LruCache;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::num::NonZeroUsize;
use crate::traits::Cache;

#[derive(Serialize, Deserialize, Clone)]
struct SerializableLruCache<K: Eq + Hash + Clone, V: Clone> {
    map: HashMap<K, V>,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V: Clone> From<LruCache<K, V>> for SerializableLruCache<K, V> {
    fn from(mut cache: LruCache<K, V>) -> Self {
        let capacity = cache.cap().get();
        let mut map = HashMap::new();
        while let Some((k, v)) = cache.pop_lru() {
            map.insert(k, v);
        }
        SerializableLruCache { map, capacity }
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Into<LruCache<K, V>> for SerializableLruCache<K, V> {
    fn into(self) -> LruCache<K, V> {
        let capacity = NonZeroUsize::new(self.capacity).unwrap();
        let mut cache = LruCache::new(capacity);
        for (k, v) in self.map {
            cache.put(k, v);
        }
        cache
    }
}

pub struct PersistentCache<K: Eq + Hash + Clone, V: Clone> {
    cache: SerializableLruCache<K, V>,
}

impl<K: Eq + Hash + Clone + Serialize + for<'de> Deserialize<'de>, V: Clone + Serialize + for<'de> Deserialize<'de>> Cache<K, V> for PersistentCache<K, V> {
    fn new(capacity: usize) -> Self {
        PersistentCache {
            cache: SerializableLruCache {
                map: HashMap::new(),
                capacity,
            },
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let mut lru_cache: LruCache<K, V> = self.cache.clone().into();
        lru_cache.put(key, value);
        self.cache = lru_cache.into();
    }

    fn get(&self, key: &K) -> Option<V> {
            let mut lru_cache: LruCache<K, V> = self.cache.clone().into();
            lru_cache.get(key).cloned()
        }

    fn persist(&self, file_path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
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
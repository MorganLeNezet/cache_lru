use project::cache::PersistentCache;
use project::traits::Cache;

fn main() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    println!("{:?}", cache.get(&"key1".to_string()));
}
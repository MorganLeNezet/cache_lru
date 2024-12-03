use project::cache::PersistentCache;
use project::traits::Cache;

fn main() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1", "value1");
    println!("{:?}", cache.get(&"key1"));
}
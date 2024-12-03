use project::cache::PersistentCache;
use project::traits::Cache;

#[test]
fn test_insert_and_get() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1", "value1");
    assert_eq!(cache.get(&"key1"), Some(&"value1"));
}

#[test]
fn test_persist_and_load() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1", "value1");
    cache.persist("test_cache.json").unwrap();

    let mut loaded_cache = PersistentCache::new(2);
    loaded_cache.load("test_cache.json").unwrap();
    assert_eq!(loaded_cache.get(&"key1"), Some(&"value1"));
}
use project::cache::PersistentCache;
use project::traits::Cache;

#[test]
fn test_insert_and_get() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
}

#[test]
fn test_persist_and_load() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.persist("test_cache.json").unwrap();

    let mut loaded_cache = PersistentCache::new(2);
    loaded_cache.load("test_cache.json").unwrap();
    assert_eq!(loaded_cache.get(&"key1".to_string()), Some("value1".to_string()));
}
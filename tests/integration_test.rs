use project::cache::PersistentCache;
use project::traits::Cache;

#[test]
fn test_insert_and_get() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    assert_eq!(cache.get(&"key1".to_string()), Some(&mut "value1".to_string()));
}

#[test]
fn test_persist_and_load() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.persist("test_cache.json").unwrap();

    let mut loaded_cache = PersistentCache::new(2);
    loaded_cache.load("test_cache.json").unwrap();
    assert_eq!(loaded_cache.get(&"key1".to_string()), Some(&mut "value1".to_string()));
}

#[test]
fn test_eviction_lru() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    cache.insert("key3".to_string(), "value3".to_string()); // This should evict "key1"
    assert_eq!(cache.get(&"key1".to_string()), None);
    assert_eq!(cache.get(&"key2".to_string()), Some(&mut "value2".to_string()));
    assert_eq!(cache.get(&"key3".to_string()), Some(&mut "value3".to_string()));
}

#[test]
fn test_update_value() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key1".to_string(), "value1_updated".to_string());
    assert_eq!(cache.get(&"key1".to_string()), Some(&mut "value1_updated".to_string()));
}

#[test]
fn test_capacity() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    cache.insert("key3".to_string(), "value3".to_string()); // This should evict "key1"
    cache.insert("key4".to_string(), "value4".to_string()); // This should evict "key2"
    assert_eq!(cache.get(&"key1".to_string()), None);
    assert_eq!(cache.get(&"key2".to_string()), None);
    assert_eq!(cache.get(&"key3".to_string()), Some(&mut "value3".to_string()));
    assert_eq!(cache.get(&"key4".to_string()), Some(&mut "value4".to_string()));
}